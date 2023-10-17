use std::collections::HashSet;
use std::collections::VecDeque;
use std::vec::Vec;
use advent_lib::read::read_input;

fn find_marker(input: &String, n_distinct: usize) -> usize {
    let mut four: VecDeque<char> = VecDeque::with_capacity(n_distinct + 1);
    for (i, c) in input.chars().enumerate() {
        four.push_back(c);
        if four.len() == n_distinct + 1 {
            four.pop_front();
        }
        if four.len() == n_distinct {
            let set:HashSet<char> = HashSet::from_iter(four.iter().cloned());
            if set.len() == n_distinct {
                return i + 1;
            }
        }
    }
    return 0;
}

fn part1(input: &Vec<String>) {
    let value = find_marker(&input[0], 4);
    println!("Part 1: {}", value);
}

fn part2(input: &Vec<String>) {
    let value = find_marker(&input[0], 14);
    println!("Part 2: {}", value);
}

fn main() {
    let input: Vec<String> = read_input::<String>();
    part1(&input);
    part2(&input);
}
