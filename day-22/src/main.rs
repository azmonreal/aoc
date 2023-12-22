use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> usize {
    let mut bricks = contents
        .lines()
        .map(|line| {
            line.split_once("~")
                .map(|(s, e)| {
                    (
                        s.split(",")
                            .map(|c| c.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                        e.split(",")
                            .map(|c| c.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    /* println!(
        "{}({})",
        bricks.iter().map(|b| {
            b.0.iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(",")
                + "~"
                + &b.1
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
        }).collect::<Vec<_>>().join("\n"),
        bricks.len()
    ); */

    collapse(&mut bricks);

    // println!("{:?}", bricks);

    let supports = supports(&bricks);
    // println!("{:?}", supports);

    let is_supported_by = is_supported_by(&bricks);
    // println!("{:?}", is_supported_by);

    let needed = is_supported_by
        .iter()
        .filter(|s| s.len() == 1)
        .map(|s| s[0])
        .collect::<HashSet<_>>();
    // println!("{:?}", needed);

    bricks.len() - needed.len()
}

fn collapse(bricks: &mut Vec<(Vec<usize>, Vec<usize>)>) {
    bricks.sort_by(|a, b| {
        let a = a.0[2].min(a.1[2]);
        let b = b.0[2].min(b.1[2]);
        a.cmp(&b)
    });

    for i in 0..bricks.len() {
        let curr = &bricks[i];

        // println!("{}: {:?}", i, curr);

        let ranges = (
            (curr.0[0].min(curr.1[0])..=curr.0[0].max(curr.1[0])),
            (curr.0[1].min(curr.1[1])..=curr.0[1].max(curr.1[1])),
            (curr.0[2].min(curr.1[2])..=curr.0[2].max(curr.1[2])),
        );

        if *ranges.2.start() == 1 {
            continue;
        }

        let bottom = ranges
            .0
            .clone()
            .map(|x| ranges.1.clone().map(move |y| (x, y)))
            .flatten()
            .collect::<Vec<_>>();

        // println!("{:?}", bottom);

        let mut z_diff = curr.0[2].min(curr.1[2]) - 1;

        for j in (0..i).rev() {
            let other = &bricks[j];
            // println!("checking: {}: {:?}", j, other);
            let other_ranges = (
                (other.0[0].min(other.1[0])..=other.0[0].max(other.1[0])),
                (other.0[1].min(other.1[1])..=other.0[1].max(other.1[1])),
                (other.0[2].min(other.1[2])..=other.0[2].max(other.1[2])),
            );

            let collision = bottom
                .iter()
                .any(|(x, y)| other_ranges.0.contains(x) && other_ranges.1.contains(y));

            if collision {
                let nz_diff = ranges.2.start() - other_ranges.2.end() - 1;
                if nz_diff < z_diff {
                    z_diff = nz_diff;
                }
                // println!("collision: {}: {:?} - {}", j, other, z_diff);
            }
        }

        bricks[i].0[2] -= z_diff;
        bricks[i].1[2] -= z_diff;
    }

    bricks.sort_by(|a, b| {
        let a = a.0[2].min(a.1[2]);
        let b = b.0[2].min(b.1[2]);
        a.cmp(&b)
    });
}

fn supports(bricks: &Vec<(Vec<usize>, Vec<usize>)>) -> Vec<Vec<usize>> {
    bricks
        .iter()
        .enumerate()
        .map(|(i, brick)| {
            // println!("checking: {} {:?}", i, brick);
            let ranges = (
                (brick.0[0].min(brick.1[0])..=brick.0[0].max(brick.1[0])),
                (brick.0[1].min(brick.1[1])..=brick.0[1].max(brick.1[1])),
                (brick.0[2].min(brick.1[2])..=brick.0[2].max(brick.1[2])),
            );
            let bottom = ranges
                .0
                .clone()
                .map(|x| ranges.1.clone().map(move |y| (x, y)))
                .flatten()
                .collect::<Vec<_>>();

            bricks[i + 1..]
                .iter()
                .enumerate()
                .filter_map(|(j, other)| {
                    let other_ranges = (
                        (other.0[0].min(other.1[0])..=other.0[0].max(other.1[0])),
                        (other.0[1].min(other.1[1])..=other.0[1].max(other.1[1])),
                        (other.0[2].min(other.1[2])..=other.0[2].max(other.1[2])),
                    );

                    if *other_ranges.2.start() == *ranges.2.end() + 1 {
                        // println!("comparing with: {} {:?}", j, other);
                        if bottom
                            .iter()
                            .any(|(x, y)| other_ranges.0.contains(x) && other_ranges.1.contains(y))
                        {
                            // println!("collision detected");
                            return Some(i + j + 1);
                        }
                    }
                    None
                })
                .collect()
        })
        .collect()
}

fn is_supported_by(bricks: &Vec<(Vec<usize>, Vec<usize>)>) -> Vec<Vec<usize>> {
    bricks
        .iter()
        .enumerate()
        .rev()
        .map(|(i, brick)| {
            // println!("checking: {} {:?}", i, brick);
            let ranges = (
                (brick.0[0].min(brick.1[0])..=brick.0[0].max(brick.1[0])),
                (brick.0[1].min(brick.1[1])..=brick.0[1].max(brick.1[1])),
                (brick.0[2].min(brick.1[2])..=brick.0[2].max(brick.1[2])),
            );
            let bottom = ranges
                .0
                .clone()
                .map(|x| ranges.1.clone().map(move |y| (x, y)))
                .flatten()
                .collect::<Vec<_>>();

            bricks[..i]
                .iter()
                .enumerate()
                .rev()
                .filter_map(|(j, other)| {
                    let other_ranges = (
                        (other.0[0].min(other.1[0])..=other.0[0].max(other.1[0])),
                        (other.0[1].min(other.1[1])..=other.0[1].max(other.1[1])),
                        (other.0[2].min(other.1[2])..=other.0[2].max(other.1[2])),
                    );

                    if *other_ranges.2.end() == *ranges.2.start() - 1 {
                        // println!("comparing with: {} {:?}", j, other);
                        if bottom
                            .iter()
                            .any(|(x, y)| other_ranges.0.contains(x) && other_ranges.1.contains(y))
                        {
                            // println!("collision detected");
                            return Some(j);
                        }
                    }
                    None
                })
                .collect()
        })
        .rev()
        .collect()
}
