use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("input.txt");

fn update_is_ordered(update: &Vec<&str>, rules: &HashMap<&str, Vec<&str>>) -> bool {
    let mut pages_seen: HashSet<&str> = HashSet::new();

    for page in update.iter() {
        let page = page;
        if let Some(successors) = rules.get(page) {
            if successors.iter().any(|p| pages_seen.contains(p)) {
                return false;
            }
        }
        pages_seen.insert(page);
    }
    return true;
}

pub fn part1() {
    let (rules, updates) = INPUT.split_once("\n\n").unwrap();

    let mut successor_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules.lines() {
        let (before, after) = rule.split_once("|").unwrap();

        successor_map.entry(before).or_insert(vec![]).push(after);
    }

    let mut total = 0;
    for update in updates.lines() {
        let update = update.split(',').collect();
        if update_is_ordered(&update, &successor_map) {
            let middle_page = update.get(update.len() / 2).unwrap();
            total += middle_page.parse::<i32>().unwrap();
        }
    }
    println!("Day 5 Part 1: {total}");
}

pub fn part2() {
    let (rules_text, updates) = INPUT.split_once("\n\n").unwrap();

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules_text.lines() {
        let (before, after) = rule.split_once("|").unwrap();
        rules.entry(before).or_default().push(after);
        rules.entry(after).or_default();
    }

    let mut total = 0;
    for update in updates.trim().split('\n') {
        let update = update.split(',').collect();
        if !update_is_ordered(&update, &rules) {
            let mut ordered_update = update.clone();
            ordered_update.sort_by(|a, b| {
                if rules.get(a).unwrap().contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            let middle_page = ordered_update.get(ordered_update.len() / 2).unwrap();
            total += middle_page.parse::<i32>().unwrap();
        }
    }

    println!("Day 5 Part 2: {total}");
}
