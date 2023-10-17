use std::vec::Vec;
use itertools::Itertools;
use advent_lib::read::read_grouped_input;

fn part1(input: &Vec<Vec<i64>>) {
    if let Some(max) = input.iter().map(|row| row.iter().sum::<i64>()).max() {
        println!("Part 1: {}", max);
    }
}

fn part2(input: &Vec<Vec<i64>>) {
    let val:i64 = input.iter()
        .map(|row| row.iter().sum::<i64>())
        .sorted()
        .rev()
        .take(3)
        .sum();
    println!("Part 2: {}", val);
}

fn main() {
    let input: Vec<Vec<i64>> = read_grouped_input::<i64>();
    part1(&input);
    part2(&input);
}
