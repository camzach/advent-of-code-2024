use regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 3 Part 1: {}", part1());
    println!("Day 3 Part 2: {}", part2());
}

pub fn part1() -> i32 {
    let pattern = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for captures in pattern.captures_iter(INPUT) {
        let (_, [a, b]) = captures.extract();
        let product = a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        sum += product;
    }
    sum
}

pub fn part2() -> i32 {
    let mul_pattern = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_pattern = regex::Regex::new(r"do()").unwrap();
    let dont_pattern = regex::Regex::new(r"don't()").unwrap();

    let any_pattern = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut sum = 0;
    for m in any_pattern.find_iter(INPUT) {
        if let Some(captures) = mul_pattern.captures(m.as_str()) {
            if !enabled {
                continue;
            }
            let (_, [a, b]) = captures.extract();
            sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
        if do_pattern.is_match(m.as_str()) {
            enabled = true;
        }
        if dont_pattern.is_match(m.as_str()) {
            enabled = false;
        }
    }
    sum
}
