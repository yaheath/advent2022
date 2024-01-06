use std::collections::HashMap;
use std::cmp::{min, max};
use std::ops::Range;
use std::vec::Vec;
use ya_advent_lib::infinite_grid::InfiniteGrid;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Elf,
}

#[derive(Copy, Clone)]
struct Elf {
    loc: Coord2D,
    proposed: Option<Coord2D>,
}

fn step(grid: &mut InfiniteGrid<Cell>, elves: &mut [Elf], searchorder: &mut Vec<CDir>) -> bool {
    let mut proposed: HashMap<Coord2D,usize> = HashMap::new();
    let mut done = true;

    // phase 1
    for elf in elves.iter_mut() {
        let elfat = |loc| -> bool { matches!(grid.get_c(loc), Cell::Elf) };
        elf.proposed = None;
        if elf.loc.neighbors8().iter().any(|c| elfat(*c)) {
            for dir in searchorder.iter() {
                match dir {
                    CDir::N => {
                        let nor = elf.loc + CDir::N;
                        if !elfat(nor + CDir::E) && !elfat(nor) && !elfat(nor + CDir::W) {
                            elf.proposed = Some(nor);
                            break;
                        }
                    },
                    CDir::S => {
                        let sou = elf.loc + CDir::S;
                        if !elfat(sou + CDir::E) && !elfat(sou) && !elfat(sou + CDir::W) {
                            elf.proposed = Some(sou);
                            break;
                        }
                    },
                    CDir::W => {
                        let wes = elf.loc + CDir::W;
                        if !elfat(wes + CDir::N) && !elfat(wes) && !elfat(wes + CDir::S) {
                            elf.proposed = Some(wes);
                            break;
                        }
                    },
                    CDir::E => {
                        let eas = elf.loc + CDir::E;
                        if !elfat(eas + CDir::N) && !elfat(eas) && !elfat(eas + CDir::S) {
                            elf.proposed = Some(eas);
                            break;
                        }
                    },
                }
            }
            if let Some(p) = elf.proposed {
                proposed.entry(p)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    // phase 2
    for elf in elves.iter_mut() {
        if let Some(p) = elf.proposed {
            if proposed[&p] == 1 {
                done = false;
                grid.set_c(elf.loc, Cell::Empty);
                elf.loc = p;
                grid.set_c(elf.loc, Cell::Elf);
            }
        }
    }

    let s = searchorder.splice(0..1, []).next().unwrap();
    searchorder.push(s);
    done
}

fn run(input: &[String], max_iters: i64) -> (i64, usize) {
    let mut grid: InfiniteGrid<Cell> = InfiniteGrid::from_input(input, Cell::Empty, |c, _, _| match c {
        '.' => None,
        '#' => Some(Cell::Elf),
        _ => panic!(),
    });
    let mut elves:Vec<Elf> = grid.iter().map(|((x,y),_)| Elf { loc: (*x, *y).into(), proposed: None }).collect();
    let mut searchorder:Vec<CDir> = vec![CDir::N, CDir::S, CDir::W, CDir::E];
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
    for ((x, y), _) in grid.iter().filter(|(_, c)| matches!(c, Cell::Elf)) {
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
    }
    let empties = (x_range.end - x_range.start) * (y_range.end - y_range.start) - (elves.len() as i64);
    (empties, n_iters)
}

fn part1(input: &[String]) -> i64 {
    let (value, _) = run(input, 10);
    value
}

fn part2(input: &[String]) -> usize {
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
    use ya_advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<String> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 110);
        assert_eq!(part2(&input), 20);
    }
}
