mod puzzles;

use puzzles::*;

use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    match args.len() {
        1 => {
            panic!("Usage: cargo run <day>");
        }
        _ => {
            let day = match args[1].parse::<i32>() {
                Ok(day) => day,
                Err(_) => panic!("Invalid day"),
            };

            let input_file = format!("data/d{}.txt", day);
            let data = fs::read_to_string(input_file);
            match data {
                Ok(_) => println!("Data loaded"),
                Err(_) => panic!("Error loading data"),
            }

            let results = match day {
                1 => d1::solve(data.unwrap()),
                _ => panic!("Day not implemented"),
            };

            println!("Part 1: {}, Part 2: {}", results.0, results.1);
        }
    }
}
