use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(input: &String) -> u64 {
    let mut prod = 1;

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


    for (t, d) in time.iter().zip(distance.iter()) {
        let mut min = 0;
        let mut max = 0;

        for i in 1..*t {
            if i * (t-i) > *d {
                min = i;
                break;
            }
        }
        for i in (1..*t).rev() {
            if i * (t-i) > *d {
                max = i;
                break;
            }
        }

        let count = max - min + 1;

        prod *= count;
    }
    prod
}
