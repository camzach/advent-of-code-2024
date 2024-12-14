use std::{fs::File, path::Path};

use bmp::{Image, Pixel};
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 14 Part 1: {}", part1());
    println!("Day 14 Part 2: {}", part2());
}

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

#[derive(Debug)]
struct RobotParseError;
impl TryFrom<&str> for Robot {
    type Error = RobotParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = regex.captures(value).unwrap();
        Ok(Robot {
            pos: (
                captures
                    .get(1)
                    .ok_or(RobotParseError)?
                    .as_str()
                    .parse()
                    .map_err(|_| RobotParseError)?,
                captures
                    .get(2)
                    .ok_or(RobotParseError)?
                    .as_str()
                    .parse()
                    .map_err(|_| RobotParseError)?,
            ),
            vel: (
                captures
                    .get(3)
                    .ok_or(RobotParseError)?
                    .as_str()
                    .parse()
                    .map_err(|_| RobotParseError)?,
                captures
                    .get(4)
                    .ok_or(RobotParseError)?
                    .as_str()
                    .parse()
                    .map_err(|_| RobotParseError)?,
            ),
        })
    }
}

pub fn part1() -> u32 {
    let mut robots: Vec<Robot> = INPUT
        .trim()
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<_, _>>()
        .unwrap();
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots.iter_mut() {
        robot.pos.0 += robot.vel.0 * 100;
        robot.pos.0 %= 101;
        while robot.pos.0 < 0 {
            robot.pos.0 += 101;
        }
        robot.pos.1 += robot.vel.1 * 100;
        robot.pos.1 %= 103;
        while robot.pos.1 < 0 {
            robot.pos.1 += 103;
        }

        if robot.pos.1 < 51 {
            if robot.pos.0 > 50 {
                q1 += 1;
            }
            if robot.pos.0 < 50 {
                q2 += 1;
            }
        }
        if robot.pos.1 > 51 {
            if robot.pos.0 < 50 {
                q3 += 1;
            }
            if robot.pos.0 > 50 {
                q4 += 1;
            }
        }
    }

    q1 * q2 * q3 * q4
}

fn render_robots(robots: &Vec<Vec<Vec<Robot>>>, path: &Path) {
    let mut img = Image::new(101, 103);
    for y in 0..103 {
        for x in 0..101 {
            if robots[y][x].is_empty() {
                img.set_pixel(x as u32, y as u32, Pixel::new(0, 0, 0));
            } else {
                img.set_pixel(x as u32, y as u32, Pixel::new(255, 255, 255));
            }
        }
    }
    let mut destination = File::options().create(true).write(true).open(path).unwrap();
    img.to_writer(&mut destination).unwrap();
}

pub fn part2() -> u32 {
    let mut grid: Vec<Vec<Vec<Robot>>> = (0..103)
        .map(|_| (0..101).map(|_| vec![]).collect())
        .collect();

    for robot in INPUT
        .trim()
        .lines()
        .map(|line| <&str as TryInto<Robot>>::try_into(line).unwrap())
    {
        let (x, y) = robot.pos;
        grid[y as usize][x as usize].push(robot);
    }
    let base = Path::new(file!()).parent().unwrap().join("results");
    for i in 0..10_000 {
        render_robots(&grid, &base.join(format!("{i}.bmp")));
        let mut next_grid: Vec<Vec<Vec<Robot>>> = (0..103)
            .map(|_| (0..101).map(|_| vec![]).collect())
            .collect();
        for y in 0..103 {
            for x in 0..101 {
                for robot in grid[y][x].drain(..) {
                    let mut x = x as isize;
                    x += robot.vel.0;
                    x %= 101;
                    while x < 0 {
                        x += 101;
                    }
                    let mut y = y as isize;
                    y += robot.vel.1;
                    y %= 103;
                    while y < 0 {
                        y += 103;
                    }
                    next_grid[y as usize][x as usize].push(robot);
                }
            }
        }
        grid = next_grid
    }
    0
}
