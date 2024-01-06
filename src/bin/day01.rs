use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_grouped_input;

fn part1(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().map(|row| row.iter().sum::<i64>()).max().unwrap()
}

fn part2(input: &Vec<Vec<i64>>) -> i64 {
    input.iter()
        .map(|row| row.iter().sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let input: Vec<Vec<i64>> = read_grouped_input::<i64>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day01_test() {
        let input: Vec<Vec<i64>> = grouped_test_input(include_str!("day01.testinput"));
        assert_eq!(part1(&input), 24000);
        assert_eq!(part2(&input), 45000);
    }
}
