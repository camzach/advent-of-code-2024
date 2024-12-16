use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 15 Part 1: {}", part1());
    println!("Day 15 Part 2: {}", part2());
}

#[derive(Debug, PartialEq)]
enum Tile {
    Robot,
    Box,
    Wall,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_robot(
    x: usize,
    y: usize,
    dir: Direction,
    grid: &mut Vec<Vec<Option<Tile>>>,
) -> (usize, usize) {
    assert!(
        grid[y][x] == Some(Tile::Robot),
        "Tile at {x},{y} wasn't a robot"
    );
    match dir {
        Direction::Up => {
            let mut y_end = y - 1;
            while grid[y_end][x] == Some(Tile::Box) {
                y_end -= 1;
            }
            if grid[y_end][x] == Some(Tile::Wall) {
                return (x, y);
            }
            grid[y][x] = None;
            grid[y - 1][x] = Some(Tile::Robot);
            if y_end != y - 1 {
                grid[y_end][x] = Some(Tile::Box);
            }
            return (x, y - 1);
        }
        Direction::Down => {
            let mut y_end = y + 1;
            while grid[y_end][x] == Some(Tile::Box) {
                y_end += 1;
            }
            if grid[y_end][x] == Some(Tile::Wall) {
                return (x, y);
            }
            grid[y][x] = None;
            grid[y + 1][x] = Some(Tile::Robot);
            if y_end != y + 1 {
                grid[y_end][x] = Some(Tile::Box);
            }
            return (x, y + 1);
        }
        Direction::Left => {
            let mut x_end = x - 1;
            while grid[y][x_end] == Some(Tile::Box) {
                x_end -= 1;
            }
            if grid[y][x_end] == Some(Tile::Wall) {
                return (x, y);
            }
            grid[y][x] = None;
            grid[y][x - 1] = Some(Tile::Robot);
            if x_end != x - 1 {
                grid[y][x_end] = Some(Tile::Box);
            }
            return (x - 1, y);
        }
        Direction::Right => {
            let mut x_end = x + 1;
            while grid[y][x_end] == Some(Tile::Box) {
                x_end += 1;
            }
            if grid[y][x_end] == Some(Tile::Wall) {
                return (x, y);
            }
            grid[y][x] = None;
            grid[y][x + 1] = Some(Tile::Robot);
            if x_end != x + 1 {
                grid[y][x_end] = Some(Tile::Box);
            }
            return (x + 1, y);
        }
    }
}

pub fn part1() -> usize {
    let (grid, instructions) = INPUT.split_once("\n\n").unwrap();
    let (mut x, mut y) = (0, 0);
    let mut grid = grid
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| match c {
                    '#' => Some(Tile::Wall),
                    'O' => Some(Tile::Box),
                    '@' => {
                        x = j;
                        y = i;
                        Some(Tile::Robot)
                    }
                    _ => None,
                })
                .collect_vec()
        })
        .collect_vec();

    for dir in instructions.chars() {
        if dir == '\n' {
            continue;
        }

        (x, y) = move_robot(
            x,
            y,
            match dir {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("Invalid direction"),
            },
            &mut grid,
        );
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| {
                    if Some(Tile::Box) == *tile {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

#[derive(PartialEq, Clone, Copy)]
enum Tile2 {
    Robot,
    Wall,
    BoxL,
    BoxR,
}

fn box_pushable(x: usize, y: usize, dir: Direction, grid: &mut Vec<Vec<Option<Tile2>>>) -> bool {
    if !matches!(grid[y][x], Some(Tile2::BoxL | Tile2::BoxR)) {
        return true;
    }
    let halves = match grid[y][x] {
        Some(Tile2::BoxL) => [(x, y), (x + 1, y)],
        Some(Tile2::BoxR) => [(x - 1, y), (x, y)],
        _ => unreachable!(),
    };
    match dir {
        Direction::Up => {
            return halves.iter().all(|(x, y)| match grid[*y - 1][*x] {
                Some(Tile2::BoxL | Tile2::BoxR) => box_pushable(*x, *y - 1, dir, grid),
                Some(Tile2::Wall) => false,
                _ => true,
            });
        }
        Direction::Down => {
            return halves.iter().all(|(x, y)| match grid[*y + 1][*x] {
                Some(Tile2::BoxL | Tile2::BoxR) => box_pushable(*x, *y + 1, dir, grid),
                Some(Tile2::Wall) => false,
                _ => true,
            });
        }
        _ => panic!("Only call box_pushable for Up and Down"),
    }
}

fn push_box(x: usize, y: usize, dir: Direction, grid: &mut Vec<Vec<Option<Tile2>>>) -> bool {
    match grid[y][x] {
        Some(Tile2::BoxL | Tile2::BoxR) => match dir {
            Direction::Up => {
                if box_pushable(x, y, dir, grid) {
                    let halves = match grid[y][x] {
                        Some(Tile2::BoxL) => [(x, y), (x + 1, y)],
                        Some(Tile2::BoxR) => [(x - 1, y), (x, y)],
                        _ => unreachable!(),
                    };
                    for half in halves {
                        if matches!(grid[half.1 - 1][half.0], Some(Tile2::BoxL | Tile2::BoxR)) {
                            push_box(half.0, half.1 - 1, dir, grid);
                        }
                        grid[half.1 - 1][half.0] = grid[half.1][half.0];
                        grid[half.1][half.0] = None;
                    }
                    return true;
                } else {
                    false
                }
            }
            Direction::Down => {
                if box_pushable(x, y, dir, grid) {
                    let halves = match grid[y][x] {
                        Some(Tile2::BoxL) => [(x, y), (x + 1, y)],
                        Some(Tile2::BoxR) => [(x - 1, y), (x, y)],
                        _ => unreachable!(),
                    };
                    for half in halves {
                        if matches!(grid[half.1 + 1][half.0], Some(Tile2::BoxL | Tile2::BoxR)) {
                            push_box(half.0, half.1 + 1, dir, grid);
                        }
                        grid[half.1 + 1][half.0] = grid[half.1][half.0];
                        grid[half.1][half.0] = None;
                    }
                    return true;
                } else {
                    false
                }
            }
            Direction::Left => {
                let mut x_end = x - 1;
                while matches!(grid[y][x_end], Some(Tile2::BoxL | Tile2::BoxR)) {
                    x_end -= 1;
                }
                if grid[y][x_end] == Some(Tile2::Wall) {
                    return false;
                }
                for x in x_end..x {
                    grid[y][x] = grid[y][x + 1];
                }
                grid[y][x] = None;
                return true;
            }
            Direction::Right => {
                let mut x_end = x + 1;
                while matches!(grid[y][x_end], Some(Tile2::BoxL | Tile2::BoxR)) {
                    x_end += 1;
                }
                if grid[y][x_end] == Some(Tile2::Wall) {
                    return false;
                }
                for x in (x..=x_end).rev() {
                    grid[y][x] = grid[y][x - 1];
                }
                grid[y][x] = None;
                return true;
            }
        },
        None => true,
        _ => false,
    }
}

fn move_robot_2(
    x: usize,
    y: usize,
    dir: Direction,
    grid: &mut Vec<Vec<Option<Tile2>>>,
) -> (usize, usize) {
    assert!(
        grid[y][x] == Some(Tile2::Robot),
        "Tile at {x},{y} wasn't a robot"
    );
    match dir {
        Direction::Up => {
            if push_box(x, y - 1, dir, grid) {
                grid[y][x] = None;
                grid[y - 1][x] = Some(Tile2::Robot);
                return (x, y - 1);
            }
            return (x, y);
        }
        Direction::Down => {
            if push_box(x, y + 1, dir, grid) {
                grid[y][x] = None;
                grid[y + 1][x] = Some(Tile2::Robot);
                return (x, y + 1);
            }
            return (x, y);
        }
        Direction::Left => {
            if push_box(x - 1, y, dir, grid) {
                grid[y][x] = None;
                grid[y][x - 1] = Some(Tile2::Robot);
                return (x - 1, y);
            }
            return (x, y);
        }
        Direction::Right => {
            if push_box(x + 1, y, dir, grid) {
                grid[y][x] = None;
                grid[y][x + 1] = Some(Tile2::Robot);
                return (x + 1, y);
            }
            return (x, y);
        }
    };
}

pub fn part2() -> usize {
    let (grid, instructions) = INPUT.split_once("\n\n").unwrap();
    let (mut x, mut y) = (0, 0);
    let mut grid = grid
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .flat_map(|(j, c)| match c {
                    '#' => [Some(Tile2::Wall); 2],
                    'O' => [Some(Tile2::BoxL), Some(Tile2::BoxR)],
                    '@' => {
                        x = j * 2;
                        y = i;
                        [Some(Tile2::Robot), None]
                    }
                    _ => [None; 2],
                })
                .collect_vec()
        })
        .collect_vec();

    for dir in instructions.chars() {
        if dir == '\n' {
            continue;
        }
        (x, y) = move_robot_2(
            x,
            y,
            match dir {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '>' => Direction::Right,
                '<' => Direction::Left,
                _ => panic!("Invalid direction"),
            },
            &mut grid,
        );
    }

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| {
                    if Some(Tile2::BoxL) == *tile {
                        y * 100 + x
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}
