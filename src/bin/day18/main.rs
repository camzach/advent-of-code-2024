const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 18 Part 1: {}", part1());
    println!("Day 18 Part 2: {}", part2());
}

const X_LEN: usize = 71;
const Y_LEN: usize = 71;

#[derive(Clone)]
enum Tile {
    Safe(Option<u32>),
    Corrupted,
}

pub fn part1() -> u32 {
    const N_BYTES: usize = 1024;
    let mut grid: Vec<Vec<Tile>> = itertools::repeat_n(
        itertools::repeat_n(Tile::Safe(None), X_LEN).collect(),
        Y_LEN,
    )
    .collect();
    for line in INPUT.trim().lines().take(N_BYTES) {
        let (x, y) = line.split_once(',').unwrap();
        grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = Tile::Corrupted;
    }

    let mut frontier: Vec<((usize, usize), u32)> = vec![((0, 0), 0)];
    while let Some(((x, y), parent_dist)) = frontier.pop() {
        let tile = &mut grid[y][x];
        if let Tile::Safe(prev_dist) = tile {
            if prev_dist.map(|d| d > parent_dist).unwrap_or(true) {
                prev_dist.replace(parent_dist);
                if y > 0 {
                    frontier.push(((x, y - 1), parent_dist + 1));
                }
                if x > 0 {
                    frontier.push(((x - 1, y), parent_dist + 1));
                }
                if y < Y_LEN - 1 {
                    frontier.push(((x, y + 1), parent_dist + 1));
                }
                if x < X_LEN - 1 {
                    frontier.push(((x + 1, y), parent_dist + 1));
                }
            }
        }
    }

    if let Tile::Safe(Some(dist)) = grid[Y_LEN - 1][X_LEN - 1] {
        dist
    } else {
        panic!("Can't reach end")
    }
}

pub fn part2() -> String {
    let mut left = 0;
    let mut right = INPUT.trim().lines().count() - 1;

    while left != right {
        let mut grid: Vec<Vec<Tile>> = itertools::repeat_n(
            itertools::repeat_n(Tile::Safe(None), X_LEN).collect(),
            Y_LEN,
        )
        .collect();
        let midpoint = left + (right - left) / 2;
        for line in INPUT.trim().lines().take(midpoint + 1) {
            let (x, y) = line.split_once(',').unwrap();
            grid[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = Tile::Corrupted;
        }

        let mut frontier: Vec<((usize, usize), u32)> = vec![((0, 0), 0)];
        while let Some(((x, y), parent_dist)) = frontier.pop() {
            let tile = &mut grid[y][x];
            if let Tile::Safe(prev_dist) = tile {
                if prev_dist.map(|d| d > parent_dist).unwrap_or(true) {
                    prev_dist.replace(parent_dist);
                    if y > 0 {
                        frontier.push(((x, y - 1), parent_dist + 1));
                    }
                    if x > 0 {
                        frontier.push(((x - 1, y), parent_dist + 1));
                    }
                    if y < Y_LEN - 1 {
                        frontier.push(((x, y + 1), parent_dist + 1));
                    }
                    if x < X_LEN - 1 {
                        frontier.push(((x + 1, y), parent_dist + 1));
                    }
                }
            }
        }

        if let Tile::Safe(None) = grid[Y_LEN - 1][X_LEN - 1] {
            right = midpoint - 1;
        } else {
            left = midpoint + 1;
        }
    }

    INPUT.lines().nth(left).unwrap().into()
}
