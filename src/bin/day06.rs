use std::collections::{HashSet, VecDeque};
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn find_marker(input: &String, n_distinct: usize) -> usize {
    let mut four: VecDeque<char> = VecDeque::with_capacity(n_distinct + 1);
    for (i, c) in input.chars().enumerate() {
        four.push_back(c);
        if four.len() == n_distinct + 1 {
            four.pop_front();
        }
        if four.len() == n_distinct {
            let set:HashSet<char> = HashSet::from_iter(four.iter().copied());
            if set.len() == n_distinct {
                return i + 1;
            }
        }
    }
    return 0;
}

fn part1(input: &Vec<String>) -> usize {
    find_marker(&input[0], 4)
}

fn part2(input: &Vec<String>) -> usize {
    find_marker(&input[0], 14)
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
    fn day06_test() {
        let input: Vec<String> = test_input("mjqjpqmgbljsphdztnvjfqwrcgsmlb\n");
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input), 19);
        let input: Vec<String> = test_input("bvwbjplbgvbhsrlpgdmjqwftvncz\n");
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 23);
        let input: Vec<String> = test_input("nppdvjthqldpwncqszvftbrmjlhg\n");
        assert_eq!(part1(&input), 6);
        assert_eq!(part2(&input), 23);
        let input: Vec<String> = test_input("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg\n");
        assert_eq!(part1(&input), 10);
        assert_eq!(part2(&input), 29);
        let input: Vec<String> = test_input("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw\n");
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 26);
    }
}
