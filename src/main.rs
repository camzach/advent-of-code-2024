use core::str;
use std::process::Command;

use chrono::{Datelike, Utc};
use regex::Regex;

fn main() {
    let day_pattern = Regex::new("^(day)?(?<number>\\d+)$").unwrap();
    match std::env::args().nth(1).unwrap_or("today".into()).as_str() {
        "today" => {
            let mut cmd = Command::new("cargo");
            cmd.args([
                "run",
                "--bin",
                format!("day{}", Utc::now().naive_local().day()).as_str(),
            ]);
            match cmd.output() {
                Ok(result) => {
                    println!("{}", str::from_utf8(&result.stdout).unwrap())
                }
                Err(_) => println!("Command failed to execute"),
            };
        }
        arg if day_pattern.is_match(arg) => {
            let day = day_pattern
                .captures(arg)
                .unwrap()
                .name("number")
                .unwrap()
                .as_str();
            println!("{day}");
            let mut cmd = Command::new("cargo");
            cmd.args(["run", "--bin", format!("day{day}").as_str()]);
            match cmd.output() {
                Ok(result) => {
                    println!("{}", str::from_utf8(&result.stdout).unwrap())
                }
                Err(_) => println!("Command failed to execute"),
            };
        }
        els => println!("Unexpected argument {els}"),
    }
}
