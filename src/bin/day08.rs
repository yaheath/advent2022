use std::ops::Range;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

fn visible(grid: &Grid<i8>, tx: i64, ty: i64) -> bool {
    let h = grid.get(tx, ty);
    let Range { start: minx, end: maxx } = grid.x_bounds();
    let Range { start: miny, end: maxy } = grid.y_bounds();

    (minx..tx).all(|x| grid.get(x, ty) < h) ||
    (tx+1..maxx).all(|x| grid.get(x, ty) < h) ||
    (miny..ty).all(|y| grid.get(tx, y) < h) ||
    (ty+1..maxy).all(|y| grid.get(tx, y) < h)
}

fn part1(grid: &Grid<i8>) -> usize {
    let mut value: usize = 0;
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            if visible(grid, x, y) {
                value += 1;
            }
        }
    }
    value
}

fn score(grid: &Grid<i8>, tx: i64, ty: i64) -> usize {
    let h = grid.get(tx, ty);
    let Range { start: minx, end: maxx } = grid.x_bounds();
    let Range { start: miny, end: maxy } = grid.y_bounds();
    if tx == minx || tx == maxx || ty == miny || ty == maxy {
        return 0;
    }
    let mut up = 0usize;
    for y in (miny..ty).rev() {
        up += 1;
        if grid.get(tx, y) >= h {
            break;
        }
    }
    let mut down = 0usize;
    for y in ty+1..maxy {
        down += 1;
        if grid.get(tx, y) >= h {
            break;
        }
    }
    let mut left = 0usize;
    for x in (minx..tx).rev() {
        left += 1;
        if grid.get(x, ty) >= h {
            break;
        }
    }
    let mut right = 0usize;
    for x in tx+1..maxx {
        right += 1;
        if grid.get(x, ty) >= h {
            break;
        }
    }
    up * left * down * right
}

fn part2(grid: &Grid<i8>) -> usize {
    let mut maxv: usize = 0;
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            let s = score(grid, x, y);
            if s > maxv {
                maxv = s;
            }
        }
    }
    maxv
}

fn main() {
    let input = read_input::<String>();
    let grid = Grid::from_input(&input, 0i8, 0, |c| (c as i8) - ('0' as i8));

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<String> = test_input(include_str!("day08.testinput"));
        let grid = Grid::from_input(&input, 0i8, 0, |c| (c as i8) - ('0' as i8));
        assert_eq!(part1(&grid), 21);
        assert_eq!(part2(&grid), 8);
    }
}
