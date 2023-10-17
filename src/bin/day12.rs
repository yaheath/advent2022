use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Start,
    End,
    Elev(u8),
    Uninitialized,
}

impl Cell {
    fn elev(&self) -> u8 {
        match self {
            Cell::Elev(e) => *e,
            Cell::Start => 0,
            Cell::End => 25,
            Cell::Uninitialized => 100,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    x: i64,
    y: i64,
    dist: i64,
}

impl State {
    fn new(x: i64, y: i64, dist: i64) -> Self {
        State {
            dist: dist,
            x: x,
            y: y,
        }
    }
    fn key(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.y.cmp(&self.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(grid: &Grid<Cell>, startx: i64, starty: i64) -> i64 {
    let mut dists: HashMap<(i64,i64), i64> = HashMap::new();
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State::new(startx, starty, 0));

    while let Some(state) = heap.pop() {
        if dists.contains_key(&state.key()) && state.dist > dists[&state.key()] {
            continue;
        }
        let cell = grid.get(state.x, state.y);
        match cell {
            Cell::End => {
                return state.dist;
            },
            _ => {},
        }
        let elev:u8 = cell.elev() ;
        let mut check = |x: i64, y: i64| {
            if !grid.x_bounds().contains(&x) || !grid.y_bounds().contains(&y) {
                return;
            }
            let ncell = grid.get(x, y);
            if ncell.elev() <= elev + 1 {
                if !dists.contains_key(&(x,y)) || state.dist + 1 < dists[&(x,y)] {
                    dists.insert((x,y), state.dist + 1);
                    heap.push(State::new(x, y, state.dist + 1));
                }
            }
        };
        check(state.x + 1, state.y);
        check(state.x, state.y + 1);
        check(state.x - 1, state.y);
        check(state.x, state.y - 1);
    }
    -1
}

fn part1(grid: &Grid<Cell>) {
    if let Some((startx, starty)) = grid.find(|c,_,_| c == Cell::Start) {
        let d = search(grid, startx, starty);
        println!("Part 1: {}", d);
    }
}

fn part2(grid: &Grid<Cell>) {
    let min: RefCell<i64> = RefCell::new(-1);

    grid.for_each(|cell, x, y| {
        match cell {
            Cell::Start | Cell::Elev(0) => {},
            _ => { return; }
        }
        let val = search(grid, x, y);
        if val < 0 { return; }
        let minval = *min.borrow();
        if minval == -1 || val < minval {
            *min.borrow_mut() = val;
        }
    });
    println!("Part 2: {}", min.borrow());
}

fn main() {
    let data = read_input::<String>();
    let grid = Grid::from_input(&data, Cell::Uninitialized, 0, |c| match c {
        'S' => Cell::Start,
        'E' => Cell::End,
        'a'..='z' => Cell::Elev((c as u8) - b'a'),
        _ => Cell::Uninitialized,
    });
    part1(&grid);
    part2(&grid);
}
