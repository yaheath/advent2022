use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

enum Op {
    Const(i64),
    Add(String,String),
    Sub(String,String),
    Mul(String,String),
    Div(String,String),
}

struct Monkey {
    name: String,
    op: Op,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CRE: Regex = Regex::new(r"^(\w+): (\d+)$").unwrap();
        }
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+): (\w+) (.) (\w+)$").unwrap();
        }
        if let Some(caps) = CRE.captures(s) {
            Ok(Monkey {
                name: caps.get(1).unwrap().as_str().into(),
                op: Op::Const(caps.get(2).unwrap().as_str().parse::<i64>().unwrap()),
            })
        } else if let Some(caps) = RE.captures(s) {
            let name:String = caps.get(1).unwrap().as_str().into();
            let a:String = caps.get(2).unwrap().as_str().into();
            let b:String = caps.get(4).unwrap().as_str().into();
            Ok(Monkey {
                name,
                op: match caps.get(3).unwrap().as_str() {
                    "+" => Op::Add(a, b),
                    "-" => Op::Sub(a, b),
                    "*" => Op::Mul(a, b),
                    "/" => Op::Div(a, b),
                    _ => panic!(),
                },
            })
        }
        else {
            Err("Invalid input line".into())
        }
    }
}

fn resolve(monkey: &str, monkeys: &HashMap<&str,&Monkey>) -> i64 {
    match &monkeys[monkey].op {
        Op::Const(n) => *n,
        Op::Add(a, b) => resolve(a, monkeys) + resolve(b, monkeys),
        Op::Sub(a, b) => resolve(a, monkeys) - resolve(b, monkeys),
        Op::Mul(a, b) => resolve(a, monkeys) * resolve(b, monkeys),
        Op::Div(a, b) => resolve(a, monkeys) / resolve(b, monkeys),
    }
}

fn part1(input: &[Monkey]) -> i64 {
    let monkeys: HashMap<&str,&Monkey> = HashMap::from_iter(
        input.iter().map(|m| (m.name.as_str(), m)));
    resolve("root", &monkeys)
}

enum HOp {
    Human,
    Add(i64,usize),
    Mul(i64,usize),
    Sub(i64,usize),
    SubR(usize,i64),
    Div(i64,usize),
    DivR(usize,i64),
}

enum Res {
    Const(i64),
    Op(usize),
}

fn resolve_h(monkey: &str, monkeys: &HashMap<&str,&Monkey>, ops: &mut Vec<HOp>) -> Res {
    if monkey == "humn" {
        let idx = ops.len();
        ops.push(HOp::Human);
        return Res::Op(idx);
    }
    let res_a: Res;
    let res_b: Res;
    let op = &monkeys[monkey].op;
    match op {
        Op::Const(n) => { return Res::Const(*n); },
        Op::Add(a, b) |
        Op::Sub(a, b) |
        Op::Mul(a, b) |
        Op::Div(a, b) => {
            res_a = resolve_h(a, monkeys, ops);
            res_b = resolve_h(b, monkeys, ops);
        },
    };
    match (res_a, res_b) {
        (Res::Const(a), Res::Const(b)) => {
            Res::Const(
                match op {
                    Op::Add(_,_) => a + b,
                    Op::Sub(_,_) => a - b,
                    Op::Mul(_,_) => a * b,
                    Op::Div(_,_) => a / b,
                    _ => panic!(),
                }
            )
        },
        (Res::Const(a_val), Res::Op(b_idx)) => {
            let hop = match op {
                Op::Add(_,_) => HOp::Add(a_val, b_idx),
                Op::Sub(_,_) => HOp::Sub(a_val, b_idx),
                Op::Mul(_,_) => HOp::Mul(a_val, b_idx),
                Op::Div(_,_) => HOp::Div(a_val, b_idx),
                _ => panic!(),
            };
            let idx = ops.len();
            ops.push(hop);
            Res::Op(idx)
        },
        (Res::Op(a_idx), Res::Const(b_val)) => {
            let hop = match op {
                Op::Add(_,_) => HOp::Add(b_val, a_idx),
                Op::Sub(_,_) => HOp::SubR(a_idx, b_val),
                Op::Mul(_,_) => HOp::Mul(b_val, a_idx),
                Op::Div(_,_) => HOp::DivR(a_idx, b_val),
                _ => panic!(),
            };
            let idx = ops.len();
            ops.push(hop);
            Res::Op(idx)
        },
        (Res::Op(_), Res::Op(_)) => panic!(),
    }
}

#[allow(dead_code)]
fn show(idx: usize, ops: &[HOp]) -> String {
    match ops[idx] {
        HOp::Human => "X".into(),
        HOp::Add(val, i) => format!("({} + {})", val, show(i, ops)),
        HOp::Sub(val, i) => format!("({} - {})", val, show(i, ops)),
        HOp::SubR(i, val) => format!("({} - {})", show(i, ops), val),
        HOp::Mul(val, i) => format!("({} * {})", val, show(i, ops)),
        HOp::Div(val, i) => format!("({} / {})", val, show(i, ops)),
        HOp::DivR(i, val) => format!("({} / {})", show(i, ops), val),
    }
}

fn solve(initialvalue: i64, initialidx: usize, ops: &[HOp]) -> i64 {
    let mut value = initialvalue;
    let mut idx = initialidx;
    loop {
        match ops[idx] {
            HOp::Human => { return value; },
            HOp::Add(val, i) => { value -= val; idx = i; },
            HOp::Mul(val, i) => { value /= val; idx = i; },
            HOp::Sub(val, i) => { value = val - value; idx = i},
            HOp::SubR(i, val) => { value += val; idx = i},
            HOp::Div(val, i) => { value = val / value; idx = i},
            HOp::DivR(i, val) => { value *= val; idx = i},
        }
    };
}

fn part2(input: &[Monkey]) -> i64 {
    let monkeys: HashMap<&str,&Monkey> = HashMap::from_iter(
        input.iter().map(|m| (m.name.as_str(), m)));
    let root = monkeys["root"];
    let (a, b) = match &root.op {
        Op::Add(a, b) |
        Op::Sub(a, b) |
        Op::Mul(a, b) |
        Op::Div(a, b) => (a, b),
        _ => panic!(),
    };
    let mut ops: Vec<HOp> = Vec::new();
    let av = resolve_h(a, &monkeys, &mut ops);
    let bv = resolve_h(b, &monkeys, &mut ops);
    let (val, opidx) = match (av, bv) {
        (Res::Const(v), Res::Op(idx)) |
        (Res::Op(idx), Res::Const(v)) => (v, idx),
        _ => panic!(),
    };
    // println!("{} = {val}", show(opidx, &ops));
    solve(val, opidx, &ops)
}

fn main() {
    let input: Vec<Monkey> = read_input::<Monkey>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day21_test() {
        let input: Vec<Monkey> = test_input(include_str!("day21.testinput"));
        assert_eq!(part1(&input), 152);
        assert_eq!(part2(&input), 301);
    }
}
