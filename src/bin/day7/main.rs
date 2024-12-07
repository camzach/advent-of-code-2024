use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 7 Part 1: {}", part1());
    println!("Day 7 Part 2: {}", part2());
}

fn part1() -> u64 {
    let mut total = 0;
    'line: for line in INPUT.lines() {
        let (target, numbers) = line.split_once(':').unwrap();
        let target = target.parse::<u64>().unwrap();
        let numbers = numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();
        let permutations = (0..numbers.len() - 1)
            .map(|_| ['*', '+'])
            .multi_cartesian_product()
            .collect_vec();
        for perm in permutations {
            let mut running_result = numbers[0];
            for (number, operator) in numbers.iter().skip(1).zip(perm) {
                match operator {
                    '+' => running_result += number,
                    '*' => running_result *= number,
                    _ => unreachable!(),
                }
            }
            if running_result == target {
                total += target;
                continue 'line;
            }
        }
    }
    total
}

fn part2() -> u64 {
    let mut total = 0;
    'line: for line in INPUT.lines() {
        let (target, numbers) = line.split_once(':').unwrap();
        let target = target.parse::<u64>().unwrap();
        let numbers = numbers
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect_vec();
        let permutations = (0..numbers.len() - 1)
            .map(|_| ['*', '+', '|'])
            .multi_cartesian_product()
            .collect_vec();
        for perm in permutations {
            let mut running_result = numbers[0];
            for (number, operator) in numbers.iter().skip(1).zip(perm) {
                match operator {
                    '+' => running_result += number,
                    '*' => running_result *= number,
                    '|' => {
                        let digits = number.ilog10() + 1;
                        running_result *= 10_u64.pow(digits);
                        running_result += number;
                    }
                    _ => unreachable!(),
                }
            }
            if running_result == target {
                total += target;
                continue 'line;
            }
        }
    }
    total
}
