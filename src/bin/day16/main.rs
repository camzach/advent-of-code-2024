use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    u32,
};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 16 Part 1: {}", part1());
    println!("Day 16 Part 2: {}", part2());
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
enum Tile {
    End(u32),
    Wall,
    Path(Rc<RefCell<HashMap<Direction, u32>>>),
}

pub fn part1() -> u32 {
    let height = INPUT.trim().lines().count();
    let width = INPUT.find("\n").unwrap();

    let mut grid: Vec<Vec<Tile>> = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == 'E' {
                        Tile::End(u32::MAX)
                    } else if c == '#' {
                        Tile::Wall
                    } else {
                        Tile::Path(Rc::new(RefCell::new(HashMap::new())))
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut frontier: Vec<((usize, usize, Direction), u32)> =
        vec![((1, height - 2, Direction::Right), 0)];
    while let Some(((x, y, dir), dist)) = frontier.pop() {
        let tile = &mut grid[y][x];
        match tile {
            Tile::End(d) => {
                if dist < *d {
                    *d = dist;
                }
            }
            Tile::Wall => continue,
            Tile::Path(dists) => {
                let mut prev_dists = dists.borrow_mut();
                if let Some(d) = prev_dists.get(&dir) {
                    if *d <= dist {
                        continue;
                    }
                }
                prev_dists.insert(dir, dist);
                match dir {
                    Direction::Up => {
                        frontier.push(((x, y - 1, Direction::Up), dist + 1));
                        frontier.push(((x, y, Direction::Left), dist + 1000));
                        frontier.push(((x, y, Direction::Right), dist + 1000));
                    }
                    Direction::Down => {
                        frontier.push(((x, y + 1, Direction::Down), dist + 1));
                        frontier.push(((x, y, Direction::Left), dist + 1000));
                        frontier.push(((x, y, Direction::Right), dist + 1000));
                    }
                    Direction::Left => {
                        frontier.push(((x - 1, y, Direction::Left), dist + 1));
                        frontier.push(((x, y, Direction::Up), dist + 1000));
                        frontier.push(((x, y, Direction::Down), dist + 1000));
                    }
                    Direction::Right => {
                        frontier.push(((x + 1, y, Direction::Right), dist + 1));
                        frontier.push(((x, y, Direction::Up), dist + 1000));
                        frontier.push(((x, y, Direction::Down), dist + 1000));
                    }
                }
            }
        }
    }

    if let Tile::End(dist) = grid[1][width - 2] {
        dist
    } else {
        0
    }
}

pub fn part2() -> usize {
    let height = INPUT.trim().lines().count();
    let width = INPUT.find("\n").unwrap();

    let mut grid: Vec<Vec<Tile>> = INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == 'E' {
                        Tile::End(u32::MAX)
                    } else if c == '#' {
                        Tile::Wall
                    } else {
                        Tile::Path(Rc::new(RefCell::new(HashMap::new())))
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut frontier: Vec<((usize, usize, Direction), u32)> =
        vec![((1, height - 2, Direction::Right), 0)];
    while let Some(((x, y, dir), dist)) = frontier.pop() {
        let tile = &mut grid[y][x];
        match tile {
            Tile::End(d) => {
                if dist < *d {
                    *d = dist;
                }
            }
            Tile::Wall => continue,
            Tile::Path(dists) => {
                let mut prev_dists = dists.borrow_mut();
                if let Some(d) = prev_dists.get(&dir) {
                    if *d <= dist {
                        continue;
                    }
                }
                prev_dists.insert(dir, dist);
                match dir {
                    Direction::Up => {
                        frontier.push(((x, y - 1, Direction::Up), dist + 1));
                        frontier.push(((x, y, Direction::Left), dist + 1000));
                        frontier.push(((x, y, Direction::Right), dist + 1000));
                    }
                    Direction::Down => {
                        frontier.push(((x, y + 1, Direction::Down), dist + 1));
                        frontier.push(((x, y, Direction::Left), dist + 1000));
                        frontier.push(((x, y, Direction::Right), dist + 1000));
                    }
                    Direction::Left => {
                        frontier.push(((x - 1, y, Direction::Left), dist + 1));
                        frontier.push(((x, y, Direction::Up), dist + 1000));
                        frontier.push(((x, y, Direction::Down), dist + 1000));
                    }
                    Direction::Right => {
                        frontier.push(((x + 1, y, Direction::Right), dist + 1));
                        frontier.push(((x, y, Direction::Up), dist + 1000));
                        frontier.push(((x, y, Direction::Down), dist + 1000));
                    }
                }
            }
        }
    }

    let mut tiles_on_track = HashSet::new();
    let mut frontier = vec![
        (width - 2, 1, Direction::Up),
        (width - 2, 1, Direction::Right),
    ];
    while let Some((x, y, dir)) = frontier.pop() {
        if tiles_on_track.contains(&(x, y, dir)) {
            continue;
        }
        let Some(d_to_here) = (match &grid[y][x] {
            Tile::End(val) => Some(*val),
            Tile::Path(lens) => lens.borrow().get(&dir).cloned(),
            Tile::Wall => None,
        }) else {
            continue;
        };
        tiles_on_track.insert((x, y, dir));
        match dir {
            Direction::Up => {
                if let Tile::Path(dists) = &grid[y + 1][x] {
                    if let Some(dist) = dists.borrow().get(&dir) {
                        if *dist < d_to_here && d_to_here - dist == 1 {
                            frontier.push((x, y + 1, dir));
                        }
                    }
                }
                if let Tile::Path(dists) = &grid[y][x] {
                    if let Some(dist) = dists.borrow().get(&Direction::Left) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Left));
                        }
                    }
                    if let Some(dist) = dists.borrow().get(&Direction::Right) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Right));
                        }
                    }
                }
            }
            Direction::Down => {
                if let Tile::Path(dists) = &grid[y - 1][x] {
                    if let Some(dist) = dists.borrow().get(&dir) {
                        if *dist < d_to_here && d_to_here - dist == 1 {
                            frontier.push((x, y - 1, dir));
                        }
                    }
                }
                if let Tile::Path(dists) = &grid[y][x] {
                    if let Some(dist) = dists.borrow().get(&Direction::Left) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Left));
                        }
                    }
                    if let Some(dist) = dists.borrow().get(&Direction::Right) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Right));
                        }
                    }
                }
            }
            Direction::Left => {
                if let Tile::Path(dists) = &grid[y][x + 1] {
                    if let Some(dist) = dists.borrow().get(&dir) {
                        if *dist < d_to_here && d_to_here - dist == 1 {
                            frontier.push((x + 1, y, dir));
                        }
                    }
                }
                if let Tile::Path(dists) = &grid[y][x] {
                    if let Some(dist) = dists.borrow().get(&Direction::Up) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Up));
                        }
                    }
                    if let Some(dist) = dists.borrow().get(&Direction::Down) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Down));
                        }
                    }
                }
            }
            Direction::Right => {
                if let Tile::Path(dists) = &grid[y][x - 1] {
                    if let Some(dist) = dists.borrow().get(&dir) {
                        if *dist < d_to_here && d_to_here - dist == 1 {
                            frontier.push((x - 1, y, dir));
                        }
                    }
                }
                if let Tile::Path(dists) = &grid[y][x] {
                    if let Some(dist) = dists.borrow().get(&Direction::Up) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Up));
                        }
                    }
                    if let Some(dist) = dists.borrow().get(&Direction::Down) {
                        if *dist < d_to_here && d_to_here - dist == 1000 {
                            frontier.push((x, y, Direction::Down));
                        }
                    }
                }
            }
        }
    }

    let tiles_on_track = tiles_on_track
        .iter()
        .cloned()
        .map(|(x, y, _)| (x, y))
        .unique()
        .collect_vec();

    tiles_on_track.len()
}
