mod puzzles;

use puzzles::*;

use std::{env, fs, time};

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

            let start = time::Instant::now();
            let results = match day {
                1 => d1::solve(data.unwrap()),
                2 => d2::solve(data.unwrap()),
                3 => d3::solve(data.unwrap()),
                4 => d4::solve(data.unwrap()),
                5 => d5::solve(data.unwrap()),
                6 => d6::solve(data.unwrap()),
                7 => d7::solve(data.unwrap()),
                8 => d8::solve(data.unwrap()),
                9 => d9::solve(data.unwrap()),
                10 => d10::solve(data.unwrap()),
                11 => d11::solve(data.unwrap()),
                12 => d12::solve(data.unwrap()),
                _ => panic!("Day not implemented"),
            };
            let elapsed = start.elapsed();

            println!("Part 1: {}, Part 2: {} ({}ms)", results.0, results.1, elapsed.as_millis());
        }
    }
}
