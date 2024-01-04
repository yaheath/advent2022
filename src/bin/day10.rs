#[macro_use] extern crate lazy_static;
use std::vec::Vec;
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_input;

enum Instr {
    Addx(i32),
    Noop,
}

impl FromStr for Instr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)(?: (-?\d+))?").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let op = caps.get(1).unwrap().as_str();
            match op {
                "noop" => Ok(Instr::Noop),
                "addx" => Ok(Instr::Addx(caps.get(2).unwrap().as_str().parse::<i32>().unwrap())),
                _ => Err("invalid opcode".into()),
            }
        }
        else {
            Err("invlid input line".into())
        }
    }
}

fn check_signal(x: i32, cycle: i32) -> i32 {
    if cycle >= 20 && cycle <= 220 {
        if (cycle - 20) % 40 == 0 {
            return cycle * x;
        }
    }
    0
}

fn run<F>(input: &Vec<Instr>, mut callback: F)
        where F: FnMut(i32, i32) {
    let mut x: i32 = 1;
    let mut cycle: i32 = 0;
    for inst in input {
        cycle += 1;
        callback(x, cycle);
        match inst {
            Instr::Addx(n) => {
                cycle += 1;
                callback(x, cycle);
                x += n;
            },
            Instr::Noop => {},
        }
    }
}

fn part1(input: &Vec<Instr>) -> i32 {
    let mut signal = 0i32;
    run(&input, |x, c| { signal += check_signal(x, c); });
    signal
}

fn check_pixel(x:i32, c:i32) -> bool {
    let xm = (c - 1) % 40;
    (x - xm).abs() <= 1
}

fn part2(input: &Vec<Instr>) -> String {
    let mut out = String::new();
    run(&input, |x, cycle| {
        out.push(if check_pixel(x, cycle) { '#' } else { '.' });
        if cycle % 40 == 0 {
            out.push('\n');
        }
    });
    out
}

fn main() {
    let input: Vec<Instr> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day10_test() {
        let input: Vec<Instr> = test_input(include_str!("day10.testinput"));
        assert_eq!(part1(&input), 13140);
        assert_eq!(part2(&input),
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".to_string());
    }
}
