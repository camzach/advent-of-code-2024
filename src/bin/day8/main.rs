use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 8 Part 1: {}", part1());
    println!("Day 8 Part 2: {}", part2());
}

pub fn part1() -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);
    for (y, line) in INPUT.trim().lines().enumerate() {
        if y as i32 > max_y {
            max_y = y as i32;
        }
        for (x, char) in line.char_indices() {
            if x as i32 > max_x {
                max_x = x as i32;
            }
            if char == '.' {
                continue;
            }
            antennas.entry(char).or_default().push((x as i32, y as i32))
        }
    }
    let mut antinodes = HashSet::new();
    for coords in antennas.values() {
        for pair in coords.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            let p1 = (b.0 + dx, b.1 + dy);
            if !(p1.0 < 0 || p1.1 < 0 || p1.0 > max_x || p1.1 > max_y) {
                antinodes.insert(p1);
            }
            let p2 = (a.0 - dx, a.1 - dy);
            if !(p2.0 < 0 || p2.1 < 0 || p2.0 > max_x || p2.1 > max_y) {
                antinodes.insert(p2);
            }
        }
    }

    antinodes.len()
}

pub fn part2() -> usize {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let (mut max_x, mut max_y) = (0, 0);
    for (y, line) in INPUT.trim().lines().enumerate() {
        if y as i32 > max_y {
            max_y = y as i32;
        }
        for (x, char) in line.char_indices() {
            if x as i32 > max_x {
                max_x = x as i32;
            }
            if char == '.' {
                continue;
            }
            antennas.entry(char).or_default().push((x as i32, y as i32))
        }
    }
    let mut antinodes = HashSet::new();
    for coords in antennas.values() {
        for pair in coords.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;

            let mut temp_x = a.0 - dx;
            let mut temp_y = a.1 - dy;
            while temp_x >= 0 && temp_x <= max_x && temp_y >= 0 && temp_y <= max_y {
                antinodes.insert((temp_x, temp_y));
                temp_x -= dx;
                temp_y -= dy;
            }
            let mut temp_x = b.0 + dx;
            let mut temp_y = b.1 + dy;
            while temp_x >= 0 && temp_x <= max_x && temp_y >= 0 && temp_y <= max_y {
                antinodes.insert((temp_x, temp_y));
                temp_x += dx;
                temp_y += dy;
            }
        }
    }

    for positions in antennas.values() {
        for pos in positions {
            antinodes.insert(*pos);
        }
    }

    antinodes.len()
}
