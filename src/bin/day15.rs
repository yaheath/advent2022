#[macro_use] extern crate lazy_static;
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_input;
use advent_lib::range::merge_ranges;

struct Sensor {
    loc: (i64,i64),
    beacon: (i64,i64),
}
impl Sensor {
    fn dx_to_beacon(&self) -> i64 {
        (self.loc.0 - self.beacon.0).abs() + (self.loc.1 - self.beacon.1).abs()
    }
}

impl FromStr for Sensor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"x=([-\d]+), y=([-\d]+).*x=([-\d]+), y=([-\d]+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Sensor {
                loc: (
                   caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                   caps.get(2).unwrap().as_str().parse::<i64>().unwrap()
                ),
                beacon: (
                   caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                   caps.get(4).unwrap().as_str().parse::<i64>().unwrap()
                ),
            })
        }
        else {
            Err("invalid input row".into())
        }
    }
}

fn bsearch(list: &Vec<i64>, range: &Range<i64>) -> usize {
    let (start_has_val, start_idx) = match list.binary_search(&range.start) {
        Ok(val) => (true, val),
        Err(val) => (false, val),
    };
    let end_idx = match list.binary_search(&range.end) {
        Ok(val) => val,
        Err(val) => val,
    };
    if start_idx == end_idx {
        if start_has_val {
            1usize
        } else {
            0usize
        }
    } else {
        end_idx - start_idx
    }
}

fn searchrow(input: &Vec<Sensor>, tgtrow: i64) -> (Vec<Range<i64>>, Vec<i64>) {
    let mut ranges:Vec<Range<i64>> = Vec::new();
    let mut beacons: HashSet<i64> = HashSet::new();
    for row in input {
        let dx = row.dx_to_beacon();
        if row.beacon.1 == tgtrow {
            beacons.insert(row.beacon.0);
        }
        if row.loc.1 - dx <= tgtrow && row.loc.1 + dx >= tgtrow {
            let xdx = dx - (row.loc.1 - tgtrow).abs();
            ranges.push(Range{ start: row.loc.0 - xdx, end: row.loc.0 + xdx + 1 });
        }
    }
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let merged_ranges: Vec<_> = merge_ranges(ranges).collect();
    let mut sorted_beacons = Vec::from_iter(beacons.iter().cloned());
    sorted_beacons.sort();
    (merged_ranges, sorted_beacons)
}

fn part1(input: &Vec<Sensor>) {
    let (ranges, beacons) = searchrow(input, 2000000i64);
    let mut sum = 0i64;
    for range in ranges {
        let nbeacons = bsearch(&beacons, &range) as i64;
        sum += range.end - range.start - nbeacons;
    }
    println!("Part 1: {}", sum);
}

fn part2(input: &Vec<Sensor>) {
    let mut x:i64 = 0;
    let mut y:i64 = 0;
    for row in 0..=4000000 {
        let (ranges, _) = searchrow(input, row);
        if ranges.len() > 1 {
            //println!("{}, {}", ranges[0].end, row);
            y = row;
            x = ranges[0].end;
            break;
        }
    }
    println!("Part 2: {}", x * 4000000 + y);
}

fn main() {
    let input = read_input::<Sensor>();

    part1(&input);
    part2(&input);
}
