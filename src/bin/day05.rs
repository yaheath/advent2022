use std::vec::Vec;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_sectioned_input;

struct Movement {
    n: i32,
    frm: usize,
    to: usize,
}

struct StackRow {
    cols: [char; 9],
}

impl FromStr for StackRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^.(\w| )...(\w| )...(\w| )...(\w| )...(\w| )...(\w| )...(\w| )...(\w| )...(\w| )").unwrap();
        }
        let mut ss = String::from(s);
        while ss.len() < 35 {
            // for example input, pad lines so that the RE will still match
            ss.push(' ');
        }
        if let Some(caps) = RE.captures(&ss) {
            let mut cols:[char; 9] = [' '; 9];
            for (i, col) in cols.iter_mut().enumerate() {
                let c = caps.get(i+1).unwrap();
                *col = c.as_str().chars().next().unwrap();
            }
            Ok(StackRow { cols })
        }
        else {
            Err(())
        }
    }
}

impl FromStr for Movement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Movement {
                n: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                frm: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            })
        }
        else {
            Err(())
        }
    }
}

type Input = (Vec<StackRow>, Vec<Movement>);

fn part(input: &Input, part: i32) -> String {
    let mut stacks: [Vec<char>; 9] = Default::default();
    let moves = &input.1;

    // omit the last StackRow as it will be the number labels of the stacks
    for row in &input.0[0 .. input.0.len() - 1] {
        for (i, stack) in stacks.iter_mut().enumerate() {
            if row.cols[i] != ' ' {
                stack.push(row.cols[i]);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    // println!("{stacks:?}");
    for mv in moves {
        if part == 1 {
            for _ in 0..mv.n {
                let v = stacks[mv.frm].pop().unwrap();
                stacks[mv.to].push(v);
            }
        }
        else {
            let mut tmp: Vec<char> = Vec::new();
            for _ in 0..mv.n {
                tmp.push(stacks[mv.frm].pop().unwrap());
            }
            for _ in 0..mv.n {
                stacks[mv.to].push(tmp.pop().unwrap());
            }
        }
    }
    stacks.iter().filter_map(|s| s.last()).collect::<String>()
}

fn main() {
    let input: Input = read_sectioned_input();
    println!("Part 1: {}", part(&input, 1));
    println!("Part 2: {}", part(&input, 2));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::sectioned_test_input;

    #[test]
    fn day05_test() {
        let input: Input = sectioned_test_input(include_str!("day05.testinput"));
        assert_eq!(part(&input, 1), "CMZ".to_string());
        assert_eq!(part(&input, 2), "MCD".to_string());
    }
}
