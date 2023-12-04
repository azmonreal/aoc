use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let result = solve(&contents);

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn solve(input: &str) -> (u32, u32) {
    let mut tickets_count: Vec<u32> = Vec::new();

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

        tickets_count.push(count);
    }

    let point_sum: u32 = tickets_count.iter().fold(0, |acc, x| {
        if x > &0 {
            u32::pow(2, x - 1) + acc
        } else {
            acc
        }
    });

    let mut copies: Vec<u32> = vec![1; tickets_count.len()];

    for (i, count) in tickets_count.into_iter().enumerate() {
        let copies_i = copies[i];
        if count > 0 {
            for copy in copies[i + 1..i + 1 + count as usize].iter_mut() {
                *copy += copies_i;
            }
        }
    }

    let copies_sum = copies.iter().sum::<u32>();

    (point_sum, copies_sum)
}
