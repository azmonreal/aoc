use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let results = part1(&contents);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn part1(input: &str) -> (i32, i32) {
    let available = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut id_sum: i32 = 0;
    let mut power_sum: i32 = 0;

    for line in input.lines() {
        let mut is_possible = true;

        let mut p1 = line.to_string();
        let p2 = &p1.split_off(p1.find(':').unwrap())[1..];

        let id = &p1.split_off(p1.find(' ').unwrap())[1..];

        let sets = p2.split([',', ';']).collect::<Vec<&str>>();

        let mut max_found = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        for set in sets {
            let parts = set[1..].split(' ').collect::<Vec<&str>>();

            let count = parts[0].parse::<i32>().unwrap();
            let color = parts[1];

            if count > available[color] {
                is_possible = false;
            }

            *max_found.get_mut(color).unwrap() =
                std::cmp::max(*max_found.get(color).unwrap(), count);
        }

        let power = max_found.values().fold(1, |acc, x| acc * x);

        power_sum += power;

        if is_possible {
            id_sum += id.parse::<i32>().unwrap();
        }
    }

    (id_sum, power_sum)
}
