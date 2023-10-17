use std::convert::From;
use std::fmt;
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Snafu(i64);

impl Snafu {
    fn to_s( val: i64) -> String {
        if val == 0 { return "".into(); }
        match val % 5 {
            0 | 1 | 2 => Self::to_s(val / 5) + &((val % 5).to_string()),
            3 => Self::to_s(1 + val / 5) + "=".into(),
            4 => Self::to_s(1 + val / 5) + "-".into(),
            _ => panic!(),
        }
    }
}

impl From<&String> for Snafu {
    fn from(value: &String) -> Self {
        let mut result: i64 = 0;
        for c in value.chars() {
            result *= 5;
            result += match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            }
        }
        Snafu(result)
    }
}
impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        Snafu(value)
    }
}
impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::to_s(self.0))
    }
}

fn main() {
    let input: Vec<String> = read_input();
    let sum:i64 = input.iter().map(|s| Snafu::from(s)).map(|s| s.0).sum();
    println!("Part 1: {}", Snafu::from(sum));
}
