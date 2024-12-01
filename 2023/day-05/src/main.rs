use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
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
                    curr = *nstart + (curr - cstart);
                    break;
                }
            }
        }
        min = min.min(curr);
    }

    min
}

fn part2(input: &String) -> i128 {
    let order = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut map: HashMap<String, Vec<(i128, i128, i128)>> = HashMap::new();

    let mut lines = input.lines().filter(|line| !line.is_empty());

    let mut seeds_line = lines.next().unwrap().split(":");
    seeds_line.next();

    let seeds = seeds_line
        .next()
        .unwrap()
        .split(" ")
        .filter(|seed| !seed.is_empty())
        .collect::<Vec<&str>>()
        .windows(2)
        .step_by(2)
        .map(|window| {
            (
                window[0].parse::<i128>().unwrap(),
                window[1].parse::<i128>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

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
            .map(|val| val.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        map.get_mut(&current)
            .unwrap()
            .push((vals[0], vals[1], vals[2]));
    }

    let mut min = i128::MAX;

    let mut ranges = seeds.clone();
    for map_key in order.iter() {
        let mut new_ranges: Vec<(i128, i128)> = Vec::new();
        while let Some(range) = ranges.pop() {
            let (mut range_start, mut range_size) = range;
            for (next_start, current_start, current_size) in
                map.get(&map_key.to_string()).unwrap().iter()
            {
                let offset: i128 = *next_start - *current_start;

                let current_end = current_start + current_size - 1;

                let range_end = range_start + range_size - 1;

                if range_start < *current_start {
                    if range_end >= *current_start {
                        if range_end <= current_end {
                            new_ranges
                                .push((*current_start + offset, range_end - *current_start + 1));
                        } else {
                            new_ranges.push((*current_start + offset, *current_size));
                            ranges.push((current_end + 1, range_end - current_end));
                        }
                        range_size = *current_start - range_start;
                    }
                } else if range_start <= current_end {
                    if range_end > current_end {
                        new_ranges.push((range_start + offset, current_end - range_start + 1));
                        range_size = range_end - current_end;
                        range_start = current_end + 1;
                    } else {
                        range_start += offset;
                        break;
                    }
                }
            }
            new_ranges.push((range_start, range_size));
        }

        ranges = new_ranges;
    }
    // println!("{:?}", ranges);

    min = min.min(ranges.iter().min_by_key(|range| range.0).unwrap().0);

    min
}
