#[macro_use] extern crate lazy_static;
use std::collections::VecDeque;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;

enum Input {
    Monkey(usize),
    StartItems(Vec<i64>),
    OpAdd(i64),
    OpMult(i64),
    OpSquare,
    Divisible(i64),
    TrueTarget(usize),
    FalseTarget(usize),
    None,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MRE: Regex = Regex::new(r"^Monkey (\d+)").unwrap();
        }
        lazy_static! {
            static ref SRE: Regex = Regex::new(r"Starting items: ([\d, ]+)").unwrap();
        }
        lazy_static! {
            static ref SPLIT: Regex = Regex::new(r"[, ]+").unwrap();
        }
        lazy_static! {
            static ref ORE: Regex = Regex::new(r"Operation: new = old (.) (old|\d+)").unwrap();
        }
        lazy_static! {
            static ref DRE: Regex = Regex::new(r"divisible by (\d+)").unwrap();
        }
        lazy_static! {
            static ref TFRE: Regex = Regex::new(r"(true|false):.* monkey (\d+)").unwrap();
        }
        if let Some(caps) = MRE.captures(s) {
            Ok(Input::Monkey(caps.get(1).unwrap().as_str().parse::<usize>().unwrap()))
        }
        else if let Some(caps) = SRE.captures(s) {
            let list = caps.get(1).unwrap().as_str();
            let items: Vec<i64> = SPLIT.split(list).map(|s| s.parse::<i64>().unwrap()).collect();
            Ok(Input::StartItems(items))
        }
        else if let Some(caps) = ORE.captures(s) {
            let op = caps.get(1).unwrap().as_str().chars().next().unwrap();
            let arg = caps.get(2).unwrap().as_str();
            match (op, arg) {
                ('*', "old") => Ok(Input::OpSquare),
                ('*', num) => Ok(Input::OpMult(num.parse::<i64>().unwrap())),
                ('+', num) => Ok(Input::OpAdd(num.parse::<i64>().unwrap())),
                (_, _) => Err("Invalid operation".into()),
            }
        }
        else if let Some(caps) = DRE.captures(s) {
            Ok(Input::Divisible(caps.get(1).unwrap().as_str().parse::<i64>().unwrap()))
        }
        else if let Some(caps) = TFRE.captures(s) {
            let val = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            match caps.get(1).unwrap().as_str() {
                "true" => Ok(Input::TrueTarget(val)),
                "false" => Ok(Input::FalseTarget(val)),
                _ => Err("Invalid target expression".into()),
            }
        }
        else {
            Ok(Input::None)
        }
    }
}

enum Operation {
    Add(i64),
    Mult(i64),
    Square,
}

struct Monkey {
    items: VecDeque<i64>,
    op: Operation,
    divisible: i64,
    true_target: usize,
    false_target: usize,
    inspections: i64,
}

fn make_monkeys(input: &Vec<Input>) -> Vec<Monkey> {
    let mut monkeys:Vec<Monkey> = Vec::new();
    let mut op:Operation = Operation::Square;
    let mut div:i64 = 0;
    let mut true_target:usize = 0;
    let mut items:VecDeque<i64> = VecDeque::new();

    for row in input {
        match row {
            Input::OpSquare => {op = Operation::Square;},
            Input::OpAdd(n) => {op = Operation::Add(*n);},
            Input::OpMult(n) => {op = Operation::Mult(*n);},
            Input::StartItems(v) => {items = VecDeque::from_iter(v.iter().cloned());},
            Input::Divisible(n) => {div = *n;},
            Input::TrueTarget(n) => {true_target = *n;},
            Input::FalseTarget(n) => {
                let false_target = *n;
                monkeys.push(Monkey{
                    items: items,
                    op: op,
                    divisible: div,
                    true_target: true_target,
                    false_target: false_target,
                    inspections: 0,
                });
                items = VecDeque::new();
                op = Operation::Square;
            },
            Input::Monkey(_) => {},
            Input::None => {},
        }
    }

    monkeys
}

fn monkey_step(id: usize, modulo: i64, monkeys: &mut Vec<Monkey>) -> bool {
    if monkeys[id].items.len() == 0 { return false; }
    let mut item = monkeys.get_mut(id).unwrap().items.pop_front().unwrap();
    monkeys.get_mut(id).unwrap().inspections += 1;
    match monkeys[id].op {
        Operation::Add(n) => { item += n; },
        Operation::Mult(n) => { item *= n; },
        Operation::Square => { item *= item; },
    }
    if modulo == 0 {
        item /= 3;
    } else {
        item %= modulo;
    }
    let target = if item % monkeys[id].divisible == 0 {
        monkeys[id].true_target
    } else {
        monkeys[id].false_target
    };
    monkeys.get_mut(target).unwrap().items.push_back(item);
    monkeys[id].items.len() > 0
}

fn bothparts(input: &Vec<Input>, part2: bool) -> i64 {
    let mut monkeys = make_monkeys(input);
    let (steps, modulo) = if part2 {
        (10000, monkeys.iter().map(|m| m.divisible).product())
    }
    else {
        (20, 0)
    };
    for _ in 0..steps {
        for id in 0..monkeys.len() {
            while monkey_step(id, modulo, &mut monkeys) {
            }
        }
    }
    let mut insp:Vec<i64> = monkeys.iter().map(|m| m.inspections).collect();
    insp.sort_unstable_by(|a,b| b.cmp(a));
    insp[0] * insp[1]
}

fn main() {
    let input: Vec<Input> = read_input();
    println!("Part 1: {}", bothparts(&input, false));
    println!("Part 2: {}", bothparts(&input, true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day11_test() {
        let input: Vec<Input> = test_input(include_str!("day11.testinput"));
        assert_eq!(bothparts(&input, false), 10605);
        assert_eq!(bothparts(&input, true), 2713310158);
    }
}
