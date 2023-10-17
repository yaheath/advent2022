#[macro_use] extern crate lazy_static;
use std::vec::Vec;
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_sectioned_input;

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
        if let Some(caps) = RE.captures(s) {
            let mut cols:[char; 9] = [' '; 9];
            for i in 0..9 {
                let c = caps.get(i+1).unwrap();
                cols[i] = c.as_str().chars().next().unwrap();
            }
            Ok(StackRow { cols: cols })
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

fn part(input: &Input, part: i32) {
    let mut stacks: [Vec<char>; 9] = Default::default();
    let moves = &input.1;
    for row in &input.0 {
        for i in 0..9 {
            if row.cols[i] != ' ' {
                stacks[i].push(row.cols[i]);
            }
        }
    }
    for i in 0..9 {
        stacks[i].reverse();
    }
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
    let mut out: String = String::new();
    for i in 0..9 {
        out.push(*stacks[i].last().unwrap())
    }
    println!("Part {}: {}", part, out);
}

fn main() {
    let input: Input = read_sectioned_input();
    part(&input, 1);
    part(&input, 2);
}
