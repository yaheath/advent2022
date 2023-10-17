#[macro_use] extern crate lazy_static;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, BTreeSet, HashMap, VecDeque};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use regex::Regex;
use advent_lib::read::read_input;

type ValveId = [char;2];
type Flow = i32;
type Minute = i32;

fn str_to_valveid(s: &str) -> ValveId {
    let c:Vec<char> = s.chars().collect();
    [c[0], c[1]]
}

#[derive(Clone)]
struct Valve {
    name: ValveId,
    rate: Flow,
    neighbors: Vec<ValveId>,
}

impl FromStr for Valve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve (\w+).*rate=(\d+).*valves? (.*)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Valve {
                name: str_to_valveid(caps.get(1).unwrap().as_str()),
                rate: caps.get(2).unwrap().as_str().parse::<Flow>().unwrap(),
                neighbors: caps.get(3).unwrap().as_str().split(", ").map(|s| str_to_valveid(s)).collect(),
            })
        }
        else {
            Err("Invalid input line".into())
        }
    }
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    minutes_remaining: Minute,
    location: ValveId,
    opened: BTreeSet<ValveId>,
    total_release: Flow,
}

impl State {
    fn new(minutes_remaining: Minute) -> Self {
        Self {
            minutes_remaining,
            location: ['A';2],
            opened: BTreeSet::new(),
            total_release: 0,
        }
    }

    fn branch<'a>(&'a self, valves: &'a HashMap<ValveId, Valve>, dxmap: &'a HashMap<(ValveId,ValveId), Minute>) -> impl Iterator<Item = Self> + 'a {
        valves.iter()
            .filter(|(_,v)| v.rate > 0)
            .filter(|(k,_)| !self.opened.contains(*k))
            .map(|entry| (entry, getdx(self.location, *entry.0, dxmap)))
            .filter(|(_, d)| *d < self.minutes_remaining)
            .map(|((k,v), d)| {
                let mut opened = self.opened.clone();
                opened.insert(*k);
                Self {
                    minutes_remaining: self.minutes_remaining - d - 1,
                    location: *k,
                    opened,
                    total_release: self.total_release + v.rate * (self.minutes_remaining - d - 1),
                }
            })
    }

}

fn getdx(loc1: ValveId, loc2: ValveId, dxmap: &HashMap<(ValveId,ValveId), Minute>) -> Minute {
    if dxmap.contains_key(&(loc1,loc2)) {
        dxmap[&(loc1,loc2)]
    } else {
        dxmap[&(loc2,loc1)]
    }
}

fn dx_between(loc1: &ValveId, loc2: &ValveId, valves: &HashMap<ValveId,Valve>) -> Minute {
    let mut heap: BinaryHeap<(Reverse<Minute>,&ValveId)> = BinaryHeap::new();
    let mut dists: HashMap<&ValveId,Minute> = HashMap::new();
    heap.push((Reverse(0), loc1));
    dists.insert(loc1, 0);
    while let Some((Reverse(dx), loc)) = heap.pop() {
        if loc == loc2 {
            return dx;
        }
        for n in valves[loc].neighbors.iter() {
            let d = dx + 1;
            if !dists.contains_key(&n) || d < dists[&n] {
                heap.push((Reverse(d), n));
                dists.insert(n, d);
            }
        }
    }
    panic!("no route found");
}

fn search(valves: &HashMap<ValveId,Valve>, dxmap: &HashMap<(ValveId,ValveId), Minute>, minutes: Minute) -> HashMap<BTreeSet<ValveId>,Flow> {

    let mut max_released: HashMap<BTreeSet<ValveId>,Flow> = HashMap::new();

    let mut q = VecDeque::new();
    q.push_back(State::new(minutes));
    while let Some(state) = q.pop_front() {
        max_released
            .entry(state.opened.clone())
            .and_modify(|val| *val = state.total_release.max(*val))
            .or_insert(state.total_release);

        for newstate in state.branch(valves, &dxmap) {
            q.push_back(newstate);
        }
    }
    max_released
}

fn part1(input: &HashMap<ValveId,Valve>, dxmap: &HashMap<(ValveId,ValveId), Minute>) {
    let states = search(input, dxmap, 30);
    let value = states.iter().map(|(_,v)| v).max().unwrap();
    println!("Part 1: {}", value);
}

fn part2(input: &HashMap<ValveId,Valve>, dxmap: &HashMap<(ValveId,ValveId), Minute>) {
    let states = search(input, dxmap, 26);
    let value = states
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();
    println!("Part 2: {}", value);
}

fn main() {
    let input: Vec<Valve> = read_input::<Valve>();
    let valves: HashMap<ValveId, Valve> = input.iter().map(|r| (r.name.clone(), r.clone())).collect();
    let dxmap: HashMap<(ValveId,ValveId), Minute> =
        valves
        .iter()
        .filter(|(k,v)| v.rate > 0 || **k == ['A';2])
        .map(|(k,_)| k)
        .tuple_combinations()
        .map(|(a, b)| ((*a,*b), dx_between(a, b, &valves)))
        .collect();
    part1(&valves, &dxmap);
    part2(&valves, &dxmap);
}
