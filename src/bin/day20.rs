use std::vec::Vec;
use advent_lib::read::read_input;

struct Num {
    value: i64,
    initial_idx: usize,
}

fn movenum(array: &mut Vec<Num>, idx: usize, dx: i64) {
    let mut newiidx:i64 = (idx as i64) + dx;
    let len = array.len() as i64;
    if newiidx < 0 {
        newiidx = newiidx.rem_euclid(len - 1);
    }
    if newiidx >= len - 1 {
        newiidx = newiidx  % (len - 1);
    }
    let newidx:usize = newiidx as usize;
    let item = array.splice(idx..=idx, []).next().unwrap();
    array.splice(newidx..newidx, [item]);
}

fn mix(input: &Vec<i64>, nums: &mut Vec<Num>) {
    for (i, _) in input.iter().enumerate() {
        let (idx, num) = nums.iter().enumerate().find(|(_, n)| n.initial_idx == i).unwrap();
        if num.value != 0 {
            movenum(nums, idx, num.value);
        }
    }
    /*
    println!("{}",
        nums.iter().map(|n| n.value.to_string()).collect::<Vec<String>>().join(", ")
    );
    */
}

fn part1(input: &Vec<i64>) {
    let mut nums: Vec<Num> = Vec::with_capacity(input.len());
    for (idx, v) in input.iter().enumerate() {
        nums.push(Num { value: *v, initial_idx: idx });
    }
    mix(input, &mut nums);
    let (idx, _) = nums.iter().enumerate().find(|(_, n)| n.value == 0).unwrap();
    let n1 = nums[(idx + 1000) % nums.len()].value;
    let n2 = nums[(idx + 2000) % nums.len()].value;
    let n3 = nums[(idx + 3000) % nums.len()].value;
    println!("Part 1: {}", n1 + n2 + n3);
}

fn part2(input: &Vec<i64>) {
    let mut nums: Vec<Num> = Vec::with_capacity(input.len());
    let key: i64 = 811589153;
    for (idx, v) in input.iter().enumerate() {
        nums.push(Num { value: *v * key, initial_idx: idx });
    }
    for _ in 0..10 {
        mix(input, &mut nums);
    }
    let (idx, _) = nums.iter().enumerate().find(|(_, n)| n.value == 0).unwrap();
    let n1 = nums[(idx + 1000) % nums.len()].value;
    let n2 = nums[(idx + 2000) % nums.len()].value;
    let n3 = nums[(idx + 3000) % nums.len()].value;
    println!("Part 2: {}", n1 + n2 + n3);
}

fn main() {
    let input: Vec<i64> = read_input::<i64>();
    part1(&input);
    part2(&input);
}
