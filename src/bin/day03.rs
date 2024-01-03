use std::vec::Vec;
use std::collections::HashSet;
use std::str::FromStr;
use advent_lib::read::read_input;

struct Rucksack {
    all: HashSet<char>,
    left: HashSet<char>,
    right: HashSet<char>,
}

impl FromStr for Rucksack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.trim_end().chars().collect();
        let len = chars.len() / 2;
        Ok(Rucksack {
            all: HashSet::from_iter(chars.iter().cloned()),
            left: HashSet::from_iter(chars[0..len].iter().cloned()),
            right: HashSet::from_iter(chars[len..].iter().cloned()),
        })
    }
}

impl Rucksack {
    fn common(&self) -> char {
        *self.left.intersection(&self.right).next().unwrap()
    }
}

fn priority(c: char) -> i32 {
    match c {
        'A'..='Z' => (c as i32) - ('A' as i32) + 27,
        'a'..='z' => (c as i32) - ('a' as i32) + 1,
        _ => 0,
    }
}

fn part1(input: &Vec<Rucksack>) -> i32 {
    input.iter()
        .map(|r| r.common())
        .map(|c| priority(c))
        .sum()
}

fn part2(input: &Vec<Rucksack>) -> i32 {
    input.chunks(3)
        .map(|chunk|
            *(chunk[0].all
                .intersection(&chunk[1].all)
                .map(|c| *c)
                .collect::<HashSet<char>>()
                .intersection(&chunk[2].all)
                .next()
                .unwrap())
        ).map(|c| priority(c))
        .sum()
}

fn main() {
    let input: Vec<Rucksack> = read_input::<Rucksack>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input: Vec<Rucksack> = test_input(include_str!("day03.testinput"));
        assert_eq!(part1(&input), 157);
        assert_eq!(part2(&input), 70);
    }
}
