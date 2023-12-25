use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
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

fn part2(contents: &String) -> f64 {
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

    let p0 = hailstones[4].1.clone();
    let p1 = hailstones[3].1.clone();
    let p2 = hailstones[2].1.clone();
    let v0 = hailstones[4].0.clone();
    let v1 = hailstones[3].0.clone();
    let v2 = hailstones[2].0.clone();

    let A = vec![
        vec![
            v1[1] - v0[1],
            v0[0] - v1[0],
            0.0,
            p0[1] - p1[1],
            p1[0] - p0[0],
            0.0,
        ],
        vec![
            v2[1] - v0[1],
            v0[0] - v2[0],
            0.0,
            p0[1] - p2[1],
            p2[0] - p0[0],
            0.0,
        ],
        vec![
            v1[2] - v0[2],
            0.0,
            v0[0] - v1[0],
            p0[2] - p1[2],
            0.0,
            p1[0] - p0[0],
        ],
        vec![
            v2[2] - v0[2],
            0.0,
            v0[0] - v2[0],
            p0[2] - p2[2],
            0.0,
            p2[0] - p0[0],
        ],
        vec![
            0.0,
            v1[2] - v0[2],
            v0[1] - v1[1],
            0.0,
            p0[2] - p1[2],
            p1[1] - p0[1],
        ],
        vec![
            0.0,
            v2[2] - v0[2],
            v0[1] - v2[1],
            0.0,
            p0[2] - p2[2],
            p2[1] - p0[1],
        ],
    ];

    let b = vec![
        (p0[1] * v0[0] - p1[1] * v1[0]) - (p0[0] * v0[1] - p1[0] * v1[1]),
        (p0[1] * v0[0] - p2[1] * v2[0]) - (p0[0] * v0[1] - p2[0] * v2[1]),
        (p0[2] * v0[0] - p1[2] * v1[0]) - (p0[0] * v0[2] - p1[0] * v1[2]),
        (p0[2] * v0[0] - p2[2] * v2[0]) - (p0[0] * v0[2] - p2[0] * v2[2]),
        (p0[2] * v0[1] - p1[2] * v1[1]) - (p0[1] * v0[2] - p1[1] * v1[2]),
        (p0[2] * v0[1] - p2[2] * v2[1]) - (p0[1] * v0[2] - p2[1] * v2[2]),
    ];

    let inv = inverse_matrix(&A)
        .iter()
        .map(|v| v.iter().map(|v| *v).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rock = matrix_mutl(&inv, &b);

    let rock = rock.iter().map(|v| (v).round()).collect::<Vec<_>>();
    // println!("{:?}", rock);

    rock[3] + rock[4] + rock[5]
}

fn inverse_matrix(m: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut m = m.clone();
    let mut inv;

    // create the identity matrix (inv)
    inv = vec![vec![0.0; m.len()]; m.len()];
    for i in 0..m.len() {
        inv[i][i] = 1.0;
    }

    // perform row operations
    for i in 0..m.len() {
        // swap rows if we have a zero in the diagonal
        if m[i][i] == 0.0 {
            for j in (i + 1)..m.len() {
                if m[j][i] != 0.0 {
                    m.swap(i, j);
                    inv.swap(i, j);
                    break;
                }
            }
        }

        // divide row by the diagonal
        let div = m[i][i];
        for j in 0..m.len() {
            m[i][j] /= div;
            inv[i][j] /= div;
        }

        // subtract this row from the others to make everything else zero
        for j in 0..m.len() {
            if i == j {
                continue;
            }

            let sub = m[j][i];
            for k in 0..m.len() {
                m[j][k] -= m[i][k] * sub;
                inv[j][k] -= inv[i][k] * sub;
            }
        }
    }

    inv
}

fn matrix_mutl(m1: &Vec<Vec<f64>>, m2: &Vec<f64>) -> Vec<f64> {
    let mut result = vec![0.0; m1.len()];

    for i in 0..m1.len() {
        for j in 0..m1.len() {
            result[i] += m1[i][j] * m2[j];
        }
    }

    result
}
