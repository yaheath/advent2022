use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Ordering;
use std::vec::Vec;
use ya_advent_lib::coords::{CDir, Coord2D};
use ya_advent_lib::read::read_input;

type Step = i64;
type Leg = i64;

struct Blizzard {
    initial_pos: Coord2D,
    dir: CDir,
}

impl Blizzard {
    fn pos_at(&self, step: Step, width: i64, height: i64) -> Coord2D {
        match self.dir {
            CDir::N => Coord2D::new(self.initial_pos.x, (self.initial_pos.y - step).rem_euclid(height)),
            CDir::S => Coord2D::new(self.initial_pos.x, (self.initial_pos.y + step).rem_euclid(height)),
            CDir::E => Coord2D::new((self.initial_pos.x + step).rem_euclid(width), self.initial_pos.y),
            CDir::W => Coord2D::new((self.initial_pos.x - step).rem_euclid(width), self.initial_pos.y),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    pos: Coord2D,
    dist: i64,
    step: Step,
    leg: Leg,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.dist.cmp(&self.dist)
            .then_with(|| other.step.cmp(&self.step))
            .then_with(|| other.pos.cmp(&self.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Valley {
    width: i64,
    height: i64,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    fn from_input(input: Vec<String>) -> Self {
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
                    '^' => CDir::N,
                    '>' => CDir::E,
                    'v' => CDir::S,
                    '<' => CDir::W,
                    _ => panic!(),
                };
                blizzards.push(Blizzard {
                    initial_pos: Coord2D::new(x as i64, height),
                    dir,
                });
            }
            height += 1;
        }
        Self {
            blizzards,
            height,
            width,
        }
    }

    fn search(&self, nlegs: Leg) -> i64 {
        let entrance = Coord2D::new(0, -1);
        let exit = Coord2D::new(self.width - 1, self.height);
        let mut byrow: HashMap<i64, Vec<usize>> = HashMap::from_iter(
            (0..self.height).map(|y| (y, Vec::new()))
        );
        let mut bycol: HashMap<i64, Vec<usize>> = HashMap::from_iter(
            (0..self.width).map(|x| (x, Vec::new()))
        );
        for (idx, b) in self.blizzards.iter().enumerate() {
            match b.dir {
                CDir::E | CDir::W => {
                    byrow.get_mut(&b.initial_pos.y).unwrap().push(idx);
                },
                CDir::N | CDir::S => {
                    bycol.get_mut(&b.initial_pos.x).unwrap().push(idx);
                },
            }
        }

        //let mut came_from: HashMap<(Coord,Step),(Coord,Step)> = HashMap::new();
        let mut gscore: HashMap<(Coord2D, Step),i64> = HashMap::new();
        gscore.insert((entrance, 0), 0);

        let mut leg: Leg = 0;
        let mut heap: BinaryHeap<State> = BinaryHeap::new();
        heap.push(State { pos: entrance, dist: &entrance.mdist_to(&exit) * nlegs, step: 0, leg: 0 });
        while let Some(state) = heap.pop() {
            //println!("state {:?}", state);
            if state.leg < leg { continue; }
            if state.pos == exit && leg & 1 == 0 {
                //println!("at exit, leg={}, step={}", leg, state.step);
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
                //println!("at entrance, leg={} step={}", leg, state.step);
                leg += 1;
            }
            let row: HashSet<i64> = if state.pos.y < 0 || state.pos.y == self.height {
                    HashSet::new()
                } else {
                    byrow[&state.pos.y].iter()
                        .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).x).collect()
                };
            let n_row: HashSet<i64> = if state.pos.y < 1 {
                    HashSet::new()
                } else {
                    byrow[&(state.pos.y - 1)].iter()
                        .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).x).collect()
                };
            let s_row: HashSet<i64> = if state.pos.y + 1 >= self.height {
                    HashSet::new()
                } else {
                    byrow[&(state.pos.y + 1)].iter()
                        .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).x).collect()
                };
            let col: HashSet<i64> = bycol[&state.pos.x].iter()
                .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).y).collect();
            let w_col: HashSet<i64> = if state.pos.x < 1 {
                    HashSet::new()
                } else {
                    bycol[&(state.pos.x - 1)].iter()
                    .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).y).collect()
                };
            let e_col: HashSet<i64> = if state.pos.x + 1 >= self.width {
                    HashSet::new()
                } else {
                    bycol[&(state.pos.x + 1)].iter()
                    .map(|idx| self.blizzards[*idx].pos_at(state.step + 1, self.width, self.height).y).collect()
                };

            let mut add = |c: Coord2D, incr: i64| {
                if c.x < 0 || c.y < -1 || (c.y == -1 && c.x != entrance.x)
                        || c.x >= self.width || c.y > self.height || (c.y == self.height && c.x != exit.x) {
                    return;
                }
                let new_gscore = gscore[&(state.pos, state.step)] + incr;
                //println!("add {:?} new_gscore={}", c, new_gscore);
                if !gscore.contains_key(&(c,state.step+1)) || new_gscore < gscore[&(c,state.step+1)] {
                    gscore.insert((c, state.step + 1), new_gscore);
                    //came_from.insert((c, state.step + 1), (state.pos, state.step));
                    heap.push(State {
                        pos: c,
                        dist: new_gscore + c.mdist_to(if leg & 1 == 1 { &entrance } else { &exit }) + (nlegs - 1 - leg) * entrance.mdist_to(&exit),
                        step: state.step + 1,
                        leg,
                    });
                }
            };
            if !col.contains(&(state.pos.y - 1)) && !n_row.contains(&state.pos.x) {
                add(Coord2D::new(state.pos.x, state.pos.y - 1), 1);
            }
            if !col.contains(&(state.pos.y + 1)) && !s_row.contains(&state.pos.x) {
                add(Coord2D::new(state.pos.x, state.pos.y + 1), 1);
            }
            if !row.contains(&(state.pos.x + 1)) && !e_col.contains(&state.pos.y) {
                add(Coord2D::new(state.pos.x + 1, state.pos.y), 1);
            }
            if !row.contains(&(state.pos.x - 1)) && !w_col.contains(&state.pos.y) {
                add(Coord2D::new(state.pos.x - 1, state.pos.y), 1);
            }
            if !row.contains(&state.pos.x) && !col.contains(&state.pos.y) {
                add(state.pos, 1);
            }
        }
        panic!("no path found");
    }
}

fn main() {
    let input: Vec<String> = read_input();
    let valley = Valley::from_input(input);

    let val = valley.search(1);
    println!("Part 1: {val}");
    let val = valley.search(3);
    println!("Part 2: {val}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day24_test() {
        let input: Vec<String> = test_input(include_str!("day24.testinput"));
        let valley = Valley::from_input(input);
        assert_eq!(valley.search(1), 18);
        assert_eq!(valley.search(3), 54);
    }
}
