use core::str;

use std::{
    fs::{create_dir_all, write},
    path::Path,
    process::Command,
};

use aoc_client::AocClient;
use chrono::{Datelike, Utc};
use clap::{self, command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Test { day: Option<u32> },
    New { day: Option<u32> },
}

fn main() {
    let today = Utc::now()
        .naive_local()
        .and_utc()
        .with_timezone(&chrono::offset::Local)
        .day();
    let cli = Cli::parse();
    match cli.command.unwrap_or(Commands::Test { day: None }) {
        Commands::Test { day } => {
            let day = day.unwrap_or(today);
            let mut cmd = Command::new("cargo");
            cmd.args(["run", "--bin", format!("day{day}").as_str()]);
            match cmd.output() {
                Ok(result) => {
                    println!("{}", str::from_utf8(&result.stdout).unwrap())
                }
                Err(_) => println!("Command failed to execute"),
            };
        }
        Commands::New { day } => {
            let day = day.unwrap_or(today);
            let template = include_str!("template.txt");
            let contents = template.replace("%DAY%", day.to_string().as_str());

            let dir_path = format!("./src/bin/day{}", day);
            let path = Path::new(dir_path.as_str());
            if path.exists() {
                println!("Directory for day {day} already exists");
                return;
            }

            let aoc_client = AocClient::builder()
                .input_filename(path.join("input.txt"))
                .year(2024)
                .expect("Invalid year")
                .day(day)
                .expect("Invalid day")
                .session_cookie_from_file("cookie")
                .expect("Failed to load session cookie")
                .build()
                .expect("Failed to connect to AoC");

            create_dir_all(path).expect("Failed to create path");
            write(path.join("main.rs"), contents).expect("Failed to write file");
            aoc_client
                .save_input()
                .expect("Failed to download puzzle input");
        }
    }
}
