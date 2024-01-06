use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use ya_advent_lib::read::read_input;

#[derive(Debug, Eq, PartialEq, Hash)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq, Hash)]
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((opponent, me)) = s.split_once(' ') {
            Ok(
                RawTurn {
                    opponent: opponent.into(),
                    me: me.into(),
                }
            )
        }
        else {
            Err(())
        }
    }
}

struct Rules {
    opponent_map: HashMap<&'static str, Move>,
    my_map: HashMap<&'static str, Move>,
    outcome_map: HashMap<&'static str, Outcome>,
    move_scoring: HashMap<Move, i32>,
    outcome_scoring: HashMap<Outcome, i32>,
}

lazy_static! {
    static ref RULES: Rules = Rules {
        opponent_map: HashMap::from([
            ("A", Move::Rock),
            ("B", Move::Paper),
            ("C", Move::Scissors),
        ]),
        my_map: HashMap::from([
            ("X", Move::Rock),
            ("Y", Move::Paper),
            ("Z", Move::Scissors),
        ]),
        outcome_map: HashMap::from([
            ("X", Outcome::Lose),
            ("Y", Outcome::Draw),
            ("Z", Outcome::Win),
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

fn score1(turn: &RawTurn) -> i32 {
    let opponent_move = RULES.opponent_map.get(&*turn.opponent).unwrap();
    let my_move = RULES.my_map.get(&*turn.me).unwrap();

    let outcome = get_outcome(&my_move, &opponent_move);

    RULES.move_scoring.get(&my_move).unwrap()
      + RULES.outcome_scoring.get(&outcome).unwrap()
}

fn part1(input: &Vec<RawTurn>) -> i32 {
    input.iter().map(|turn| score1(&turn)).sum::<i32>()
}

fn score2(turn: &RawTurn) -> i32 {
    let opponent_move = RULES.opponent_map.get(&*turn.opponent).unwrap();
    let my_move = get_move(RULES.outcome_map.get(&*turn.me).unwrap(), &opponent_move);

    let outcome = get_outcome(&my_move, &opponent_move);

    RULES.move_scoring.get(&my_move).unwrap()
      + RULES.outcome_scoring.get(&outcome).unwrap()
}
fn part2(input: &Vec<RawTurn>) -> i32 {
    input.iter().map(|turn| score2(&turn)).sum::<i32>()
}

fn main() {
    let input: Vec<RawTurn> = read_input::<RawTurn>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<RawTurn> = test_input("A Y\nB X\nC Z\n");
        assert_eq!(part1(&input), 15);
        assert_eq!(part2(&input), 12);
    }
}
