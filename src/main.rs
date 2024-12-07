use core::str;

use std::{
    fs::{create_dir_all, write},
    path::Path,
    process::Command,
};

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
            let mut cmd = Command::new("cargo");
            cmd.args(["run", "--bin", format!("day{day}").as_str()]);
            match cmd.output() {
                Ok(result) => {
                    println!("{}", str::from_utf8(&result.stdout).unwrap())
                }
                Err(_) => println!("Command failed to execute"),
            };
        }
        "new" => {
            let mut cmd = Command::new("sed");
            let day = Utc::now().naive_local().day();
            cmd.args([format!("s/%DAY%/{}/", day).as_mut_str(), "template.txt"]);
            match cmd.output() {
                Ok(result) => {
                    let dir_path = format!("./src/bin/day{}", day);
                    let path = Path::new(dir_path.as_str());
                    if path.exists() {
                        println!("Directory for day {day} already exists");
                        return;
                    }
                    create_dir_all(path).expect("Failed to create path");
                    write(path.join("main.rs"), result.stdout).expect("Failed to write file");
                    write(path.join("input.txt"), "").expect("Failed to create input.txt");
                }
                Err(_) => println!("Failed to fill out template"),
            }
        }
        els => println!("Unexpected argument {els}"),
    }
}
