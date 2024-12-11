use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 11 Part 1: {}", part1());
    println!("Day 11 Part 2: {}", part2());
}

fn blink(row: &mut Vec<u64>) {
    *row = row
        .iter()
        .flat_map(|stone| {
            if *stone == 0 {
                return vec![1];
            }
            let str = stone.to_string();
            if str.len() % 2 == 0 {
                let (left, right) = str.split_at(str.len() / 2);
                return vec![left.parse().unwrap(), right.parse().unwrap()];
            }
            return vec![stone * 2024];
        })
        .collect_vec()
}

pub fn part1() -> usize {
    let mut row = INPUT
        .trim()
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    for _ in 0..25 {
        blink(&mut row);
    }
    row.len()
}

fn blink_unordered(row: &mut HashMap<u64, u64>) {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (stone, count) in row.iter() {
        if *stone == 0 {
            *new_stones.entry(1).or_default() += *count;
        } else if stone.ilog10() % 2 == 1 {
            let string = stone.to_string();
            let (left, right) = string.split_at(string.len() / 2);
            *new_stones.entry(left.parse().unwrap()).or_default() += *count;
            *new_stones.entry(right.parse().unwrap()).or_default() += *count;
        } else {
            *new_stones.entry(stone * 2024).or_default() += *count;
        }
    }
    *row = new_stones;
}

pub fn part2() -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for stone in INPUT
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
    {
        *stones.entry(stone).or_default() += 1;
    }

    for _ in 0..75 {
        blink_unordered(&mut stones);
    }

    stones.values().sum()
}
