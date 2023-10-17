#[macro_use] extern crate lazy_static;
use std::cmp::{min,max};
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;

struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Cube {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Cube {
                x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                z: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            })
        }
        else {
            Err("Invalid input row".into())
        }
    }
}

fn part1(set: &HashSet<(i32,i32,i32)>) {
    let mut exposed: usize = 0;
    for (x,y,z) in set {
        if !set.contains(&(*x+1, *y, *z)) { exposed += 1; }
        if !set.contains(&(*x-1, *y, *z)) { exposed += 1; }
        if !set.contains(&(*x, *y+1, *z)) { exposed += 1; }
        if !set.contains(&(*x, *y-1, *z)) { exposed += 1; }
        if !set.contains(&(*x, *y, *z+1)) { exposed += 1; }
        if !set.contains(&(*x, *y, *z-1)) { exposed += 1; }
    }
    println!("Part 1: {}", exposed);
}

fn flood_fill(steam: &mut HashSet<(i32,i32,i32)>, set: &HashSet<(i32,i32,i32)>, coord: &(i32,i32,i32), x_range: &Range<i32>, y_range: &Range<i32>, z_range: &Range<i32>) {
    steam.insert(coord.clone());

    let mut check = |newcoord: &(i32,i32,i32)| {
        if newcoord.0 < x_range.start - 1 ||
            newcoord.0 > x_range.end ||
            newcoord.1 < y_range.start - 1 ||
            newcoord.1 > y_range.end ||
            newcoord.2 < z_range.start - 1 ||
            newcoord.2 > z_range.end ||
            steam.contains(newcoord) ||
            set.contains(newcoord) {
                return;
        }
        flood_fill(steam, set, newcoord, x_range, y_range, z_range);
    };

    check(&(coord.0+1, coord.1, coord.2));
    check(&(coord.0-1, coord.1, coord.2));
    check(&(coord.0, coord.1+1, coord.2));
    check(&(coord.0, coord.1-1, coord.2));
    check(&(coord.0, coord.1, coord.2+1));
    check(&(coord.0, coord.1, coord.2-1));
}

fn part2(set: &HashSet<(i32,i32,i32)>, x_range: &Range<i32>, y_range: &Range<i32>, z_range: &Range<i32>) {
    let mut steam: HashSet<(i32,i32,i32)> = HashSet::new();
    flood_fill(&mut steam, set, &(x_range.end, y_range.end, z_range.end), x_range, y_range, z_range);
    let mut exposed: usize = 0;
    for (x,y,z) in set {
        if steam.contains(&(*x+1, *y, *z)) { exposed += 1; }
        if steam.contains(&(*x-1, *y, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y+1, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y-1, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y, *z+1)) { exposed += 1; }
        if steam.contains(&(*x, *y, *z-1)) { exposed += 1; }
    }
    /*
    for z in (z_range.start - 1)..=z_range.end {
        for y in (y_range.start - 1)..=y_range.end {
            for x in (x_range.start - 1)..=x_range.end {
                let issteam = steam.contains(&(x,y,z));
                let islava = set.contains(&(x,y,z));
                if issteam && islava { println!("warning: {},{},{} is both steam and lava", x,y,z); }
                if issteam { print!("~"); }
                else if islava { print!("#"); }
                else { print!(" "); }
            }
            println!("");
        }
        println!("");
    }
    */
    println!("Part 2: {}", exposed);
}

fn main() {
    let input: Vec<Cube> = read_input::<Cube>();
    let mut set: HashSet<(i32,i32,i32)> = HashSet::new();
    let mut x_range: Range<i32> = Range {start: 0, end: 0};
    let mut y_range: Range<i32> = Range {start: 0, end: 0};
    let mut z_range: Range<i32> = Range {start: 0, end: 0};
    for row in input {
        set.insert((row.x, row.y, row.z));
        if x_range.is_empty() {
            x_range.start = row.x;
            x_range.end = row.x + 1;
            y_range.start = row.y;
            y_range.end = row.y + 1;
            z_range.start = row.z;
            z_range.end = row.z + 1;
        } else {
            x_range.start = min(x_range.start, row.x);
            x_range.end = max(x_range.end, row.x + 1);
            y_range.start = min(y_range.start, row.y);
            y_range.end = max(y_range.end, row.y + 1);
            z_range.start = min(z_range.start, row.z);
            z_range.end = max(z_range.end, row.z + 1);
        }
    }
    part1(&set);
    part2(&set, &x_range, &y_range, &z_range);
}
