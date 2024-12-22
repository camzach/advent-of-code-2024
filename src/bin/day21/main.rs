use std::collections::HashMap;

use itertools::{repeat_n, Itertools};
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 21 Part 1: {}", part1());
    println!("Day 21 Part 2: {}", part2());
}

fn num_pad_coords(num: char) -> (i8, i8) {
    match num {
        '0' => (1, 3),
        'A' => (2, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!("Not a vaild numpad key"),
    }
}

fn dir_pad_coords(dir: char) -> (i8, i8) {
    match dir {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!("Not a valid dir key"),
    }
}

fn code_movements(code: &str) -> Vec<char> {
    let mut pos = num_pad_coords('A');
    code.chars()
        .flat_map(|c| {
            let target = num_pad_coords(c);
            let (dx, dy) = (target.0 - pos.0, target.1 - pos.1);
            let horizontal = repeat_n(if dx > 0 { '>' } else { '<' }, dx.abs() as usize);
            let vertical = repeat_n(if dy > 0 { 'v' } else { '^' }, dy.abs() as usize);
            let res =
                (if (dx > 0 && !(pos.0 == 0 && target.1 == 3)) || (target.0 == 0 && pos.1 == 3) {
                    vertical.chain(horizontal)
                } else {
                    horizontal.chain(vertical)
                })
                .chain(['A']);
            pos = target;
            res.collect_vec()
        })
        .collect_vec()
}

fn abstract_movements(moves: &Vec<char>) -> Vec<char> {
    let mut pos = dir_pad_coords('A');
    moves
        .iter()
        .flat_map(|c| {
            let target = dir_pad_coords(*c);
            let (dx, dy) = (target.0 - pos.0, target.1 - pos.1);
            let horizontal = repeat_n(if dx > 0 { '>' } else { '<' }, dx.abs() as usize);
            let vertical = repeat_n(if dy > 0 { 'v' } else { '^' }, dy.abs() as usize);
            let res =
                (if (dx > 0 && !(pos.0 == 0 && target.1 == 0)) || (target.0 == 0 && pos.1 == 0) {
                    vertical.chain(horizontal)
                } else {
                    horizontal.chain(vertical)
                })
                .chain(['A']);
            pos = target;
            res.collect_vec()
        })
        .collect_vec()
}

fn sequence_complexity(sequence: &str) -> usize {
    let mut dirs = code_movements(sequence);
    for _ in 0..2 {
        dirs = abstract_movements(&dirs);
    }

    let numeric_part: usize = Regex::new(r"(\d+)A")
        .unwrap()
        .captures(sequence)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    numeric_part * dirs.len()
}

pub fn part1() -> usize {
    INPUT
        .trim()
        .lines()
        .map(|seq| sequence_complexity(seq))
        .sum()
}

fn recursive_complexity(
    sequence: String,
    depth: usize,
    cache: &mut HashMap<(String, usize), u64>,
) -> u64 {
    if let Some(result) = cache.get(&(sequence.clone(), depth)) {
        return *result;
    }

    if depth == 0 {
        return sequence.len() as u64;
    }

    let mut parts = sequence.chars().fold(vec![String::new()], |mut acc, e| {
        let last = acc.last_mut().unwrap();
        last.push(e);
        if e == 'A' {
            acc.push(String::new())
        }
        acc
    });
    parts.retain(|part| part.len() > 0);

    if parts.len() == 1 {
        let abstracted = abstract_movements(&sequence.chars().collect());
        let result = recursive_complexity(abstracted.iter().join(""), depth - 1, cache);
        cache.insert((sequence, depth), result);
        return result;
    }

    let mut total = 0;
    for part in parts {
        total += recursive_complexity(part, depth, cache);
    }
    cache.insert((sequence, depth), total);

    total
}

pub fn part2() -> u64 {
    let mut cache: HashMap<(String, usize), u64> = HashMap::new();

    INPUT
        .trim()
        .lines()
        .map(|line| {
            let inputs = code_movements(line).iter().join("");
            let numeric_part: usize = Regex::new(r"(\d+)A")
                .unwrap()
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            numeric_part as u64 * recursive_complexity(inputs, 25, &mut cache)
        })
        .sum()
}
