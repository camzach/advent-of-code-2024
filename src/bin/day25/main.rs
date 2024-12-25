use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 25 Part 1: {}", part1());
    println!("Day 25 Part 2: {}", part2());
}

pub fn part1() -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) = INPUT
        .split("\n\n")
        .partition(|lock_or_key| lock_or_key.starts_with("#####"));
    let locks: Vec<[u8; 5]> = locks
        .iter()
        .map(|lock| {
            lock.trim()
                .lines()
                .skip(1)
                .fold([0, 0, 0, 0, 0], |mut acc, el| {
                    for (i, c) in el.char_indices() {
                        if c == '#' {
                            acc[i] += 1;
                        }
                    }
                    acc
                })
        })
        .collect();
    let keys: Vec<[u8; 5]> = keys
        .iter()
        .map(|key| {
            key.trim()
                .lines()
                .rev()
                .skip(1)
                .fold([0, 0, 0, 0, 0], |mut acc, el| {
                    for (i, c) in el.char_indices() {
                        if c == '#' {
                            acc[i] += 1;
                        }
                    }
                    acc
                })
        })
        .collect();

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5))
        .count()
}

pub fn part2() -> String {
    "No part 2 today! Just deliver the final chronicle to Santa! 🎅".into()
}
