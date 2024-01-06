use std::collections::HashSet;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::{CDir, Coord2D};

struct Move {
    dir: CDir,
    steps: i64,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w) (\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let dir = match caps.get(1).unwrap().as_str() {
                "L" => CDir::W,
                "R" => CDir::E,
                "U" => CDir::N,
                "D" => CDir::S,
                _ => panic!(),
            };
            Ok(Move {
                dir,
                steps: caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            })
        }
        else {
            Err("invalid input row".into())
        }
    }
}

trait MoveToward {
    fn move_toward(&mut self, other: &Self);
}

impl MoveToward for Coord2D {
    fn move_toward(&mut self, other: &Coord2D) {
        if other.x == self.x {
            let diff = other.y - self.y;
            if diff > 1 {
                self.y += 1;
            }
            else if diff < -1 {
                self.y -= 1;
            }
        }
        else if other.y == self.y {
            let diff = other.x - self.x;
            if diff > 1 {
                self.x += 1;
            }
            else if diff < -1 {
                self.x -= 1;
            }
        }
        else {
            let dx = other.x - self.x;
            let dy = other.y - self.y;
            if dx.abs() > 1 || dy.abs() > 1 {
                self.x += if dx > 0 {1} else {-1};
                self.y += if dy > 0 {1} else {-1};
            }
        }
    }
}

fn do_moves(input: &[Move], depth: usize) -> usize {
    let mut visited: HashSet<Coord2D> = HashSet::new();
    let mut chain = vec![Coord2D::new(0,0); depth];
    for mv in input {
        for _ in 0..mv.steps {
            let head = chain.get_mut(0).unwrap();
            *head += mv.dir;
            for i in 1..depth {
                let prev = chain[i-1];
                let cur = chain.get_mut(i).unwrap();
                cur.move_toward(&prev);
            }
            visited.insert(chain[depth-1]);
        }
    }
    visited.len()
}

fn part1(input: &[Move]) -> usize {
    do_moves(input, 2)
}

fn part2(input: &[Move]) -> usize {
    do_moves(input, 10)
}

fn main() {
    let input = read_input::<Move>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input: Vec<Move> = test_input(include_str!("day09.testinput"));
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 1);
        let input: Vec<Move> = test_input(include_str!("day09.test2input"));
        assert_eq!(part2(&input), 36);
    }
}
