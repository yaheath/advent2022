use std::collections::HashMap;
use std::cmp::{min, max};
use std::ops::Range;
use std::vec::Vec;
use advent_lib::infinite_grid::InfiniteGrid;
use advent_lib::read::read_input;

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Elf,
}

#[derive(Copy, Clone)]
struct Elf {
    x: i64,
    y: i64,
    proposed: Option<(i64, i64)>,
}

enum Dir {
    North,
    East,
    South,
    West,
}

fn step(grid: &mut InfiniteGrid<Cell>, elves: &mut Vec<Elf>, searchorder: &mut Vec<Dir>) -> bool {
    let mut proposed: HashMap<(i64,i64),usize> = HashMap::new();
    let mut done = true;

    // phase 1
    for elf in elves.iter_mut() {
        let elfat = |x, y| -> bool { match grid.get(x, y) { Cell::Elf => true, _ => false } };
        elf.proposed = None;
        if elfat(elf.x - 1, elf.y - 1) || elfat(elf.x, elf.y - 1) || elfat(elf.x + 1, elf.y - 1)
                || elfat(elf.x - 1, elf.y) ||                             elfat(elf.x + 1, elf.y)
                || elfat(elf.x - 1, elf.y + 1) || elfat(elf.x, elf.y + 1) || elfat(elf.x + 1, elf.y + 1) {
            for dir in searchorder.iter() {
                match dir {
                    Dir::North => {
                        if !elfat(elf.x - 1, elf.y - 1) && !elfat(elf.x, elf.y - 1) && !elfat(elf.x + 1, elf.y - 1) {
                            elf.proposed = Some((elf.x, elf.y - 1));
                            break;
                        }
                    },
                    Dir::South => {
                        if !elfat(elf.x - 1, elf.y + 1) && !elfat(elf.x, elf.y + 1) && !elfat(elf.x + 1, elf.y + 1) {
                            elf.proposed = Some((elf.x, elf.y + 1));
                            break;
                        }
                    },
                    Dir::West => {
                        if !elfat(elf.x - 1, elf.y - 1) && !elfat(elf.x - 1, elf.y) && !elfat(elf.x - 1, elf.y + 1) {
                            elf.proposed = Some((elf.x - 1, elf.y));
                            break;
                        }
                    },
                    Dir::East => {
                        if !elfat(elf.x + 1, elf.y - 1) && !elfat(elf.x + 1, elf.y) && !elfat(elf.x + 1, elf.y + 1) {
                            elf.proposed = Some((elf.x + 1, elf.y));
                            break;
                        }
                    },
                }
            }
            match elf.proposed {
                Some(p) => {
                    if let Some(val) = proposed.get_mut(&p) {
                        *val += 1;
                    } else {
                        proposed.insert(p, 1);
                    }
                },
                None => {},
            }
        }
    }

    // phase 2
    for elf in elves.iter_mut() {
        match elf.proposed {
            Some(p) => {
                if proposed[&p] == 1 {
                    done = false;
                    grid.set(elf.x, elf.y, Cell::Empty);
                    elf.x = p.0;
                    elf.y = p.1;
                    grid.set(elf.x, elf.y, Cell::Elf);
                }
            },
            None => {},
        };
    }

    let s = searchorder.splice(0..1, []).next().unwrap();
    searchorder.push(s);
    done
}

fn run(input: &Vec<String>, max_iters: i64) -> (i64, usize) {
    let mut grid: InfiniteGrid<Cell> = InfiniteGrid::from_input(&input, Cell::Empty, |c, _, _| match c {
        '.' => None,
        '#' => Some(Cell::Elf),
        _ => panic!(),
    });
    let mut elves:Vec<Elf> = grid.iter().map(|((x,y),_)| Elf { x: *x, y: *y, proposed: None }).collect();
    let mut searchorder:Vec<Dir> = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    //grid.print(|c| match c { Cell::Elf => '#', _ => '.' });
    //println!("");
    let mut n_iters: usize = 1;
    for _ in 0..max_iters {
        if step(&mut grid, &mut elves, &mut searchorder) {
            break;
        }
        n_iters += 1;
        //grid.print(|c| match c { Cell::Elf => '#', _ => '.' });
        //println!("");
    }
    let mut x_range: Range<i64> = Range { start: 0, end: 0 };
    let mut y_range: Range<i64> = Range { start: 0, end: 0 };
    for ((x, y), c) in grid.iter() {
        match c {
            Cell::Elf => {
                if x_range.is_empty() {
                    x_range.start = *x;
                    x_range.end = *x + 1;
                    y_range.start = *y;
                    y_range.end = *y + 1;
                } else {
                    x_range.start = min(x_range.start, *x);
                    x_range.end = max(x_range.end, *x + 1);
                    y_range.start = min(y_range.start, *y);
                    y_range.end = max(y_range.end, *y + 1);
                }
            },
            _ => {},
        };
    }
    let empties = (x_range.end - x_range.start) * (y_range.end - y_range.start) - (elves.len() as i64);
    (empties, n_iters)
}

fn part1(input: &Vec<String>) -> i64 {
    let (value, _) = run(input, 10);
    value
}

fn part2(input: &Vec<String>) -> usize {
    let (_, value) = run(input, i64::MAX);
    value
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<Pair> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 110);
        assert_eq!(part2(&input), 20);
    }
}
