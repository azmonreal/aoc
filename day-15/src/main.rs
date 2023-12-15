use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> u32 {
    let steps = contents.trim_end().split(",").collect::<Vec<_>>();

    let sequence = steps
        .iter()
        .map(|step| {
            step.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
        })
        .collect::<Vec<_>>();

    let sum = sequence.iter().sum();

    sum
}
