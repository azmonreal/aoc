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

fn solve(contents: &String) -> (i32, u64) {
    let original = contents
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let galaxies = original
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            let found = line
                .iter()
                .enumerate()
                .filter_map(|(j, &c)| {
                    if c == '#' {
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if found.len() > 0 {
                Some(found)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let row_size = (0..original.len())
        .map(|i| {
            if galaxies.iter().any(|&(x, _)| x == i ) {
                1
            } else {
                2
            }
        })
        .collect::<Vec<_>>();

    let col_size = (0..original[0].len())
        .map(|j| {
            if galaxies.iter().any(|&(_, y)| y == j) {
                1
            } else {
                2
            }
        })
        .collect::<Vec<_>>();

    let distances = galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy)| {
            galaxies[i + 1..]
                .iter()
                .map(|other| {
                    let smaller = (usize::min(galaxy.0, other.0), usize::min(galaxy.1, other.1));
                    let larger = (usize::max(galaxy.0, other.0), usize::max(galaxy.1, other.1));

                    row_size[smaller.0..larger.0]
                        .iter()
                        .sum::<usize>()
                        + col_size[smaller.1..larger.1]
                            .iter()
                            .sum::<usize>()

                    // (galaxy.0 - other.0).abs() + (galaxy.1 - other.1).abs()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = distances
        .iter()
        .map(|row| row.iter().sum::<usize>())
        .sum::<usize>() as i32;

    let row_size = row_size.iter().map(|&x| if x == 1 { x} else {1000000}).collect::<Vec<_>>();
    let col_size = col_size.iter().map(|&x| if x == 1 { x} else {1000000}).collect::<Vec<_>>();

    let distances = galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy)| {
            galaxies[i + 1..]
                .iter()
                .map(|other| {
                    let smaller = (usize::min(galaxy.0, other.0), usize::min(galaxy.1, other.1));
                    let larger = (usize::max(galaxy.0, other.0), usize::max(galaxy.1, other.1));

                    (row_size[smaller.0..larger.0]
                        .iter()
                        .sum::<usize>()
                        + col_size[smaller.1..larger.1]
                            .iter()
                            .sum::<usize>()) as u64

                    // (galaxy.0 - other.0).abs() + (galaxy.1 - other.1).abs()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let large_sum = distances
        .iter()
        .map(|row| row.iter().sum::<u64>())
        .sum::<u64>();


    (sum, large_sum)
}
