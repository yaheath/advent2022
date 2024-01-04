use std::cmp::{min,max};
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
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

struct Area {
    cubes: HashSet<(i32,i32,i32)>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
}

impl Area {
    fn from_input(input: &Vec<Cube>) -> Self {
        let mut cubes: HashSet<(i32,i32,i32)> = HashSet::new();
        let mut x_range: Range<i32> = Range {start: 0, end: 0};
        let mut y_range: Range<i32> = Range {start: 0, end: 0};
        let mut z_range: Range<i32> = Range {start: 0, end: 0};
        for row in input {
            cubes.insert((row.x, row.y, row.z));
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
        Self { cubes, x_range, y_range, z_range }
    }
}

fn part1(area: &Area) -> usize {
    let mut exposed: usize = 0;
    for (x,y,z) in &area.cubes {
        if !area.cubes.contains(&(*x+1, *y, *z)) { exposed += 1; }
        if !area.cubes.contains(&(*x-1, *y, *z)) { exposed += 1; }
        if !area.cubes.contains(&(*x, *y+1, *z)) { exposed += 1; }
        if !area.cubes.contains(&(*x, *y-1, *z)) { exposed += 1; }
        if !area.cubes.contains(&(*x, *y, *z+1)) { exposed += 1; }
        if !area.cubes.contains(&(*x, *y, *z-1)) { exposed += 1; }
    }
    exposed
}

fn flood_fill(steam: &mut HashSet<(i32,i32,i32)>, area: &Area, coord: &(i32,i32,i32)) {
    steam.insert(coord.clone());

    let mut check = |newcoord: &(i32,i32,i32)| {
        if newcoord.0 < area.x_range.start - 1 ||
            newcoord.0 > area.x_range.end ||
            newcoord.1 < area.y_range.start - 1 ||
            newcoord.1 > area.y_range.end ||
            newcoord.2 < area.z_range.start - 1 ||
            newcoord.2 > area.z_range.end ||
            steam.contains(newcoord) ||
            area.cubes.contains(newcoord) {
                return;
        }
        flood_fill(steam, area, newcoord);
    };

    check(&(coord.0+1, coord.1, coord.2));
    check(&(coord.0-1, coord.1, coord.2));
    check(&(coord.0, coord.1+1, coord.2));
    check(&(coord.0, coord.1-1, coord.2));
    check(&(coord.0, coord.1, coord.2+1));
    check(&(coord.0, coord.1, coord.2-1));
}

fn part2(area: &Area) -> usize {
    let mut steam: HashSet<(i32,i32,i32)> = HashSet::new();
    flood_fill(&mut steam, area, &(area.x_range.end, area.y_range.end, area.z_range.end));
    let mut exposed: usize = 0;
    for (x,y,z) in &area.cubes {
        if steam.contains(&(*x+1, *y, *z)) { exposed += 1; }
        if steam.contains(&(*x-1, *y, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y+1, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y-1, *z)) { exposed += 1; }
        if steam.contains(&(*x, *y, *z+1)) { exposed += 1; }
        if steam.contains(&(*x, *y, *z-1)) { exposed += 1; }
    }
    /*
    for z in (area.z_range.start - 1)..=area.z_range.end {
        for y in (area.y_range.start - 1)..=area.y_range.end {
            for x in (area.x_range.start - 1)..=area.x_range.end {
                let issteam = steam.contains(&(x,y,z));
                let islava = area.cubes.contains(&(x,y,z));
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
    exposed
}

fn main() {
    let input: Vec<Cube> = read_input::<Cube>();
    let area = Area::from_input(&input);
    println!("Part 1: {}", part1(&area));
    println!("Part 2: {}", part2(&area));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Cube> = test_input(include_str!("day18.testinput"));
        let area = Area::from_input(&input);
        assert_eq!(part1(&area), 64);
        assert_eq!(part2(&area), 58);
    }
}
