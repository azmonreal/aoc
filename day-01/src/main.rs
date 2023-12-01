use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;

        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                first = i;
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if line.chars().nth(i).unwrap().is_digit(10) {
                last = i;
                break;
            }
        }

        sum += line.chars().nth(first).unwrap().to_digit(10).unwrap() * 10
            + line.chars().nth(last).unwrap().to_digit(10).unwrap();
    }

    sum
}

fn part2(input: &str) -> u32 {
    let digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];

    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first = 0;
        let mut last = 0;

        'outer: for i in 0..line.len() {
            for (j, d) in digits.iter().enumerate() {
                if i + d.len() > line.len() {
                    continue;
                }
                if &line[i..d.len() + i] == *d {
                    first = j as u32;
                    break 'outer;
                }
            }
            if line.chars().nth(i).unwrap().is_digit(10) {
                first = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                break;
            }
        }

        'outer: for i in (0..line.len()).rev() {
            for (j, d) in digits.iter().enumerate() {
                if i + d.len() > line.len() {
                    continue;
                }
                if &line[i..d.len() + i] == *d {
                    last = j as u32;
                    break 'outer;
                }
            }
            if line.chars().nth(i).unwrap().is_digit(10) {
                last = line.chars().nth(i).unwrap().to_digit(10).unwrap();
                break;
            }
        }

        // println!("{} {}", first, last);
        sum += first * 10 + last;
    }

    sum
}
