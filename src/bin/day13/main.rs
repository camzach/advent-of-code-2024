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

pub fn part1() -> f64 {
    let machines: Vec<Machine> = INPUT
        .trim()
        .split("\n\n")
        .map(parse_machine)
        .collect::<Option<_>>()
        .unwrap();

    let mut total_cost = 0.;
    for machine in machines {
        let (a, d) = machine.a;
        let (b, e) = machine.b;
        let (c, f) = machine.prize;
        let x = (c * e - f * b) / (a * e - d * b);
        let y = (c - (a * x)) / b;
        if (x.round() - x).abs() < 0.001 && (y.round() - y).abs() < 0.001 {
            total_cost += 3.0 * x + y;
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
        let (a, d) = machine.a;
        let (b, e) = machine.b;
        let (mut c, mut f) = machine.prize;
        c += 10_000_000_000_000.0;
        f += 10_000_000_000_000.0;
        let x = (c * e - f * b) / (a * e - d * b);
        let y = (c - (a * x)) / b;
        if (x.round() - x).abs() < 0.001 && (y.round() - y).abs() < 0.001 {
            total_cost += 3.0 * x + y;
        }
    }

    total_cost
}
