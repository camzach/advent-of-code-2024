use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn add_coords(pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    Some((
        pos.0.checked_add_signed(dir.0)?,
        pos.1.checked_add_signed(dir.1)?,
    ))
}

pub fn part1() {
    let grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let guard_idx = INPUT.find('^').unwrap();
    let mut dir: (isize, isize) = (0, -1);
    let mut guard_pos = (
        guard_idx % (grid[0].len() + 1),
        guard_idx / (grid[0].len() + 1),
    );
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    loop {
        visited.insert(guard_pos);
        if let Some(next_pos) = add_coords(guard_pos, dir) {
            if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                break;
            }
            match grid[next_pos.1][next_pos.0] {
                '.' | '^' => {
                    guard_pos = next_pos;
                }
                '#' => {
                    dir = match dir {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => panic!("Invalid direction"),
                    }
                }
                c => panic!("Unexpected char in grid, {c}"),
            }
        } else {
            break;
        }
    }

    println!("Day 6 Part 1: {}", visited.len())
}

pub fn part2() {
    let grid: Vec<Vec<char>> = INPUT
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let guard_idx = INPUT.find('^').unwrap();
    let mut dir: (isize, isize) = (0, -1);
    let mut guard_pos = (
        guard_idx % (grid[0].len() + 1),
        guard_idx / (grid[0].len() + 1),
    );
    let mut visited: HashMap<(usize, usize), (isize, isize)> = HashMap::new();
    loop {
        visited.entry(guard_pos).or_insert(dir);
        if let Some(next_pos) = add_coords(guard_pos, dir) {
            if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                break;
            }
            match grid[next_pos.1][next_pos.0] {
                '.' | '^' => {
                    guard_pos = next_pos;
                }
                '#' => {
                    dir = match dir {
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        _ => panic!("Invalid direction"),
                    }
                }
                c => panic!("Unexpected char in grid, {c}"),
            }
        } else {
            break;
        }
    }

    let mut total = 0;
    let initial_guard_pos = (
        guard_idx % (grid[0].len() + 1),
        guard_idx / (grid[0].len() + 1),
    );
    for (guard_pos, mut dir) in visited {
        if guard_pos == initial_guard_pos {
            continue;
        }
        let mut grid = grid.clone();
        grid[guard_pos.1][guard_pos.0] = '#';
        let Some(mut guard_pos) = add_coords(guard_pos, (dir.0 * -1, dir.1 * -1)) else {
            continue;
        };

        let mut visited: HashMap<(usize, usize), Vec<(isize, isize)>> = HashMap::new();

        loop {
            let visits = visited.entry(guard_pos).or_default();
            if visits.contains(&dir) {
                total += 1;
                break;
            }
            visits.push(dir);
            if let Some(next_pos) = add_coords(guard_pos, dir) {
                if next_pos.0 >= grid.len() || next_pos.1 >= grid[0].len() {
                    break;
                }
                match grid[next_pos.1][next_pos.0] {
                    '.' | '^' => {
                        guard_pos = next_pos;
                    }
                    '#' => {
                        dir = match dir {
                            (0, -1) => (1, 0),
                            (1, 0) => (0, 1),
                            (0, 1) => (-1, 0),
                            (-1, 0) => (0, -1),
                            _ => panic!("Invalid direction"),
                        }
                    }
                    c => panic!("Unexpected char in grid, {c}"),
                }
            } else {
                break;
            }
        }
    }

    println!("Day 6 Part 2: {total}");
}
