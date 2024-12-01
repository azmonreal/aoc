use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    let digs = contents
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            (
                parts[0].chars().nth(0).unwrap(),
                parts[1].parse::<usize>().unwrap(),
                parts[2],
            )
        })
        .collect::<Vec<_>>();
    // println!("{:?}", digs);

    let mut corners = vec![(0, 0)];

    digs.iter().for_each(|dig| {
        let last = corners.last().unwrap();
        let dir = match dig.0 {
            'R' => (1, 0),
            'L' => (-1, 0),
            'D' => (0, 1),
            'U' => (0, -1),
            _ => (0, 0),
        };
        let step = (dir.0 * dig.1 as i32, dir.1 * dig.1 as i32);
        corners.push((last.0 + step.0, last.1 + step.1));
    });
    // println!("{:?}", corners);

    let area = corners.windows(2).fold(0, |acc, corner| {
        acc + corner[0].0 * corner[1].1 - corner[0].1 * corner[1].0
    }) / 2;

    let boundry = corners.windows(2).fold(0, |acc, corner| {
        acc + corner[0].0.abs_diff(corner[1].0) + corner[0].1.abs_diff(corner[1].1)
    });

    let inner = (boundry as i32 / 2 + area + 1) as usize;

    inner
}

fn part2(contents: &String) -> usize {
    let digs = contents
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();

            (
                parts[0].chars().nth(0).unwrap(),
                parts[1].parse::<usize>().unwrap(),
                parts[2],
            )
        })
        .collect::<Vec<_>>();
    // println!("{:?}", digs);

    let mut corners = vec![(0, 0)];

    digs.iter().for_each(|dig| {
        let last = corners.last().unwrap();

        let hex = &dig.2[2..dig.2.len()-1].split_at(5);

        let hex_steps = i64::from_str_radix(hex.0, 16).unwrap();
        let hex_dir = usize::from_str_radix(hex.1, 10).unwrap();

        let dir = match hex_dir {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => (0, 0),
        };
        let step = (dir.0 * hex_steps as i64, dir.1 * hex_steps as i64);
        corners.push((last.0 + step.0, last.1 + step.1));
    });
    // println!("{:?}", corners);

    let area = corners.windows(2).fold(0, |acc, corner| {
        acc + corner[0].0 * corner[1].1 - corner[0].1 * corner[1].0
    }) / 2;

    let boundry = corners.windows(2).fold(0, |acc, corner| {
        acc + corner[0].0.abs_diff(corner[1].0) + corner[0].1.abs_diff(corner[1].1)
    });

    let inner = (boundry as i64 / 2 + area + 1) as usize;

    inner
}
