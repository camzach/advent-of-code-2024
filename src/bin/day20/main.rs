use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 20 Part 1: {}", part1());
    println!("Day 20 Part 2: {}", part2());
}

#[derive(Debug)]
enum Tile {
    Path(Option<u32>),
    Wall,
}

fn build_fastest_path((x, y): (usize, usize), grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let Tile::Path(Some(dist)) = grid[y][x] else {
        return vec![];
    };
    if dist == 0 {
        return vec![];
    }
    let mut result = vec![];
    if y > 0 {
        if let Tile::Path(Some(d)) = grid[y - 1][x] {
            if d == dist - 1 {
                result.push((x, y - 1));
                result.append(&mut build_fastest_path((x, y - 1), grid));
            }
        }
    }
    if y < grid.len() - 1 {
        if let Tile::Path(Some(d)) = grid[y + 1][x] {
            if d == dist - 1 {
                result.push((x, y + 1));
                result.append(&mut build_fastest_path((x, y + 1), grid));
            }
        }
    }
    if x > 0 {
        if let Tile::Path(Some(d)) = grid[y][x - 1] {
            if d == dist - 1 {
                result.push((x - 1, y));
                result.append(&mut build_fastest_path((x - 1, y), grid));
            }
        }
    }
    if x < grid.len() - 1 {
        if let Tile::Path(Some(d)) = grid[y][x + 1] {
            if d == dist - 1 {
                result.push((x + 1, y));
                result.append(&mut build_fastest_path((x + 1, y), grid));
            }
        }
    }

    result
}

fn discover_cheats(
    (x, y): (usize, usize),
    timer: usize,
    parent_dist: u32,
    grid: &Vec<Vec<Tile>>,
) -> Vec<((usize, usize), u32)> {
    let min_x = x.saturating_sub(timer);
    let min_y = y.saturating_sub(timer);
    let max_x = (x + timer).min(grid[0].len() - 1);
    let max_y = (y + timer).min(grid.len() - 1);

    if x == 7 && y == 1 {
        println!("");
    }

    (min_x..=max_x)
        .cartesian_product(min_y..=max_y)
        .filter_map(|(x_, y_)| {
            let d = x_.abs_diff(x) + y_.abs_diff(y);
            if d <= timer {
                Some(((x_, y_), d))
            } else {
                None
            }
        })
        .filter_map(|((x, y), shortcut_len)| {
            let Tile::Path(Some(d)) = grid[y][x] else {
                return None;
            };
            if d + shortcut_len as u32 >= parent_dist {
                return None;
            }
            return Some(((x, y), parent_dist - (d + shortcut_len as u32)));
        })
        .collect_vec()
}

pub fn part1() -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: Vec<Vec<Tile>> = INPUT
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Path(None),
                    'S' => {
                        start = (x, y);
                        Tile::Path(None)
                    }
                    'E' => {
                        end = (x, y);
                        Tile::Path(None)
                    }
                    _ => Tile::Wall,
                })
                .collect()
        })
        .collect();

    let mut frontier: Vec<((usize, usize), u32)> = vec![(end, 0)];
    while let Some(((x, y), parent_dist)) = frontier.pop() {
        let tile = &mut grid[y][x];
        if let Tile::Path(prev_dist) = tile {
            if prev_dist.map(|d| d > parent_dist).unwrap_or(true) {
                prev_dist.replace(parent_dist);
                if y > 0 {
                    frontier.push(((x, y - 1), parent_dist + 1));
                }
                if x > 0 {
                    frontier.push(((x - 1, y), parent_dist + 1));
                }
                if y < grid.len() - 1 {
                    frontier.push(((x, y + 1), parent_dist + 1));
                }
                if x < grid[0].len() - 1 {
                    frontier.push(((x + 1, y), parent_dist + 1));
                }
            }
        }
    }

    let path = vec![start]
        .into_iter()
        .chain(build_fastest_path(start, &grid).into_iter())
        .collect_vec();

    let cheats = path
        .iter()
        .flat_map(|(x, y)| {
            if let Tile::Path(Some(parent_dist)) = grid[*y][*x] {
                discover_cheats((*x, *y), 2, parent_dist, &grid)
            } else {
                vec![]
            }
        })
        .collect_vec();

    cheats.iter().filter(|(_, len)| *len >= 100).count()
}

pub fn part2() -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: Vec<Vec<Tile>> = INPUT
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.char_indices()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Path(None),
                    'S' => {
                        start = (x, y);
                        Tile::Path(None)
                    }
                    'E' => {
                        end = (x, y);
                        Tile::Path(None)
                    }
                    _ => Tile::Wall,
                })
                .collect()
        })
        .collect();

    let mut frontier: Vec<((usize, usize), u32)> = vec![(end, 0)];
    while let Some(((x, y), parent_dist)) = frontier.pop() {
        let tile = &mut grid[y][x];
        if let Tile::Path(prev_dist) = tile {
            if prev_dist.map(|d| d > parent_dist).unwrap_or(true) {
                prev_dist.replace(parent_dist);
                if y > 0 {
                    frontier.push(((x, y - 1), parent_dist + 1));
                }
                if x > 0 {
                    frontier.push(((x - 1, y), parent_dist + 1));
                }
                if y < grid.len() - 1 {
                    frontier.push(((x, y + 1), parent_dist + 1));
                }
                if x < grid[0].len() - 1 {
                    frontier.push(((x + 1, y), parent_dist + 1));
                }
            }
        }
    }

    let path = vec![start]
        .into_iter()
        .chain(build_fastest_path(start, &grid).into_iter())
        .collect_vec();

    let cheats = path
        .iter()
        .flat_map(|(x, y)| {
            if let Tile::Path(Some(parent_dist)) = grid[*y][*x] {
                discover_cheats((*x, *y), 20, parent_dist, &grid)
            } else {
                vec![]
            }
        })
        .collect_vec();

    let mut cheat_counts: HashMap<u32, u32> = HashMap::new();
    for (_, len) in cheats.iter() {
        *cheat_counts.entry(*len).or_default() += 1;
    }

    cheats.iter().filter(|(_, len)| *len >= 100).count()
}
