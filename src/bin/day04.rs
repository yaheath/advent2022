#[macro_use] extern crate lazy_static;
use std::vec::Vec;
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_input;

struct Pair {
    //a: HashSet<u32>,
    //b: HashSet<u32>,
    a_frm: u32,
    a_to: u32,
    b_frm: u32,
    b_to: u32,
}

impl FromStr for Pair {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Pair {
                a_frm: caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                a_to: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                b_frm: caps.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                b_to: caps.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            })
        }
        else {
            Err("invalid input line".to_string())
        }
    }
}

impl Pair {
    fn is_contained(&self) -> bool {
        self.a_frm >= self.b_frm && self.a_to <= self.b_to
            || self.b_frm >= self.a_frm && self.b_to <= self.a_to
    }
    fn is_overlapped(&self) -> bool {
        self.a_frm <= self.b_frm && self.a_to >= self.b_frm
            || self.b_frm <= self.a_frm && self.b_to >= self.a_frm
    }
}

fn part1(input: &Vec<Pair>) {
    let value = input.iter().filter(|p| p.is_contained()).count();
    println!("Part 1: {}", value);
}

fn part2(input: &Vec<Pair>) {
    let value = input.iter().filter(|p| p.is_overlapped()).count();
    println!("Part 2: {}", value);
}

fn main() {
    let input: Vec<Pair> = read_input::<Pair>();
    part1(&input);
    part2(&input);
}
