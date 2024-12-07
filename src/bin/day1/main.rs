use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 1 Part 1: {}", part1());
    println!("Day 1 Part 2: {}", part2());
}

pub fn part1() -> u32 {
    let mut list_a: Vec<u32> = vec![];
    let mut list_b: Vec<u32> = vec![];
    for line in INPUT.lines() {
        let mut split = line.split(' ');
        let a = split.next().unwrap();
        let b = split.last().unwrap();
        list_a.push(a.parse().unwrap());
        list_b.push(b.parse().unwrap());
    }
    list_a.sort();
    list_b.sort();

    let mut sum = 0;
    for (a, b) in list_a.iter().zip(list_b) {
        sum += a.abs_diff(b);
    }

    sum
}

pub fn part2() -> u32 {
    let mut counts_left: HashMap<u32, u32> = HashMap::new();
    let mut counts_right: HashMap<u32, u32> = HashMap::new();
    for line in INPUT.lines() {
        let mut split = line.split(' ');
        let a = split.next().unwrap().parse().unwrap();
        let b = split.last().unwrap().parse().unwrap();

        *(counts_left.entry(a).or_insert(0)) += 1;
        *(counts_right.entry(b).or_insert(0)) += 1;
    }

    let mut sum = 0;
    for (num, count_left) in counts_left {
        if let Some(count_right) = counts_right.get(&num) {
            sum += count_left * (num * count_right)
        }
    }

    sum
}
