use std::collections::HashMap;
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

fn solve(contents: &String) -> (u32, u32) {
    let steps = contents.trim_end().split(",").collect::<Vec<_>>();

    let hash_sum = steps.iter().map(|step| hash(step)).sum();

    let labels = steps
        .iter()
        .map(|step| {
            let (label, length) = step.split_once(['-', '=']).unwrap();
            let length = length.parse::<u32>().unwrap_or(0);
            (label, length)
        })
        .collect::<Vec<_>>();

    let mut hash_map: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    labels.iter().for_each(|(label, length)| {
        match length {
            0 => match hash_map.get_mut(&hash(label)) {
                Some(inner) => match inner.iter().position(|e| e.0 == *label) {
                    Some(i) => {
                        inner.remove(i);
                    }
                    None => {}
                },
                None => {}
            },
            _ => match hash_map.get_mut(&hash(label)) {
                Some(inner) => match inner.iter().position(|e| e.0 == *label) {
                    Some(i) => {
                        inner[i].1 = *length;
                    }
                    None => {
                        inner.push((*label, *length));
                    }
                },
                None => {
                    let mut inner = Vec::new();
                    inner.push((*label, *length));
                    hash_map.insert(hash(label), inner);
                }
            },
        };
    });

    let power = hash_map
        .iter()
        .map(|bx| {
            bx.1.iter()
                .enumerate()
                .map(|(i, (_, l))| (bx.0 + 1) * (i as u32 + 1) * l)
                .sum::<u32>()
        })
        .sum();

    println!("{:?}", hash_map);

    (hash_sum, power)
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}
