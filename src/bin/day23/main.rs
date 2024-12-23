use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 23 Part 1: {}", part1());
    println!("Day 23 Part 2: {}", part2());
}

fn find_k_cliques<'a>(edges: &'a HashMap<&str, Vec<&str>>, k: usize) -> Vec<Vec<&'a str>> {
    if k == 1 {
        return edges.keys().map(|n| vec![*n]).collect_vec();
    }
    let smaller_cliques = find_k_cliques(edges, k - 1);
    let extended_cliques = smaller_cliques
        .iter()
        .flat_map(|clique| {
            clique.iter().flat_map(|node| {
                edges
                    .get(node)
                    .unwrap()
                    .iter()
                    .filter(|n| {
                        !clique.contains(n)
                            && clique.iter().all(|c| edges.get(c).unwrap().contains(n))
                    })
                    .map(|n| {
                        let mut new = clique.clone();
                        new.push(n);
                        new
                    })
                    .collect_vec()
            })
        })
        .unique_by(|c| {
            let mut sorted = c.clone();
            sorted.sort();
            sorted.join("")
        })
        .collect_vec();

    extended_cliques
}

pub fn part1() -> usize {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in INPUT.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }

    let cliques = find_k_cliques(&edges, 4);
    cliques
        .iter()
        .filter(|c| c.iter().any(|el| el.starts_with('t')))
        .count()
}

pub fn part2() -> String {
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in INPUT.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }

    for k in (1..edges.values().map(|vec| vec.len()).max().unwrap() + 1).rev() {
        let k_cliques = find_k_cliques(&edges, k);
        if k_cliques.len() != 0 {
            let mut first = k_cliques.first().unwrap().to_owned();
            first.sort();
            return first.join(",");
        }
    }
    return "Could not find maximal clique".into();
}
