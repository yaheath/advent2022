use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

enum Push {
    Left,
    Right,
}

struct PushList(Vec<Push>);

impl FromStr for PushList {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pat:Vec<Push> = s.chars().filter(|c| *c == '<' || *c == '>').map(|c|
            match c {
                '<' => Push::Left,
                '>' => Push::Right,
                _ => panic!(),
            }
        ).collect();
        Ok(PushList(pat))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    A,
    B,
    C,
    D,
    E,
}

struct Rock<'a> {
    id: Cell,
    cells: &'a [(usize,usize)],
}

const ROCKS: &[Rock] = &[
    Rock { id: Cell::A, cells: &[ (0, 0), (1, 0), (2, 0), (3, 0) ]},
    Rock { id: Cell::B, cells: &[ (1, 0), (0, 1), (1, 1), (2, 1), (1, 2) ]},
    Rock { id: Cell::C, cells: &[ (0, 0), (1, 0), (2, 0), (2, 1), (2, 2) ]},
    Rock { id: Cell::D, cells: &[ (0, 0), (0, 1), (0, 2), (0, 3) ]},
    Rock { id: Cell::E, cells: &[ (0, 0), (1, 0), (0, 1), (1, 1) ]},
];

type Row = [Cell; 7];

impl Rock<'_> {
    fn collides(&self, x: usize, y: usize, shaft: &[Row]) -> bool {
        for (cx, cy) in self.cells {
            if cx + x > 6 { return true; }
            if cy + y >= shaft.len() { continue; }
            match shaft[cy + y][cx + x] {
                Cell::Empty => { continue; },
                _ => { return true; },
            }
        }
        false
    }

    fn place(&self, x: usize, y: usize, shaft: &mut Vec<Row>) {
        for (cx, cy) in self.cells {
            while y + cy >= shaft.len() {
                shaft.push([Cell::Empty; 7]);
            }
            shaft[y + cy][x + cx] = self.id;
        }
    }
}

fn simulate(pushpattern: &[Push], n_rocks: usize) -> usize {
    let mut shaft: Vec<Row> = Vec::new();
    let mut cur_rock_idx: usize = 0;
    let mut cur_push_idx: usize = 0;
    let mut seen: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();
    let mut rock_count: usize = 0;
    let mut repeated_rows: usize = 0;

    while rock_count < n_rocks {
        let top = shaft.len();
        let mut y = top + 3;
        let mut x: usize = 2;
        let rock: &Rock = &ROCKS[cur_rock_idx];

        loop {
            if let Some(newx) = match pushpattern[cur_push_idx] {
                Push::Left => x.checked_sub(1),
                Push::Right => x.checked_add(1),
            } {
                if !rock.collides(newx, y, &shaft) {
                    x = newx;
                }
            }
            cur_push_idx = (cur_push_idx + 1) % pushpattern.len();

            if y == 0 || rock.collides(x, y - 1, &shaft) {
                rock.place(x, y, &mut shaft);
                break;
            }
            y -= 1;
        }

        cur_rock_idx = (cur_rock_idx + 1) % ROCKS.len();
        rock_count += 1;

        if repeated_rows == 0 {
            let key = (cur_rock_idx, cur_push_idx);
            if let Some((2, old_rock_count, old_top)) = seen.get(&key) {
                let delta_top = shaft.len() - old_top;
                let delta_rock_count = rock_count - old_rock_count;
                let repeats = (n_rocks - rock_count) / delta_rock_count;
                repeated_rows = repeats * delta_top;
                rock_count += repeats * delta_rock_count;
            }
            else {
                seen.entry(key)
                    .and_modify(|(seen_count, old_rock_count, old_top)| {
                        *seen_count += 1;
                        *old_rock_count = rock_count;
                        *old_top = shaft.len();
                    })
                    .or_insert((1, rock_count, shaft.len()));
            }
        }
    }
    shaft.len() + repeated_rows
}

fn part1(pushpattern: &[Push]) -> usize {
    simulate(pushpattern, 2022)
}

fn part2(pushpattern: &[Push]) -> usize {
    simulate(pushpattern, 1000000000000)
}

fn main() {
    let input: Vec<PushList> = read_input();
    println!("Part 1: {}", part1(&input[0].0));
    println!("Part 2: {}", part2(&input[0].0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day17_test() {
        let input: Vec<PushList> = test_input(include_str!("day17.testinput"));
        assert_eq!(part1(&input[0].0), 3068);
        assert_eq!(part2(&input[0].0), 1514285714288);
    }
}
