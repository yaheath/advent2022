#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;
use derivative::Derivative;

#[derive(Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
struct RawTurn {
    opponent: String,
    me: String,
}

impl FromStr for RawTurn {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w) (\w)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            Ok(
                RawTurn {
                    opponent: caps.get(1).unwrap().as_str().into(),
                    me: caps.get(2).unwrap().as_str().into(),
                }
            )
        }
        else {
            Err("invalid input line".to_string())
        }
    }
}

struct Rules {
    opponent_map: HashMap<String, Move>,
    my_map: HashMap<String, Move>,
    outcome_map: HashMap<String, Outcome>,
    move_scoring: HashMap<Move, i32>,
    outcome_scoring: HashMap<Outcome, i32>,
}

fn get_outcome(me:&Move, opponent:&Move) -> Outcome {
    match (me, opponent) {
        (Move::Paper, Move::Rock) => Outcome::Win,
        (Move::Rock, Move::Paper) => Outcome::Lose,
        (Move::Rock, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Lose,
        (Move::Scissors, Move::Paper) => Outcome::Win,
        (Move::Paper, Move::Scissors) => Outcome::Lose,
        _ => Outcome::Draw,
    }
}

fn get_move(me:&Outcome, opponent:&Move) -> Move {
    match (me, opponent) {
        (Outcome::Win, Move::Rock) => Move::Paper,
        (Outcome::Win, Move::Paper) => Move::Scissors,
        (Outcome::Win, Move::Scissors) => Move::Rock,
        (Outcome::Lose, Move::Rock) => Move::Scissors,
        (Outcome::Lose, Move::Paper) => Move::Rock,
        (Outcome::Lose, Move::Scissors) => Move::Paper,
        (Outcome::Draw, Move::Rock) => Move::Rock,
        (Outcome::Draw, Move::Paper) => Move::Paper,
        (Outcome::Draw, Move::Scissors) => Move::Scissors,
    }
}

fn score1(turn: &RawTurn, rules: &Rules) -> i32 {
    let opponent_move = rules.opponent_map.get(&turn.opponent).unwrap();
    let my_move = rules.my_map.get(&turn.me).unwrap();

    let outcome = get_outcome(&my_move, &opponent_move);

    rules.move_scoring.get(&my_move).unwrap()
      + rules.outcome_scoring.get(&outcome).unwrap()
}

fn part1(input: &Vec<RawTurn>, rules: &Rules) {
    let result = input.iter().map(|turn| score1(&turn, rules)).sum::<i32>();
    println!("Part 1: {}", result);
}

fn score2(turn: &RawTurn, rules: &Rules) -> i32 {
    let opponent_move = rules.opponent_map.get(&turn.opponent).unwrap();
    let my_move = get_move(rules.outcome_map.get(&turn.me).unwrap(), &opponent_move);

    let outcome = get_outcome(&my_move, &opponent_move);

    rules.move_scoring.get(&my_move).unwrap()
      + rules.outcome_scoring.get(&outcome).unwrap()
}
fn part2(input: &Vec<RawTurn>, rules: &Rules) {
    let result = input.iter().map(|turn| score2(&turn, rules)).sum::<i32>();
    println!("Part 2: {}", result);
}

fn main() {
    let rules = Rules {
        opponent_map: HashMap::from([
            ("A".into(), Move::Rock),
            ("B".into(), Move::Paper),
            ("C".into(), Move::Scissors),
        ]),
        my_map: HashMap::from([
            ("X".into(), Move::Rock),
            ("Y".into(), Move::Paper),
            ("Z".into(), Move::Scissors),
        ]),
        outcome_map: HashMap::from([
            ("X".into(), Outcome::Lose),
            ("Y".into(), Outcome::Draw),
            ("Z".into(), Outcome::Win),
        ]),
        move_scoring: HashMap::from([
            (Move::Rock, 1),
            (Move::Paper, 2),
            (Move::Scissors, 3),
        ]),
        outcome_scoring: HashMap::from([
            (Outcome::Win, 6),
            (Outcome::Lose, 0),
            (Outcome::Draw, 3),
        ]),
    };

    let input: Vec<RawTurn> = read_input::<RawTurn>();
    part1(&input, &rules);
    part2(&input, &rules);
}
