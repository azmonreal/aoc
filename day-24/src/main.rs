use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> usize {
    let hailstones = contents
        .lines()
        .map(|line| line.split_once(" @ ").unwrap())
        .map(|(p, v)| {
            (
                p.split(", ")
                    .map(|v| v.trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>(),
                v.split(", ")
                    .map(|v| v.trim().parse::<f64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let equations: Vec<(f64, f64)> = hailstones
        .iter()
        .map(|(position, velocity)| {
            let m = velocity[1] / velocity[0];
            let b = position[1] - m * position[0];
            (m, b)
        })
        .collect();

    let bounding_box = (200000000000000.0, 400000000000000.0);

    let intersections = equations
        .iter()
        .enumerate()
        .map(|(i, eq)| {
            // println!("{}: {:?}", i, eq);

            equations[i + 1..]
                .iter()
                .enumerate()
                .filter_map(|(j, other)| {
                    // println!("  {}: {:?}", i + j + 1, other);

                    if eq.0 == other.0 {
                        return None;
                    }

                    let x = (eq.1 - other.1) / (other.0 - eq.0);
                    let y = eq.0 * x + eq.1;

                    // println!("    ({}, {})", x, y);

                    if x < bounding_box.0
                        || x > bounding_box.1
                        || y < bounding_box.0
                        || y > bounding_box.1
                        || (x < hailstones[i].0[0] && hailstones[i].1[0] > 0.0)
                        || (x > hailstones[i].0[0] && hailstones[i].1[0] < 0.0)
                        || (x < hailstones[i + j + 1].0[0] && hailstones[i + j + 1].1[0] > 0.0)
                        || (x > hailstones[i + j + 1].0[0] && hailstones[i + j + 1].1[0] < 0.0)
                    {
                        return None;
                    }

                    Some(((i, i + j + 1), (x, y)))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    // println!("{:?}", intersections);

    intersections.len()
}
