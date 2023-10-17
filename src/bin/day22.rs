//use std::cell::RefCell;
//use std::cmp::min;
use std::vec::Vec;
use regex::Regex;
use advent_lib::grid::Grid;
use advent_lib::read::read_grouped_input;

#[derive(Debug, Copy, Clone)]
enum Cell {
    Void,
    Open,
    Wall,
    Wrap(i64, i64, Dir),
    WrapCorner((i64, i64, Dir),(i64, i64, Dir)),
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
    dir: Dir,
}

impl Pos {
    fn turn(&mut self, m: Move) {
        match (self.dir, m) {
            (Dir::Up, Move::TurnLeft) => { self.dir = Dir::Left; },
            (Dir::Up, Move::TurnRight) => { self.dir = Dir::Right; },
            (Dir::Down, Move::TurnLeft) => { self.dir = Dir::Right; },
            (Dir::Down, Move::TurnRight) => { self.dir = Dir::Left; },
            (Dir::Right, Move::TurnLeft) => { self.dir = Dir::Up; },
            (Dir::Right, Move::TurnRight) => { self.dir = Dir::Down; },
            (Dir::Left, Move::TurnLeft) => { self.dir = Dir::Down; },
            (Dir::Left, Move::TurnRight) => { self.dir = Dir::Up; },
            _ => {},
        };
    }
    fn try_step(&mut self, grid: &Grid<Cell>) {
        let (mut nx, mut ny) = match self.dir {
            Dir::Up => (self.x, self.y-1),
            Dir::Down => (self.x, self.y+1),
            Dir::Right => (self.x+1, self.y),
            Dir::Left => (self.x-1, self.y),
        };
        loop {
            match grid.get(nx, ny) {
                Cell::Open => { self.x = nx; self.y = ny; return; },
                Cell::Wall => { return; },
                Cell::Wrap(wx, wy, wd) => {
                    match grid.get(wx, wy) {
                        Cell::Open => { self.x = wx; self.y = wy; self.dir = wd; },
                        _ => {},
                    };
                    return;
                },
                Cell::WrapCorner(w1, w2) => {
                    let (wx, wy, wd) = if w1.0 == self.x && w1.1 == self.y { w2 } else { w1 };
                    match grid.get(wx, wy) {
                        Cell::Open => { self.x = wx; self.y = wy; self.dir = wd; },
                        _ => {},
                    };
                    return;
                },
                _ => {},
            };
            let search = |dx, dy| -> (i64, i64) {
                let mut sx = self.x;
                let mut sy = self.y;
                loop {
                    match grid.get(sx + dx, sy + dy) {
                        Cell::Void => { return (sx, sy); },
                        Cell::Wrap(_,_,_) => panic!(),
                        Cell::WrapCorner(_,_) => panic!(),
                        _ => { sx += dx; sy += dy; },
                    }
                }
            };
            (nx, ny) = match self.dir {
                Dir::Up => search(0, 1),
                Dir::Down => search(0, -1),
                Dir::Right => search(-1, 0),
                Dir::Left => search(1, 0),
            };
            self.x = nx;
            self.y = ny;
        }
    }
    fn apply_move(&mut self, m: &Move, grid: &Grid<Cell>) {
        match m {
            Move::TurnLeft | Move::TurnRight => { self.turn(*m); },
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    self.try_step(grid);
                }
            },
        }
    }
    fn password(&self) -> i64 {
        (self.x + 1) * 4 +
        (self.y + 1) * 1000 +
        match self.dir {
            Dir::Up => 3,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Right => 0,
        }
    }
}

/*
fn on_cube(c:Cell) -> bool {
    match c {
        Cell::Open | Cell::Wall => true,
        _ => false,
    }
}
*/

fn mkwrap(x: i64, y: i64, dir: Dir) -> Cell {
    match dir {
        Dir::Up => Cell::Wrap(x, y-1, dir),
        Dir::Down => Cell::Wrap(x, y+1, dir),
        Dir::Left => Cell::Wrap(x-1, y, dir),
        Dir::Right => Cell::Wrap(x+1, y, dir),
    }
}

fn connect_edge(grid: &mut Grid<Cell>,
    edge1: &mut dyn Iterator<Item=(i64,i64)>, dir1: Dir,
    edge2: &mut dyn Iterator<Item=(i64,i64)>, dir2: Dir
) {
    for (c1, c2) in Iterator::zip(edge1, edge2) {
        grid.set(c1.0, c1.1, mkwrap(c2.0, c2.1, dir2));
        grid.set(c2.0, c2.1, mkwrap(c1.0, c1.1, dir1));
    }
}

fn fold(grid: &mut Grid<Cell>) {
    /*
    let shorter = min(
        grid.x_bounds().end - grid.x_bounds().start,
        grid.y_bounds().end - grid.y_bounds().start) - 2;
    let cubeside = shorter / 3;
    let corners: RefCell<Vec<(i64,i64,Dir,Dir)>> = RefCell::new(Vec::new());
    grid.for_each(|cell, x, y| {
        // find an inner corner
        if x == grid.x_bounds().start || x == grid.x_bounds().end - 1
            || y == grid.y_bounds().start || y == grid.y_bounds().end - 1
            || on_cube(cell) {
                return;
        }
        let mut n:i64 = 0;
        let mut above = false;
        let mut below = false;
        let mut toleft = false;
        let mut toright = false;
        if on_cube(grid.get(x-1, y)) { n += 1; toleft = true; }
        if on_cube(grid.get(x+1, y)) { n += 1; toright = true; }
        if on_cube(grid.get(x, y-1)) { n += 1; above = true; }
        if on_cube(grid.get(x, y+1)) { n += 1; below = true; }
        if n != 2 { return; }
        corners.borrow_mut().push(
            match (above, toright, below, toleft) {
                (true, true, false, false) => (x, y, Dir::Right, Dir::Up),
                (false, true, true, false) => (x, y, Dir::Right, Dir::Down),
                (false, false, true, true) => (x, y, Dir::Down, Dir::Left),
                (true, false, false, true) => (x, y, Dir::Up, Dir::Left),
                _ => panic!(),
            }
        );
    });
    let corners = corners.into_inner();
    assert!(corners.len() == 3);
    fn mktup(x:i64, y:i64, dir:Dir) -> (i64,i64,Dir) {
        let (nx, ny) = match dir {
            Dir::Right => (x+1, y),
            Dir::Left => (x-1, y),
            Dir::Up => (x, y-1),
            Dir::Down => (x, y+1),
        };
        (nx, ny, dir)
    }
    for (x, y, dir1, dir2) in corners {
        let c = Cell::WrapCorner(mktup(x, y, dir1), mktup(x, y, dir2));
        println!("({x},{y}) -> {c:?}");
        grid.set(x, y, c);
    }
    */
    grid.set(100, 50, Cell::WrapCorner((100, 49, Dir::Up), (99, 50, Dir::Left)));
    grid.set(49, 99, Cell::WrapCorner((50, 99, Dir::Right), (49, 100, Dir::Down)));
    grid.set(50, 150, Cell::WrapCorner((50, 149, Dir::Up), (49, 150, Dir::Left)));

    connect_edge(grid,
        &mut (0..49i64).map(|x| (x, 99i64)), Dir::Down,
        &mut (50..99i64).map(|y| (49i64, y)), Dir::Right,
    );
    connect_edge(grid,
        &mut (100..150i64).map(|y| (-1i64, y)), Dir::Right,
        &mut (0..50i64).rev().map(|y| (49i64, y)), Dir::Right,
    );
    connect_edge(grid,
        &mut (50..100i64).map(|x| (x, -1i64)), Dir::Down,
        &mut (150..200i64).map(|y| (-1i64, y)), Dir::Right,
    );
    connect_edge(grid,
        &mut (0..50i64).map(|x| (x, 200i64)), Dir::Up,
        &mut (100..150i64).map(|x| (x, -1i64)), Dir::Down,
    );
    connect_edge(grid,
        &mut (51..100i64).map(|y| (100i64, y)), Dir::Left,
        &mut (101..150i64).map(|x| (x, 50i64)), Dir::Up,
    );
    connect_edge(grid,
        &mut (100..150i64).map(|y| (100i64, y)), Dir::Left,
        &mut (0..50i64).rev().map(|y| (150i64, y)), Dir::Left,
    );
    connect_edge(grid,
        &mut (151..200i64).map(|y| (50i64, y)), Dir::Left,
        &mut (51..100i64).map(|x| (x, 150i64)), Dir::Up,
    );
}

fn part(grid: &mut Grid<Cell>, moves: &Vec<Move>, partnum:i64) {
    let mut pos = Pos {x: 0, y: 0, dir: Dir::Right};
    for x in grid.x_bounds() {
        match grid.get(x, 0) {
            Cell::Open => { pos.x = x; break; },
            _ => {},
        }
    }
    if partnum == 2 {
        fold(grid);
    }
    for m in moves {
        pos.apply_move(m, grid);
    }
    println!("Part {}: {}", partnum, pos.password());
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input::<String>();
    let mut grid: Grid<Cell> = Grid::from_input(&input[0], Cell::Void, 1, |c| match c {
        ' ' => Cell::Void,
        '#' => Cell::Wall,
        '.' => Cell::Open,
        _ => panic!(),
    });
    let moves: Vec<Move> =
        Regex::new(r"(\d+)|([LR])").unwrap()
        .find_iter(&input[1][0])
        .map(|m| match m.as_str() {
            "L" => Move::TurnLeft,
            "R" => Move::TurnRight,
            d => Move::Forward(d.parse::<usize>().unwrap()),
        })
        .collect();
    part(&mut grid, &moves, 1);
    part(&mut grid, &moves, 2);
}
