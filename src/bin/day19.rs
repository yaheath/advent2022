use std::ops::{Add, Mul};
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

#[derive(Debug, Clone, Copy, Default)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Resources {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
        })
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
        }
    }
}

const ONE_ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};
const ONE_CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};
const ONE_OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};

struct Blueprint {
    ore_bot: Resources,
    clay_bot: Resources,
    obsidian_bot: Resources,
    geode_bot: Resources,
}

impl FromStr for Blueprint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"s (\d+) ore.*s (\d+) ore.*s (\d+) ore and (\d+) c.*s (\d+) ore and (\d+) o").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(Blueprint {
                ore_bot: Resources {
                    ore: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                clay_bot: Resources {
                    ore: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    clay: 0,
                    obsidian: 0,
                },
                obsidian_bot: Resources {
                    ore: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                    clay: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                    obsidian: 0,
                },
                geode_bot: Resources {
                    ore: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                    obsidian: caps.get(6).unwrap().as_str().parse::<usize>().unwrap(),
                    clay: 0,
                },
            })
        }
        else {
            Err("invalid input line".into())
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_remaining: usize,
    geodes: usize,
    resources: Resources,
    production: Resources,
}

impl State {
    fn new(minutes_remaining: usize) -> Self {
        Self {
            minutes_remaining,
            geodes: 0,
            resources: Default::default(),
            production: ONE_ORE,
        }
    }

    fn choose_robot(&self, cost: Resources, robot: Resources) -> Option<Self> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.production * minutes_passed;
                resources.checked_sub(cost).map(|resources| Self {
                    minutes_remaining,
                    resources: resources + self.production,
                    production: self.production + robot,
                    geodes: self.geodes,
                })
            },
        )
    }

    fn branch(&self, blueprint: &Blueprint) -> impl Iterator<Item = Self> {
        let max_ore_cost = blueprint.clay_bot.ore
            .max(blueprint.obsidian_bot.ore)
            .max(blueprint.geode_bot.ore);
        let ore_robot_viable = self.production.ore < max_ore_cost;
        let clay_robot_viable = self.production.clay < blueprint.obsidian_bot.clay;
        let obsidian_robot_viable = self.production.obsidian
            < blueprint.geode_bot.obsidian
            && self.production.clay > 0;
        let geode_robot_viable = self.production.obsidian > 0;
        [
            ore_robot_viable.then(|| self.choose_robot(blueprint.ore_bot, ONE_ORE)),
            clay_robot_viable.then(|| self.choose_robot(blueprint.clay_bot, ONE_CLAY)),
            obsidian_robot_viable
                .then(|| self.choose_robot(blueprint.obsidian_bot, ONE_OBSIDIAN)),
            geode_robot_viable.then(|| {
                self.choose_robot(blueprint.geode_bot, Default::default())
                    .map(|state| Self {
                        geodes: state.geodes + state.minutes_remaining,
                        ..state
                    })
            }),
        ]
        .into_iter().flatten().flatten()
    }

    fn bound(self, blueprint: &Blueprint) -> usize {
        let geode_cost = blueprint.geode_bot.obsidian;
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.production.obsidian,
                self.geodes,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= geode_cost {
                    (
                        obsidian + rate - geode_cost,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    // pretend we can build an obsidian bot every turn
                    // that we're not building a geode bot
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }
}

fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut usize) {
    *best = state.geodes.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
    }
}

fn search(bp: &Blueprint, timelimit: usize) -> usize {
    let mut best = 0;
    branch_and_bound(bp, State::new(timelimit), &mut best);
    best
}

fn part1(input: &Vec<Blueprint>) -> usize {
    input.iter().enumerate()
        .map(|(idx, bp)| (idx + 1) * search(bp, 24))
        .sum()
}

fn part2(input: &Vec<Blueprint>) -> usize {
    input.iter().take(3)
        .map(|bp| search(bp, 32))
        .product()
}

fn main() {
    let input: Vec<Blueprint> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day19_test() {
        let input: Vec<Blueprint> = test_input(include_str!("day19.testinput"));
        assert_eq!(part1(&input), 33);
        assert_eq!(part2(&input), 56 * 62);
    }
}
