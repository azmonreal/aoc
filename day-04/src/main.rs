use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let numbers = line.to_string().split_off(line.find(":").unwrap() + 2);

        let split = numbers.split("|").collect::<Vec<&str>>();

        let winning = split[0]
            .split(" ")
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>();

        let yours = split[1]
            .split(" ")
            .filter(|c| !c.is_empty())
            .collect::<Vec<&str>>();

        let mut count = 0;
        for element in winning {
            if yours.contains(&element) {
                count += 1;
            }
        }

        if count > 0 {
            sum += u32::pow(2, count - 1);
        }
    }
    sum
}
