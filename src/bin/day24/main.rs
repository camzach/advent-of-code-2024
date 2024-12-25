use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 24 Part 1: {}", part1());
    println!("Day 24 Part 2: {}", part2());
}

#[derive(Debug)]
enum Gate {
    Static(bool),
    AND(&'static str, &'static str),
    OR(&'static str, &'static str),
    XOR(&'static str, &'static str),
}

fn parse_gates() -> HashMap<&'static str, Gate> {
    let mut gates = HashMap::new();
    let (init, logic) = INPUT.trim().split_once("\n\n").unwrap();
    for line in init.lines() {
        let pattern = Regex::new(r"(.{3}): (\d+)").unwrap();
        let captures = pattern.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        gates.insert(name, Gate::Static(if value == 1 { true } else { false }));
    }
    for line in logic.lines() {
        let pattern = Regex::new(r"(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})").unwrap();
        let captures = pattern.captures(line).unwrap();
        match captures.get(2).unwrap().as_str() {
            "AND" => {
                gates.insert(
                    captures.get(4).unwrap().as_str(),
                    Gate::AND(
                        captures.get(1).unwrap().as_str(),
                        captures.get(3).unwrap().as_str(),
                    ),
                );
            }
            "OR" => {
                gates.insert(
                    captures.get(4).unwrap().as_str(),
                    Gate::OR(
                        captures.get(1).unwrap().as_str(),
                        captures.get(3).unwrap().as_str(),
                    ),
                );
            }
            "XOR" => {
                gates.insert(
                    captures.get(4).unwrap().as_str(),
                    Gate::XOR(
                        captures.get(1).unwrap().as_str(),
                        captures.get(3).unwrap().as_str(),
                    ),
                );
            }
            _ => unreachable!(),
        }
    }
    gates
}

fn resolve_gate(gate: &Gate, gates: &HashMap<&str, Gate>) -> bool {
    match gate {
        Gate::Static(b) => *b,
        Gate::AND(a, b) => {
            resolve_gate(gates.get(a).unwrap(), gates) & resolve_gate(gates.get(b).unwrap(), gates)
        }
        Gate::OR(a, b) => {
            resolve_gate(gates.get(a).unwrap(), gates) | resolve_gate(gates.get(b).unwrap(), gates)
        }
        Gate::XOR(a, b) => {
            resolve_gate(gates.get(a).unwrap(), gates) ^ resolve_gate(gates.get(b).unwrap(), gates)
        }
    }
}

pub fn part1() -> u64 {
    let gates = parse_gates();

    let mut result: u64 = 0;
    for (_, v) in gates
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .sorted_by_key(|(k, _)| *k)
        .rev()
    {
        let value = resolve_gate(v, &gates);
        result <<= 1;
        result |= value as u64;
    }

    result
}

fn gate_name(name: &str, gate: &Gate) -> String {
    match gate {
        Gate::Static(_) => name.to_string(),
        Gate::AND(_, _) => format!("AND({name})"),
        Gate::OR(_, _) => format!("OR({name})"),
        Gate::XOR(_, _) => format!("XOR({name})"),
    }
}

pub fn part2() -> String {
    // Computing this seems very difficult, but the task is easy enough to accomplish with eyeballs
    // Use this output in a graph visualizer and do it by hand
    let gates = parse_gates();

    for (name, gate) in gates.iter() {
        let other_names = gates
            .iter()
            .filter(|(_, gate)| match gate {
                Gate::AND(a, b) | Gate::OR(a, b) | Gate::XOR(a, b) => a == name || b == name,
                _ => false,
            })
            .map(|(name, gate)| gate_name(name, gate));
        let name = gate_name(name, gate);
        println!(
            "{}",
            other_names.map(|n| format!("{name} -> {n}")).join("\n")
        );
    }
    "Inspect the above output in graphviz and find errors manually".into()
}
