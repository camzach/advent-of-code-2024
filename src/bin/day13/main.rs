use ndarray::{array, s, Axis};
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 13 Part 1: {}", part1());
    println!("Day 13 Part 2: {}", part2());
}

#[derive(Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

fn parse_machine(machine: &str) -> Option<Machine> {
    let machine_parser = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .ok()?;

    let captures = machine_parser.captures(machine)?;

    Some(Machine {
        a: (
            captures.get(1)?.as_str().parse().ok()?,
            captures.get(2)?.as_str().parse().ok()?,
        ),
        b: (
            captures.get(3)?.as_str().parse().ok()?,
            captures.get(4)?.as_str().parse().ok()?,
        ),
        prize: (
            captures.get(5)?.as_str().parse().ok()?,
            captures.get(6)?.as_str().parse().ok()?,
        ),
    })
}

fn gauss_jordan(matrix: &mut ndarray::Array2<f64>) {
    for j in 0..matrix.len_of(Axis(0)) {
        let v = matrix[[j, j]];
        if v != 1. {
            let mut row = matrix.row_mut(j);
            row /= v;
        }

        for i in 0..matrix.len_of(Axis(0)) {
            if i != j {
                let subtract = &matrix.row(j) * matrix[[i, j]];
                let mut row = matrix.row_mut(i);
                row -= &subtract;
            }
        }
    }
}

pub fn part1() -> f64 {
    let machines: Vec<Machine> = INPUT
        .trim()
        .split("\n\n")
        .map(parse_machine)
        .collect::<Option<_>>()
        .unwrap();

    let mut total_cost = 0.;
    for machine in machines {
        let mut matrix = array![
            [machine.a.0, machine.b.0, machine.prize.0],
            [machine.a.1, machine.b.1, machine.prize.1]
        ];
        gauss_jordan(&mut matrix);
        let presses = matrix.slice(s![.., 2]);
        // Icky yucky float slop
        if presses.iter().all(|el| f64::abs(*el - el.round()) < 0.001) {
            total_cost += presses[0 as usize] * 3.0 + presses[1 as usize];
        }
    }

    total_cost
}

pub fn part2() -> f64 {
    let machines: Vec<Machine> = INPUT
        .trim()
        .split("\n\n")
        .map(parse_machine)
        .collect::<Option<_>>()
        .unwrap();

    let mut total_cost = 0.;
    for machine in machines {
        let mut matrix = array![
            [
                machine.a.0,
                machine.b.0,
                machine.prize.0 + 10_000_000_000_000.0
            ],
            [
                machine.a.1,
                machine.b.1,
                machine.prize.1 + 10_000_000_000_000.0
            ]
        ];
        gauss_jordan(&mut matrix);
        let presses = matrix.slice(s![.., 2]);
        // Icky yucky float slop
        if presses.iter().all(|el| f64::abs(*el - el.round()) < 0.001) {
            total_cost += presses[0 as usize] * 3.0 + presses[1 as usize];
        }
    }

    total_cost
}
