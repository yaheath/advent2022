use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

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
impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'S' => Cell::Start,
            'E' => Cell::End,
            'a'..='z' => Cell::Elev((value as u8) - b'a'),
            _ => Cell::Uninitialized,
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
            dist,
            x,
            y,
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
        if cell == Cell::End {
            return state.dist;
        }
        let elev:u8 = cell.elev() ;
        let mut check = |x: i64, y: i64| {
            if !grid.x_bounds().contains(&x) || !grid.y_bounds().contains(&y) {
                return;
            }
            let ncell = grid.get(x, y);
            if ncell.elev() <= elev + 1 && (!dists.contains_key(&(x,y)) || state.dist + 1 < dists[&(x,y)]) {
                dists.insert((x,y), state.dist + 1);
                heap.push(State::new(x, y, state.dist + 1));
            }
        };
        check(state.x + 1, state.y);
        check(state.x, state.y + 1);
        check(state.x - 1, state.y);
        check(state.x, state.y - 1);
    }
    i64::MAX
}

fn part1(grid: &Grid<Cell>) -> i64 {
    let (startx, starty) = grid.find(|c,_,_| c == Cell::Start).unwrap();
    search(grid, startx, starty)
}

fn part2(grid: &Grid<Cell>) -> i64 {
    grid.iter_with_coord()
        .map(|(cell, x, y)| {
            match cell {
                Cell::Start | Cell::Elev(0) => {},
                _ => { return i64::MAX; }
            }
            search(grid, x, y)
        })
        .min()
        .unwrap()
}

fn mkgrid(input: Vec<String>) -> Grid<Cell> {
    Grid::from_input(&input, Cell::Uninitialized, 0)
}

fn main() {
    let input = read_input::<String>();
    let grid = mkgrid(input);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day12_test() {
        let input: Vec<String> = test_input(include_str!("day12.testinput"));
        let grid = mkgrid(input);
        assert_eq!(part1(&grid), 31);
        assert_eq!(part2(&grid), 29);
    }
}
