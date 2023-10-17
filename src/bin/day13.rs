use std::cmp::Ordering;
use std::str::FromStr;
use std::vec::Vec;
use json;
use json::JsonValue;
use advent_lib::read::read_grouped_input;

struct JsonValueWrapper {
    value: JsonValue
}

impl FromStr for JsonValueWrapper {
    type Err = json::JsonError;
    fn from_str(s: &str) -> Result<Self, json::JsonError> {
        let result = json::parse(s);
        match result {
            Ok(value) => Ok(JsonValueWrapper { value: value }),
            Err(e) => Err(e),
        }
    }
}

fn check_order(left: &JsonValue, right: &JsonValue) -> Ordering {
    match (left, right) {
        (JsonValue::Number(l), JsonValue::Number(r)) => {
            let ln:f64 = (*l).into();
            let rn:f64 = (*r).into();
            ln.partial_cmp(&rn).unwrap()
        }
        (JsonValue::Array(l), JsonValue::Array(r)) => {
            let mut idx = 0usize;
            while idx < l.len() && idx < r.len() {
                let v = check_order(&l[idx], &r[idx]);
                idx += 1;
                match v {
                   Ordering::Equal => { continue; },
                    _ => { return v },
                }
            }
            l.len().cmp(&r.len())
        },
        (JsonValue::Number(l), JsonValue::Array(r)) =>
            check_order(&JsonValue::Array(vec![JsonValue::Number(l.clone())]), &JsonValue::Array(r.to_vec())),
        (JsonValue::Array(l), JsonValue::Number(r)) =>
            check_order(&JsonValue::Array(l.to_vec()), &JsonValue::Array(vec![JsonValue::Number(r.clone())])),
        _ => panic!(),
    }
}

fn part1(input: &Vec<Vec<JsonValueWrapper>>) {
    let mut correct_pairs: Vec<usize> = Vec::new();
    for (idx, row) in input.iter().enumerate() {
        let result = check_order(&row[0].value, &row[1].value);
        match result {
            Ordering::Less => { correct_pairs.push(idx + 1); },
            Ordering::Greater => {},
            Ordering::Equal => { panic!(); },
        }
    }
    let result:usize = correct_pairs.iter().sum();
    println!("Part 1: {}", result);
}

fn part2(input: &Vec<Vec<JsonValueWrapper>>) {
    let mut items:Vec<&JsonValue> = input.iter().flatten().map(|x| &x.value).collect();
    let div1 = json::parse("[[2]]").unwrap();
    let div2 = json::parse("[[6]]").unwrap();
    items.push(&div1);
    items.push(&div2);
    items.sort_by(|a,b| check_order(a,b));
    let idx1 = items.iter().position(|x| *x == &div1).unwrap() + 1;
    let idx2 = items.iter().position(|x| *x == &div2).unwrap() + 1;
    println!("Part 2: {}", idx1 * idx2);
}

fn main() {
    let input: Vec<Vec<JsonValueWrapper>> = read_grouped_input();
    part1(&input);
    part2(&input);
}
