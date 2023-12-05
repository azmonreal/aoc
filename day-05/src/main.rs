use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(input: &String) -> u64 {
    let mut seeds: Vec<u64> = Vec::new();

    let order = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut map: HashMap<String, Vec<(u64, u64, u64)>> = HashMap::new();

    let mut lines = input.lines().filter(|line| !line.is_empty());

    let mut seeds_line = lines.next().unwrap().split(":");
    seeds_line.next();

    for seed in seeds_line
        .next()
        .unwrap()
        .split(" ")
        .filter(|seed| !seed.is_empty())
    {
        seeds.push(seed.parse::<u64>().unwrap());
    }

    let mut current = String::new();

    for line in lines {
        if line.contains(":") {
            current = line.split(" ").next().unwrap().to_string();
            map.insert(current.clone(), Vec::new());
            continue;
        }

        let vals = line
            .split(" ")
            .filter(|val| !val.is_empty())
            .map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        map.get_mut(&current)
            .unwrap()
            .push((vals[0], vals[1], vals[2]));
    }

    let mut min = u64::MAX;
    for seed in seeds {
        let mut curr = seed;

        for map_key in order.iter() {
            let key = map_key.to_string();
            for (nstart, cstart, size) in map.get(&key).unwrap() {
                if cstart <= &curr && &curr <= &(cstart + size) {
                    curr = *nstart+(curr-cstart);
                    break;
                }
            }
        }
        min = min.min(curr);
    }

    min
}
