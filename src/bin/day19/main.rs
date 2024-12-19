use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 19 Part 1: {}", part1());
    println!("Day 19 Part 2: {}", part2());
}

fn can_make_design(design: &str, towels: &Vec<&str>) -> bool {
    if design.len() == 0 {
        return true;
    }
    for towel in towels {
        if design.starts_with(towel) && can_make_design(&design[towel.len()..], towels) {
            return true;
        }
    }
    return false;
}

pub fn part1() -> usize {
    let (towels, designs) = INPUT.trim().split_once("\n\n").unwrap();
    let towels = towels
        .trim()
        .split(',')
        .map(|towel| towel.trim())
        .collect_vec();
    let designs = designs.trim().split('\n').map(|design| design.trim());

    designs
        .filter(|design| can_make_design(&design, &towels))
        .count()
}

fn ways_to_make_design<'a>(
    design: &'a str,
    towels: &Vec<&'a str>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }
    if let Some(result) = cache.get(design) {
        return *result;
    }
    let mut acc = 0;
    for towel in towels {
        if design.starts_with(towel) {
            acc += ways_to_make_design(&design[towel.len()..], towels, cache);
        }
    }
    cache.insert(design, acc);
    acc
}

pub fn part2() -> u64 {
    let (towels, designs) = INPUT.trim().split_once("\n\n").unwrap();
    let towels = towels
        .trim()
        .split(',')
        .map(|towel| towel.trim())
        .collect_vec();
    let designs = designs.trim().split('\n').map(|design| design.trim());

    let mut cache = HashMap::new();
    designs
        .map(|design| ways_to_make_design(&design, &towels, &mut cache))
        .sum()
}
