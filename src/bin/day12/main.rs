use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 12 Part 1: {}", part1());
    println!("Day 12 Part 2: {}", part2());
}

struct Region {
    label: char,
    spaces: HashSet<(usize, usize)>,
}

fn flood_fill(
    x: usize,
    y: usize,
    parent: &Rc<RefCell<Region>>,
    grid: &Vec<Vec<char>>,
    regions: &mut HashMap<(usize, usize), Rc<RefCell<Region>>>,
) {
    if regions.contains_key(&(x, y)) {
        return;
    }
    let mut borrowed_parent = RefCell::borrow_mut(&parent);
    if grid[y][x] != borrowed_parent.label {
        return;
    }
    borrowed_parent.spaces.insert((x, y));
    drop(borrowed_parent);
    regions.insert((x, y), parent.clone());
    if y > 0 {
        flood_fill(x, y - 1, parent, grid, regions);
    }
    if x > 0 {
        flood_fill(x - 1, y, parent, grid, regions);
    }
    if y < grid.len() - 1 {
        flood_fill(x, y + 1, parent, grid, regions);
    }
    if x < grid[y].len() - 1 {
        flood_fill(x + 1, y, parent, grid, regions);
    }
}

fn region_perimeter(region: &Region, grid: &Vec<Vec<char>>) -> u32 {
    let mut perimeter = 0;
    for (x, y) in region.spaces.iter() {
        if *y == 0 || grid[y - 1][*x] != region.label {
            perimeter += 1;
        }
        if *x == 0 || grid[*y][x - 1] != region.label {
            perimeter += 1;
        }
        if *y >= grid.len() - 1 || grid[y + 1][*x] != region.label {
            perimeter += 1
        }
        if *x >= grid[*y].len() - 1 || grid[*y][x + 1] != region.label {
            perimeter += 1
        }
    }
    perimeter
}

pub fn part1() -> u32 {
    let mut space_to_region: HashMap<(usize, usize), Rc<RefCell<Region>>> = HashMap::new();
    let mut regions: Vec<Rc<RefCell<Region>>> = vec![];
    let grid = INPUT
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if space_to_region.contains_key(&(x, y)) {
                continue;
            }
            let region = Rc::new(RefCell::new(Region {
                label: grid[y][x],
                spaces: HashSet::new(),
            }));
            flood_fill(x, y, &region, &grid, &mut space_to_region);
            regions.push(region);
        }
    }

    regions
        .iter()
        .map(|region| {
            let region = RefCell::borrow(&region);
            region.spaces.len() as u32 * region_perimeter(&region, &grid)
        })
        .sum::<u32>()
}

fn count_sides(region: &Region) -> usize {
    let (xs, ys): (Vec<_>, Vec<_>) = region.spaces.iter().cloned().unzip();
    let top = *ys.iter().min().unwrap();
    let bottom = *ys.iter().max().unwrap();
    let left = *xs.iter().min().unwrap();
    let right = *xs.iter().max().unwrap();

    let mut horizontal_sides: usize = 0;
    for y in top..=bottom + 1 {
        let mut on_side = false;
        let mut prev = None;
        for x in left..=right {
            let here = region.spaces.contains(&(x, y));
            let here_above = if y > 0 {
                region.spaces.contains(&(x, y - 1))
            } else {
                false
            };
            if here == here_above {
                on_side = false;
                prev = Some(here);
                continue;
            }
            if !on_side {
                on_side = true;
                horizontal_sides += 1;
            } else if let Some(prevv) = prev {
                if prevv != here {
                    horizontal_sides += 1;
                }
            }
            prev = Some(here);
        }
    }
    let mut vertical_sides: usize = 0;
    for x in left..=right + 1 {
        let mut on_side = false;
        let mut prev = None;
        for y in top..=bottom {
            let here = region.spaces.contains(&(x, y));
            let here_above = if x > 0 {
                region.spaces.contains(&(x - 1, y))
            } else {
                false
            };
            if here == here_above {
                on_side = false;
                prev = Some(here);
                continue;
            }
            if !on_side {
                on_side = true;
                vertical_sides += 1;
            } else if let Some(prevv) = prev {
                if prevv != here {
                    vertical_sides += 1;
                }
            }
            prev = Some(here);
        }
    }
    horizontal_sides + vertical_sides
}

pub fn part2() -> usize {
    let mut space_to_region: HashMap<(usize, usize), Rc<RefCell<Region>>> = HashMap::new();
    let mut regions: Vec<Rc<RefCell<Region>>> = vec![];
    let grid = INPUT
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if space_to_region.contains_key(&(x, y)) {
                continue;
            }
            let region = Rc::new(RefCell::new(Region {
                label: grid[y][x],
                spaces: HashSet::new(),
            }));
            flood_fill(x, y, &region, &grid, &mut space_to_region);
            regions.push(region);
        }
    }

    regions
        .iter()
        .map(|region| {
            let region = RefCell::borrow(&region);
            region.spaces.len() * count_sides(&region)
        })
        .sum()
}
