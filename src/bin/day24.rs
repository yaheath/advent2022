use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

type Coord = (i64, i64);
type Step = i64;
type Leg = i64;

struct Blizzard {
    initial_pos: Coord,
    dir: Dir,
}

impl Blizzard {
    fn pos_at(&self, step: Step, width: i64, height: i64) -> Coord {
        match self.dir {
            Dir::North => (self.initial_pos.0, (self.initial_pos.1 - step).rem_euclid(height)),
            Dir::South => (self.initial_pos.0, (self.initial_pos.1 + step).rem_euclid(height)),
            Dir::East => ((self.initial_pos.0 + step).rem_euclid(width), self.initial_pos.1),
            Dir::West => ((self.initial_pos.0 - step).rem_euclid(width), self.initial_pos.1),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    pos: Coord,
    dist: i64,
    step: Step,
    leg: Leg,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.step.cmp(&self.step))
            .then_with(|| other.pos.1.cmp(&self.pos.1))
            .then_with(|| other.pos.0.cmp(&self.pos.0))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn md(pos1: &Coord, pos2: &Coord) -> i64 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

fn search(blizzards: &Vec<Blizzard>, width: i64, height: i64, nlegs: Leg) -> i64 {
    let entrance:Coord = (0i64, -1i64);
    let exit:Coord = (width - 1, height);
    let mut byrow: HashMap<i64, Vec<usize>> = HashMap::from_iter(
        (0..height).map(|y| (y, Vec::new()))
    );
    let mut bycol: HashMap<i64, Vec<usize>> = HashMap::from_iter(
        (0..width).map(|x| (x, Vec::new()))
    );
    for (idx, b) in blizzards.iter().enumerate() {
        match b.dir {
            Dir::East | Dir::West => {
                byrow.get_mut(&b.initial_pos.1).unwrap().push(idx);
            },
            Dir::North | Dir::South => {
                bycol.get_mut(&b.initial_pos.0).unwrap().push(idx);
            },
        }
    }

    //let mut came_from: HashMap<(Coord,Step),(Coord,Step)> = HashMap::new();
    let mut gscore: HashMap<(Coord,Step),i64> = HashMap::new();
    gscore.insert((entrance, 0), 0);

    let mut leg: Leg = 0;
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State { pos: entrance, dist: md(&entrance, &exit) * nlegs, step: 0, leg: 0 });
    while let Some(state) = heap.pop() {
        //println!("state {:?}", state);
        if state.leg < leg { continue; }
        if state.pos == exit && leg & 1 == 0 {
            println!("at exit, leg={}, step={}", leg, state.step);
            leg += 1;
            if leg >= nlegs {
                /*
                let mut pos = state.pos;
                let mut step = state.step;
                loop {
                    if let Some(next) = came_from.get(&(pos, step)) {
                        println!("{:?} {}", next.0, next.1);
                        pos = next.0;
                        step = next.1;
                    } else {
                        break;
                    }
                }
                */
                return state.step;
            }
        }
        if state.pos == entrance && leg & 1 == 1 {
            println!("at entrance, leg={} step={}", leg, state.step);
            leg += 1;
        }
        let row: HashSet<i64> = if state.pos.1 < 0 || state.pos.1 == height {
                HashSet::new()
            } else {
                byrow[&state.pos.1].iter()
                    .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).0).collect()
            };
        let n_row: HashSet<i64> = if state.pos.1 < 1 {
                HashSet::new()
            } else {
                byrow[&(state.pos.1 - 1)].iter()
                    .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).0).collect()
            };
        let s_row: HashSet<i64> = if state.pos.1 + 1 >= height {
                HashSet::new()
            } else {
                byrow[&(state.pos.1 + 1)].iter()
                    .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).0).collect()
            };
        let col: HashSet<i64> = bycol[&state.pos.0].iter()
            .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).1).collect();
        let w_col: HashSet<i64> = if state.pos.0 < 1 {
                HashSet::new()
            } else {
                bycol[&(state.pos.0 - 1)].iter()
                .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).1).collect()
            };
        let e_col: HashSet<i64> = if state.pos.0 + 1 >= width {
                HashSet::new()
            } else {
                bycol[&(state.pos.0 + 1)].iter()
                .map(|idx| blizzards[*idx].pos_at(state.step + 1, width, height).1).collect()
            };

        let mut add = |c: Coord, incr: i64| {
            if c.0 < 0 || c.1 < -1 || (c.1 == -1 && c.0 != entrance.0)
                    || c.0 >= width || c.1 > height || (c.1 == height && c.0 != exit.0) {
                return;
            }
            let new_gscore = gscore[&(state.pos, state.step)] + incr;
            //println!("add {:?} new_gscore={}", c, new_gscore);
            if !gscore.contains_key(&(c,state.step+1)) || new_gscore < gscore[&(c,state.step+1)] {
                gscore.insert((c, state.step + 1), new_gscore);
                //came_from.insert((c, state.step + 1), (state.pos, state.step));
                heap.push(State {
                    pos: c,
                    dist: new_gscore + md(&c, if leg & 1 == 1 { &entrance } else { &exit }) + (nlegs - 1 - leg) * md(&entrance, &exit),
                    step: state.step + 1,
                    leg: leg,
                });
            }
        };
        if !col.contains(&(state.pos.1 - 1)) && !n_row.contains(&state.pos.0) { add((state.pos.0, state.pos.1 - 1), 1); }
        if !col.contains(&(state.pos.1 + 1)) && !s_row.contains(&state.pos.0) { add((state.pos.0, state.pos.1 + 1), 1); }
        if !row.contains(&(state.pos.0 + 1)) && !e_col.contains(&state.pos.1) { add((state.pos.0 + 1, state.pos.1), 1); }
        if !row.contains(&(state.pos.0 - 1)) && !w_col.contains(&state.pos.1) { add((state.pos.0 - 1, state.pos.1), 1); }
        if !row.contains(&state.pos.0) && !col.contains(&state.pos.1) { add(state.pos, 1); }
    }
    panic!("no path found");
}

fn main() {
    let input: Vec<String> = read_input();
    let mut height = 0i64;
    let width:i64 = input[0].len() as i64 - 2;
    let mut blizzards: Vec<Blizzard> = Vec::new();
    for line in input.iter().skip(1) {
        if line.contains("######") {
            continue;
        }
        for (x, c) in line.chars().skip(1).enumerate() {
            match c {
                '.'|'#' => { continue; },
                _ => {},
            }
            let dir = match c {
                '^' => Dir::North,
                '>' => Dir::East,
                'v' => Dir::South,
                '<' => Dir::West,
                _ => panic!(),
            };
            blizzards.push(Blizzard {
                initial_pos: (x as i64, height),
                dir: dir,
            });
        }
        height += 1;
    }
    let val = search(&blizzards, width, height, 1);
    println!("Part 1: {}", val);
    let val2 = search(&blizzards, width, height, 3);
    println!("Part 2: {}", val2);
}
