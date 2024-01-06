use std::ops::RangeInclusive;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

struct Pair {
    a: RangeInclusive<u32>,
    b: RangeInclusive<u32>,
}

impl FromStr for Pair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let a_frm = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let a_to = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let b_frm = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
            let b_to = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();
            Ok(Pair {
                a: a_frm ..= a_to,
                b: b_frm ..= b_to,
            })
        }
        else {
            Err("invalid input line".to_string())
        }
    }
}

impl Pair {
    fn is_contained(&self) -> bool {
        self.a.contains(self.b.start()) && self.a.contains(self.b.end())
            || self.b.contains(self.a.start()) && self.b.contains(self.a.end())
    }
    fn is_overlapped(&self) -> bool {
        self.a.start() <= self.b.start() && self.a.end() >= self.b.start()
            || self.b.start() <= self.a.start() && self.b.end() >= self.a.start()
    }
}

fn part1(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.is_contained()).count()
}

fn part2(input: &[Pair]) -> usize {
    input.iter().filter(|p| p.is_overlapped()).count()
}

fn main() {
    let input: Vec<Pair> = read_input::<Pair>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day04_test() {
        let input: Vec<Pair> = test_input(include_str!("day04.testinput"));
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
