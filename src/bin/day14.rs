use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::infinite_grid::InfiniteGrid;

struct CoordList {
    list: Vec<(i64,i64)>
}

impl FromStr for CoordList {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<(i64, i64)> = s.split(" -> ")
            .map(|c| {
                let nn:Vec<i64> = c.split(",").map(|cc| cc.parse::<i64>().unwrap()).collect();
                (nn[0], nn[1])
            })
            .collect();
        Ok(CoordList {list: v})
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

impl Cell {
    fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

fn make_grid(input: &Vec<CoordList>) -> InfiniteGrid<Cell> {
    let mut grid = InfiniteGrid::new(Cell::Empty);

    for row in input {
        for idx in 0..(row.list.len() - 1) {
            let (x1, y1) = row.list[idx];
            let (x2, y2) = row.list[idx + 1];
            if y1 == y2 {
                for x in if x1 < x2 { x1..=x2 } else { x2..=x1 } {
                    grid.set(x, y1, Cell::Rock);
                }
            }
            else {
                for y in if y1 < y2 { y1..=y2 } else { y2..=y1 } {
                    grid.set(x1, y, Cell::Rock);
                }
            }
        }
    }
    grid
}

/*
fn printgrid(grid: &InfiniteGrid<Cell>) {
    grid.print(|c| match c {
        Cell::Rock => '#',
        Cell::Sand => 'o',
        _ => '.',
    });
}
*/

fn placesand(grid: &mut InfiniteGrid<Cell>, start_x: i64, start_y: i64, floor: i64) -> bool {
    let mut x = start_x;
    let mut y = start_y;

    if !grid.get(x, y).is_empty() {
        return false
    }

    while floor > 0 && y < floor || floor == 0 && y < grid.y_bounds().end - 1 {
        if grid.get(x, y + 1).is_empty() {
            y += 1;
            continue;
        }
        if grid.get(x - 1, y + 1).is_empty() {
            x -= 1;
            y += 1;
            continue;
        }
        if grid.get(x + 1, y + 1).is_empty() {
            x += 1;
            y += 1;
            continue;
        }
        grid.set(x, y, Cell::Sand);
        return true;
    }
    if floor > 0 {
        grid.set(x, y, Cell::Sand);
        return true;
    }
    false
}

fn part1(input: &Vec<CoordList>) {
    let mut grid = make_grid(&input);
    let mut count: usize = 0;
    while placesand(&mut grid, 500, 0, 0) {
        count += 1;
    }
    //printgrid(&grid);
    println!("Part 1: {}", count);
}

fn part2(input: &Vec<CoordList>) {
    let mut grid = make_grid(&input);
    let mut count: usize = 0;
    let floor = grid.y_bounds().end;
    while placesand(&mut grid, 500, 0, floor) {
        count += 1;
    }
    //printgrid(&grid);
    println!("Part 2: {}", count);
}

fn main() {
    let input: Vec<CoordList> = read_input::<CoordList>();
    part1(&input);
    part2(&input);
}
