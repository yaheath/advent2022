use std::convert::From;
use std::fmt;
use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Snafu(i64);

impl Snafu {
    fn to_s( val: i64) -> String {
        if val == 0 { return "".into(); }
        match val % 5 {
            0..=2 => Self::to_s(val / 5) + &((val % 5).to_string()),
            3 => Self::to_s(1 + val / 5) + "=",
            4 => Self::to_s(1 + val / 5) + "-",
            _ => panic!(),
        }
    }
}

impl FromStr for Snafu {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: i64 = 0;
        for c in s.chars() {
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
        Ok(Snafu(result))
    }
}
impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        Snafu(value)
    }
}
impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 0 {
            write!(f, "0")
        }
        else {
            write!(f, "{}", Self::to_s(self.0))
        }
    }
}
impl From<Snafu> for String {
    fn from(value: Snafu) -> String {
        Snafu::to_s(value.0)
    }
}
impl From<Snafu> for i64 {
    fn from(value: Snafu) -> i64 {
        value.0
    }
}

fn main() {
    let input: Vec<Snafu> = read_input();
    let sum = input.iter().map(|s| s.0).sum::<i64>();
    println!("Part 1: {}", Snafu::from(sum));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn snafu_test() {
        let stuff: Vec<(i64, &str)> = vec![
            (1,         "1"),
            (2,         "2"),
            (3,         "1="),
            (4,         "1-"),
            (5,         "10"),
            (6,         "11"),
            (7,         "12"),
            (8,         "2="),
            (9,         "2-"),
            (10,        "20"),
            (15,        "1=0"),
            (20,        "1-0"),
            (2022,      "1=11-2"),
            (12345,     "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (n, st) in stuff {
            let sn = st.parse::<Snafu>().unwrap();
            assert_eq!(n, sn.0);
            let sn:Snafu = n.into();
            let snstr:String = sn.to_string();
            assert_eq!(snstr, st.to_string());
        }
    }

    #[test]
    fn day25_test() {
        let input: Vec<Snafu> = test_input(include_str!("day25.testinput"));
        let sum: String = Snafu::from(input.iter().map(|s| s.0).sum::<i64>()).into();
        assert_eq!(sum, "2=-1=0".to_string());
    }
}
