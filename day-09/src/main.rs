use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let (part1, part2) = solve(&contents);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(input: &String) -> (i64, i64) {
    let readings = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|reading| reading.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", readings);

    let mut predictions = Vec::new();
    let mut b_predictions = Vec::new();

    for reading in readings.iter() {
        let mut differences: Vec<Vec<i64>> = Vec::new();

        differences.push(
            reading
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>(),
        );

        differences.push(
            differences[0]
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect::<Vec<_>>(),
        );

        while differences.last().unwrap().iter().sum::<i64>() != 0 {
            differences.push(
                differences
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<_>>(),
            );
        }

        println!("{:?}", differences);

        predictions.push(
            differences.iter().fold(0, |acc, b| acc + b.last().unwrap()) + reading.last().unwrap(),
        );

        b_predictions.push(
            differences.iter().enumerate().fold(*reading.first().unwrap(), |acc, (i, b)| acc + b.first().unwrap() * i64::pow(-1, i as u32 + 1))
        );
    }

    // println!("{:?}", predictions);

    let sum = predictions.iter().sum::<i64>();
    let b_sum = b_predictions.iter().sum::<i64>();

    (sum,b_sum)
}
