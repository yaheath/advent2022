use std::cmp::{min,max};
use std::collections::HashSet;
use std::ops::Range;
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord3D;

struct Area {
    cubes: HashSet<Coord3D>,
    x_range: Range<i64>,
    y_range: Range<i64>,
    z_range: Range<i64>,
}

impl Area {
    fn from_input(input: &Vec<Coord3D>) -> Self {
        let mut cubes: HashSet<Coord3D> = HashSet::new();
        let mut x_range: Range<i64> = Range {start: 0, end: 0};
        let mut y_range: Range<i64> = Range {start: 0, end: 0};
        let mut z_range: Range<i64> = Range {start: 0, end: 0};
        for row in input {
            cubes.insert(*row);
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
    area.cubes.iter()
        .map(|loc| loc.neighbors6())
        .flatten()
        .filter(|loc| !area.cubes.contains(loc))
        .count()
}

fn flood_fill(steam: &mut HashSet<Coord3D>, area: &Area, coord: &Coord3D) {
    steam.insert(coord.clone());

    coord.neighbors6().iter()
        .filter(|c| !area.cubes.contains(c))
        .filter(|c|
            c.x >= area.x_range.start - 1 &&
            c.x <= area.x_range.end &&
            c.y >= area.y_range.start - 1 &&
            c.y <= area.y_range.end &&
            c.z >= area.z_range.start - 1 &&
            c.z <= area.z_range.end)
        .for_each(|c| {
            if !steam.contains(c) {
                flood_fill(steam, area, c);
            }
        });
}

fn part2(area: &Area) -> usize {
    let mut steam: HashSet<Coord3D> = HashSet::new();
    flood_fill(&mut steam, area, &Coord3D::new(area.x_range.end, area.y_range.end, area.z_range.end));
    area.cubes.iter()
        .map(|loc| loc.neighbors6())
        .flatten()
        .filter(|loc| steam.contains(loc))
        .count()
}

fn main() {
    let input: Vec<Coord3D> = read_input();
    let area = Area::from_input(&input);
    println!("Part 1: {}", part1(&area));
    println!("Part 2: {}", part2(&area));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day18_test() {
        let input: Vec<Coord3D> = test_input(include_str!("day18.testinput"));
        let area = Area::from_input(&input);
        assert_eq!(part1(&area), 64);
        assert_eq!(part2(&area), 58);
    }
}
