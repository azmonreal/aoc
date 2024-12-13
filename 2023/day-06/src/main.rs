use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let results = solve(&contents);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn solve(input: &String) -> (u64, u64) {
    let mut lines = input.lines();

    let time: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let distance: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let prod = time.iter().zip(distance.iter()).fold(1, |a, b| {
        let (&t, &d) = b.clone();

        let min = (1..t).find(|x| x * (t - x) > d).unwrap();
        let max = (1..t).rev().find(|x| x * (t - x) > d).unwrap();

        a * (max - min + 1)
    });

    let t: u64 = time
        .into_iter()
        .reduce(|a, b| a * u64::pow(10, b.ilog10() + 1) + b)
        .unwrap();

    let d: u64 = distance
        .into_iter()
        .reduce(|a, b| a * u64::pow(10, b.ilog10() + 1) + b)
        .unwrap();

    let min = (1..t).find(|x| x * (t - x) > d).unwrap();
    let max = (1..t).rev().find(|x| x * (t - x) > d).unwrap();

    let long = max - min + 1;

    (prod, long)
}
