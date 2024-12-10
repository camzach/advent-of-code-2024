use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 10 Part 1: {}", part1());
    println!("Day 10 Part 2: {}", part2());
}

fn find_endpoints(
    (x, y): (usize, usize),
    grid: &Vec<Vec<i8>>,
    endpoints: &mut Vec<Vec<Option<HashSet<(usize, usize)>>>>,
) -> HashSet<(usize, usize)> {
    if let Some(set) = endpoints[y][x].clone() {
        return set;
    }
    let n = grid[y][x];
    if n == 9 {
        return HashSet::from([(x, y)]);
    }
    let mut collected_points = HashSet::new();
    if y > 0 && grid[y - 1][x] - n == 1 {
        let set =
            endpoints[y - 1][x]
                .clone()
                .unwrap_or(find_endpoints((x, y - 1), grid, endpoints));
        for p in set {
            collected_points.insert(p);
        }
    }
    if y < grid.len() - 1 && grid[y + 1][x] - n == 1 {
        let set =
            endpoints[y + 1][x]
                .clone()
                .unwrap_or(find_endpoints((x, y + 1), grid, endpoints));
        for p in set {
            collected_points.insert(p);
        }
    }
    if x > 0 && grid[y][x - 1] - n == 1 {
        let set =
            endpoints[y][x - 1]
                .clone()
                .unwrap_or(find_endpoints((x - 1, y), grid, endpoints));
        for p in set {
            collected_points.insert(p);
        }
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] - n == 1 {
        let set =
            endpoints[y][x + 1]
                .clone()
                .unwrap_or(find_endpoints((x + 1, y), grid, endpoints));
        for p in set {
            collected_points.insert(p);
        }
    }
    if let Some(set) = &mut endpoints[y][x] {
        for point in collected_points.iter() {
            set.insert(point.clone());
        }
    } else {
        endpoints[y][x] = Some(collected_points.clone());
    }
    collected_points
}

pub fn part1() -> usize {
    let grid: Vec<Vec<i8>> = INPUT
        .lines()
        .map(|l| l.bytes().map(|d| d as i8 - b'0' as i8).collect())
        .collect();

    let mut endpoints: Vec<Vec<Option<HashSet<(usize, usize)>>>> = itertools::repeat_n(
        itertools::repeat_n(None, grid[0].len()).collect_vec(),
        grid.len(),
    )
    .collect_vec();

    let mut total_scores = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let points = find_endpoints((x, y), &grid, &mut endpoints);
                total_scores += points.len()
            }
        }
    }

    total_scores
}

fn find_paths(
    (x, y): (usize, usize),
    grid: &Vec<Vec<i8>>,
    paths: &mut Vec<Vec<Option<u32>>>,
) -> u32 {
    if let Some(n_paths) = paths[y][x].clone() {
        return n_paths;
    }
    let n = grid[y][x];
    if n == 9 {
        return 1;
    }
    let mut collected_paths = 0;
    if y > 0 && grid[y - 1][x] - n == 1 {
        let n_paths = paths[y - 1][x]
            .clone()
            .unwrap_or(find_paths((x, y - 1), grid, paths));
        collected_paths += n_paths;
    }
    if y < grid.len() - 1 && grid[y + 1][x] - n == 1 {
        let n_paths = paths[y + 1][x]
            .clone()
            .unwrap_or(find_paths((x, y + 1), grid, paths));
        collected_paths += n_paths;
    }
    if x > 0 && grid[y][x - 1] - n == 1 {
        let n_paths = paths[y][x - 1]
            .clone()
            .unwrap_or(find_paths((x - 1, y), grid, paths));
        collected_paths += n_paths;
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] - n == 1 {
        let n_paths = paths[y][x + 1]
            .clone()
            .unwrap_or(find_paths((x + 1, y), grid, paths));
        collected_paths += n_paths;
    }
    if let Some(n_paths) = &mut paths[y][x] {
        *n_paths += collected_paths;
    } else {
        paths[y][x] = Some(collected_paths);
    }
    collected_paths
}

pub fn part2() -> u32 {
    let grid: Vec<Vec<i8>> = INPUT
        .lines()
        .map(|l| l.bytes().map(|d| d as i8 - b'0' as i8).collect())
        .collect();

    let mut paths: Vec<Vec<Option<u32>>> = itertools::repeat_n(
        itertools::repeat_n(None, grid[0].len()).collect_vec(),
        grid.len(),
    )
    .collect_vec();

    let mut total_trails = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                let paths = find_paths((x, y), &grid, &mut paths);
                total_trails += paths;
            }
        }
    }

    total_trails
}
