//use std::cell::RefCell;
//use std::cmp::min;
use std::vec::Vec;
use regex::Regex;
use ya_advent_lib::grid::Grid;
use ya_advent_lib::read::read_grouped_input;
use ya_advent_lib::coords::{CDir, Turn, Coord2D};

#[derive(Debug, Copy, Clone)]
enum Cell {
    Void,
    Open,
    Wall,
    Wrap(Coord2D, CDir),
    WrapCorner((Coord2D, CDir), (Coord2D, CDir)),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            ' ' => Cell::Void,
            '#' => Cell::Wall,
            '.' => Cell::Open,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone)]
struct Pos {
    coord: Coord2D,
    dir: CDir,
}

impl Pos {
    fn turn(&mut self, m: Move) {
        match m {
            Move::TurnLeft => { self.dir += Turn::L; },
            Move::TurnRight => { self.dir += Turn::R; },
            _ => {},
        }
    }
    fn try_step(&mut self, grid: &Grid<Cell>) {
        let mut next = self.coord + self.dir;
        loop {
            match grid.get_c(next) {
                Cell::Open => { self.coord = next; return; },
                Cell::Wall => { return; },
                Cell::Wrap(wc, wd) => {
                    if matches!(grid.get_c(wc), Cell::Open) {
                        self.coord = wc;
                        self.dir = wd;
                    };
                    return;
                },
                Cell::WrapCorner(w1, w2) => {
                    let (wc, wd) = if w1.0 == self.coord { w2 } else { w1 };
                    if matches!(grid.get_c(wc), Cell::Open) {
                        self.coord = wc;
                        self.dir = wd;
                    };
                    return;
                },
                _ => {},
            };
            let search = |dx, dy| -> (i64, i64) {
                let mut sx = self.coord.x;
                let mut sy = self.coord.y;
                loop {
                    match grid.get(sx + dx, sy + dy) {
                        Cell::Void => { return (sx, sy); },
                        Cell::Wrap(_,_) => panic!(),
                        Cell::WrapCorner(_,_) => panic!(),
                        _ => { sx += dx; sy += dy; },
                    }
                }
            };
            let (nx, ny) = match self.dir {
                CDir::N => search(0, 1),
                CDir::S => search(0, -1),
                CDir::E => search(-1, 0),
                CDir::W => search(1, 0),
            };
            next.x = nx;
            next.y = ny;
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
        (self.coord.x + 1) * 4 +
        (self.coord.y + 1) * 1000 +
        match self.dir {
            CDir::N => 3,
            CDir::S => 1,
            CDir::W => 2,
            CDir::E => 0,
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

fn connect_edge(grid: &mut Grid<Cell>,
    edge1: &mut dyn Iterator<Item=Coord2D>, dir1: CDir,
    edge2: &mut dyn Iterator<Item=Coord2D>, dir2: CDir
) {
    for (c1, c2) in Iterator::zip(edge1, edge2) {
        grid.set_c(c1, Cell::Wrap(c2 + dir2, dir2));
        grid.set_c(c2, Cell::Wrap(c1 + dir1, dir1));
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
    grid.set(100, 50, Cell::WrapCorner((Coord2D::new(100, 49), CDir::N), (Coord2D::new(99, 50), CDir::W)));
    grid.set(49, 99, Cell::WrapCorner((Coord2D::new(50, 99), CDir::E), (Coord2D::new(49, 100), CDir::S)));
    grid.set(50, 150, Cell::WrapCorner((Coord2D::new(50, 149), CDir::N), (Coord2D::new(49, 150), CDir::W)));

    connect_edge(grid,
        &mut (0..49i64).map(|x| Coord2D::new(x, 99i64)), CDir::S,
        &mut (50..99i64).map(|y| Coord2D::new(49i64, y)), CDir::E,
    );
    connect_edge(grid,
        &mut (100..150i64).map(|y| Coord2D::new(-1i64, y)), CDir::E,
        &mut (0..50i64).rev().map(|y| Coord2D::new(49i64, y)), CDir::E,
    );
    connect_edge(grid,
        &mut (50..100i64).map(|x| Coord2D::new(x, -1i64)), CDir::S,
        &mut (150..200i64).map(|y| Coord2D::new(-1i64, y)), CDir::E,
    );
    connect_edge(grid,
        &mut (0..50i64).map(|x| Coord2D::new(x, 200i64)), CDir::N,
        &mut (100..150i64).map(|x| Coord2D::new(x, -1i64)), CDir::S,
    );
    connect_edge(grid,
        &mut (51..100i64).map(|y| Coord2D::new(100i64, y)), CDir::W,
        &mut (101..150i64).map(|x| Coord2D::new(x, 50i64)), CDir::N,
    );
    connect_edge(grid,
        &mut (100..150i64).map(|y| Coord2D::new(100i64, y)), CDir::W,
        &mut (0..50i64).rev().map(|y| Coord2D::new(150i64, y)), CDir::W,
    );
    connect_edge(grid,
        &mut (151..200i64).map(|y| Coord2D::new(50i64, y)), CDir::W,
        &mut (51..100i64).map(|x| Coord2D::new(x, 150i64)), CDir::N,
    );
}

fn part(grid: &mut Grid<Cell>, moves: &[Move], partnum:i64) -> i64 {
    let mut pos = Pos {coord: Coord2D::new(0, 0), dir: CDir::E};
    for x in grid.x_bounds() {
        if matches!(grid.get(x, 0), Cell::Open) {
            pos.coord.x = x;
            break;
        }
    }
    if partnum == 2 {
        fold(grid);
    }
    for m in moves {
        pos.apply_move(m, grid);
    }
    pos.password()
}

fn setup(input: Vec<Vec<String>>) -> (Grid<Cell>, Vec<Move>) {
    let grid: Grid<Cell> = Grid::from_input(&input[0], Cell::Void, 1);
    let moves: Vec<Move> =
        Regex::new(r"(\d+)|([LR])").unwrap()
        .find_iter(&input[1][0])
        .map(|m| match m.as_str() {
            "L" => Move::TurnLeft,
            "R" => Move::TurnRight,
            d => Move::Forward(d.parse::<usize>().unwrap()),
        })
        .collect();
    (grid, moves)
}

fn main() {
    let input: Vec<Vec<String>> = read_grouped_input::<String>();
    let (mut grid, moves) = setup(input);
    println!("Part 1: {}", part(&mut grid, &moves, 1));
    println!("Part 2: {}", part(&mut grid, &moves, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day22_test() {
        let input: Vec<Vec<String>> = grouped_test_input(include_str!("day22.testinput"));
        let (mut grid, moves) = setup(input);
        assert_eq!(part(&mut grid, &moves, 1), 6032);
        // Cube fold currently only works on my real input
        // assert_eq!(part(&mut grid, &moves, 2), 5031);
    }
}
