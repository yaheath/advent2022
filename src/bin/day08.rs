use std::ops::Range;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

fn visible(grid: &Grid<i8>, tx: i64, ty: i64) -> bool {
    let h = grid.get(tx, ty);
    let Range { start: minx, end: maxx } = grid.x_bounds();
    let Range { start: miny, end: maxy } = grid.y_bounds();
    let mut blocked = false;
    for x in minx..tx {
        if grid.get(x, ty) >= h {
            blocked = true;
            break;
        }
    }
    if !blocked { return true; }
    blocked = false;
    for x in (tx+1..maxx).rev() {
        if grid.get(x, ty) >= h {
            blocked = true;
            break;
        }
    }
    if !blocked { return true; }
    blocked = false;
    for y in miny..ty {
        if grid.get(tx, y) >= h {
            blocked = true;
            break;
        }
    }
    if !blocked { return true; }
    blocked = false;
    for y in (ty+1..maxy).rev() {
        if grid.get(tx, y) >= h {
            blocked = true;
            break;
        }
    }
    return !blocked;
}

fn part1(grid: &Grid<i8>) {
    let mut value: usize = 0;
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            if visible(grid, x, y) {
                value += 1;
            }
        }
    }
    println!("Part 1: {}", value);
}

fn score(grid: &Grid<i8>, tx: i64, ty: i64) -> i64 {
    let h = grid.get(tx, ty);
    let Range { start: minx, end: maxx } = grid.x_bounds();
    let Range { start: miny, end: maxy } = grid.y_bounds();
    if tx == minx || tx == maxx || ty == miny || ty == maxy {
        return 0;
    }
    let mut up = 0i64;
    for y in (miny..ty).rev() {
        up += 1;
        if grid.get(tx, y) >= h {
            break;
        }
    }
    let mut down = 0i64;
    for y in ty+1..maxy {
        down += 1;
        if grid.get(tx, y) >= h {
            break;
        }
    }
    let mut left = 0i64;
    for x in (minx..tx).rev() {
        left += 1;
        if grid.get(x, ty) >= h {
            break;
        }
    }
    let mut right = 0i64;
    for x in tx+1..maxx {
        right += 1;
        if grid.get(x, ty) >= h {
            break;
        }
    }
    up * left * down * right
}

fn part2(grid: &Grid<i8>) {
    let mut maxv: i64 = 0;
    for y in grid.y_bounds() {
        for x in grid.x_bounds() {
            let s = score(grid, x, y);
            if s > maxv {
                maxv = s;
            }
        }
    }
    println!("Part 2: {}", maxv);
}

fn main() {
    let data = read_input::<String>();
    let grid = Grid::from_input(&data, 0i8, 0, |c| (c as i8) - ('0' as i8));

    part1(&grid);
    part2(&grid);
}
