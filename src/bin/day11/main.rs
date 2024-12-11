use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Clone)]
struct Stone {
    timer: usize,
    children: (u64, u64),
}

fn cached_stone(n: u64, cache: &mut HashMap<u64, Stone>) -> Stone {
    if let Some(stone) = cache.get(&n) {
        return stone.clone();
    }
    let mut t = 0;
    let mut new_n = n;
    if new_n == 0 {
        new_n = 1;
        t += 1;
    }
    while new_n.ilog10() % 2 == 0 {
        new_n *= 2024;
        t += 1;
    }
    let string = new_n.to_string();
    let (left, right) = string.split_at(string.len() / 2);
    let stone = Stone {
        timer: t,
        children: (left.parse().unwrap(), right.parse().unwrap()),
    };
    cache.insert(n, stone.clone());
    stone
}

fn blink_unordered(row: &mut HashMap<u64, VecDeque<u64>>, cache: &mut HashMap<u64, Stone>) {
    let mut new_stones = vec![];
    for (stone, timer_counts) in row.iter_mut() {
        let completed = timer_counts.pop_front().unwrap();
        timer_counts.push_back(0);
        if completed > 0 {
            let stone = cached_stone(*stone, cache);
            let (left, right) = stone.children;
            new_stones.push((left, completed));
            new_stones.push((right, completed));
        }
    }
    for (stone, count) in new_stones {
        let timer = cached_stone(stone, cache).timer;
        let row = row.entry(stone).or_default();
        row.resize(timer + 1, 0);
        row[timer] += count;
    }
}

pub fn part2() -> u64 {
    let mut cache = HashMap::new();
    let mut stones: HashMap<u64, VecDeque<u64>> = HashMap::new();
    for (n, timer) in INPUT.trim().split_ascii_whitespace().map(|n| {
        let n = n.parse::<u64>().unwrap();
        let stone = cached_stone(n, &mut cache);
        (n, stone.timer)
    }) {
        let counts = stones.entry(n).or_default();
        counts.resize(timer + 1, 0);
        counts[timer] += 1;
    }

    for _ in 0..75 {
        blink_unordered(&mut stones, &mut cache);
    }

    stones.values().flatten().sum()
}
