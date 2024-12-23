use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 22 Part 1: {}", part1());
    println!("Day 22 Part 2: {}", part2());
}

pub fn part1() -> u64 {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut num: u64 = line.parse().unwrap();
            for _ in 0..2000 {
                num = num ^ (num << 6);
                num = num % 16777216;
                num = num ^ (num >> 5);
                num = num % 16777216;
                num = num ^ (num << 11);
                num = num % 16777216;
            }
            num
        })
        .sum()
}

pub fn part2() -> u32 {
    let sequences = INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut changes: Vec<(u8, i8)> = vec![];
            let mut num: u64 = line.parse().unwrap();
            for _ in 0..2000 {
                let mut next = num;
                next = next ^ (next << 6);
                next = next % 16777216;
                next = next ^ (next >> 5);
                next = next % 16777216;
                next = next ^ (next << 11);
                next = next % 16777216;
                changes.push(((next % 10) as u8, (next % 10) as i8 - (num % 10) as i8));
                num = next;
            }
            changes
        })
        .collect_vec();

    let mut caches: Vec<HashMap<[i8; 4], u8>> = vec![];
    for seq in sequences.iter() {
        let mut cache = HashMap::new();
        for subseq in seq.windows(4) {
            let target_seq: [i8; 4] = subseq
                .iter()
                .map(|(_, delta)| *delta)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            if cache.contains_key(&target_seq) {
                continue;
            }
            for test_seq in seq.windows(4) {
                if test_seq
                    .iter()
                    .map(|(_, d)| d)
                    .zip(target_seq.iter())
                    .all(|(a, b)| a == b)
                {
                    cache.insert(target_seq, test_seq.last().unwrap().0);
                    break;
                }
            }
        }
        caches.push(cache);
    }

    let mut totals: HashMap<[i8; 4], u32> = HashMap::new();
    for cache in caches {
        for (key, value) in cache {
            *totals.entry(key).or_default() += value as u32;
        }
    }

    *totals.iter().max_by_key(|(_, v)| *v).unwrap().1
}
