use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 17 Part 1: {}", part1());
    println!("Day 17 Part 2: {}", part2());
}

pub fn part1() -> String {
    let (init, instructions) = INPUT.split_once("\n\n").unwrap();
    let regex = Regex::new(r"Register [ABC]: (\d+)").unwrap();
    let (mut a, mut b, mut c) = init
        .trim()
        .lines()
        .map(|line| {
            regex
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();
    let instructions: Vec<u64> = instructions.trim()[9..]
        .split(",")
        .map(|str| str.trim().parse().unwrap())
        .collect();
    let mut inst = 0;
    let mut output = vec![];
    while let (Some(opcode), Some(operand)) = (
        instructions.get(inst).cloned(),
        instructions.get(inst + 1).cloned(),
    ) {
        match opcode {
            0 => {
                let combo = match operand {
                    0..=3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Invalid combo operand"),
                };
                a = a >> combo;
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                let combo = match operand {
                    0..=3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Invalid combo operand"),
                };
                b = combo % 8;
            }
            3 => {
                if a != 0 {
                    inst = operand as usize;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                let combo = match operand {
                    0..=3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Invalid combo operand"),
                };
                output.push(combo % 8);
            }
            6 => {
                let combo = match operand {
                    0..=3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Invalid combo operand"),
                };
                b = a >> combo;
            }
            7 => {
                let combo = match operand {
                    0..=3 => operand,
                    4 => a,
                    5 => b,
                    6 => c,
                    _ => panic!("Invalid combo operand"),
                };
                c = a >> combo;
            }
            _ => panic!("Invalid opcode"),
        }
        inst += 2;
    }
    output.into_iter().join(",")
}

fn find_chunks(trits: Vec<u8>, targets: &Vec<u8>) -> Option<Vec<u8>> {
    if trits.len() == targets.len() {
        return Some(trits);
    }
    let mut a: u64 = 0;
    for trit in trits.iter() {
        a <<= 3;
        a |= *trit as u64;
    }

    'val: for val in 0..8 {
        let mut next_a = (a << 3) | val;
        for target in targets.iter().rev().take(trits.len() + 1).rev().cloned() {
            // one-line simplification of my particular input program
            let digit =
                (((next_a & 0b111) ^ 0b011) ^ (next_a >> ((next_a & 0b111) ^ 0b101))) & 0b111;
            next_a >>= 3;
            if digit as u8 != target {
                continue 'val;
            }
        }
        let mut result = trits.clone();
        result.push(val as u8);
        if let Some(successors) = find_chunks(result, targets) {
            return Some(successors);
        }
    }

    None
}

pub fn part2() -> u64 {
    let (_, instructions) = INPUT.split_once("\n\n").unwrap();
    let instructions: Vec<u8> = instructions.trim()[9..]
        .split(",")
        .map(|str| str.trim().parse().unwrap())
        .collect();

    let Some(trits) = find_chunks(vec![], &instructions) else {
        panic!("Unable to find the input")
    };
    let mut result: u64 = 0;
    for trit in trits.iter() {
        result <<= 3;
        result |= *trit as u64;
    }
    result
}
