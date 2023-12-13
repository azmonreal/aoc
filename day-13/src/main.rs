use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

fn part1(contents: &String) -> usize {
    let patterns = contents.split("\n\n").collect::<Vec<_>>();

    let sum = patterns.iter().enumerate().fold(0, |acc, (i, pattern)| {
        let rows = pattern
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let columns: Vec<String> = rows.iter().fold(Vec::new(), |mut acc, row| {
            row.chars().enumerate().for_each(|(i, c)| {
                if acc.len() <= i {
                    acc.push(String::new());
                }

                acc[i].push(c);
            });

            acc
        });

        let horizontals = rows
            .windows(2)
            .enumerate()
            .filter(|(_, rows)| rows[0] == rows[1])
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut h_count = 0;

        for &h in horizontals.iter() {
            let (mut left, mut right) = (
                h as i32,
                (h + if rows[h] == rows[h + 1] { 1 } else { 2 }) as i32,
            );
            // println!("left: {}, right: {}", left, right);

            let mut mirror = true;

            while left >= 0 && right < rows.len() as i32 {
                if rows[left as usize] != rows[right as usize] {
                    mirror = false;
                    // println!("{} != {}", rows[left as usize], rows[right as usize]);
                    break;
                }
                left -= 1;
                right += 1;
            }

            if mirror {
                h_count = h + 1;
            }
        }

        let vertical = columns
            .windows(2)
            .enumerate()
            .filter(|(_, columns)| columns[0] == columns[1])
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        let mut v_count = 0;
        for &v in vertical.iter() {
            let (mut top, mut bottom) = (
                v as i32,
                (v + if columns[v] == columns[v + 1] { 1 } else { 2 }) as i32,
            );
            // println!("top: {}, bottom: {}", top, bottom);

            let mut mirror = true;

            while top >= 0 && bottom < columns.len() as i32 {
                if columns[top as usize] != columns[bottom as usize] {
                    mirror = false;
                    // println!("{} != {}", columns[top as usize], columns[bottom as usize]);
                    break;
                }
                top -= 1;
                bottom += 1;
            }

            if mirror {
                v_count = v + 1;
            }
        }

        if v_count == 0 && h_count == 0 {
            println!("{}", i);
            println!("H: {:?}", horizontals);
            println!("V: {:?}", vertical);
            println!("{}", pattern);
        }

        acc + v_count + h_count * 100
    });

    sum
}

fn part2(contents: &String) -> usize {
    let patterns = contents.split("\n\n").collect::<Vec<_>>();

    let sum = patterns.iter().enumerate().fold(0, |acc, (_, pattern)| {
        // println!("{}", i);
        println!("{}", pattern);

        let rows = pattern
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let rows_range = (0..rows.len()).collect::<Vec<_>>();

        let horizontals = rows_range
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| {
                let mut fixed = false;
                let diff_count = rows[w[0]]
                    .chars()
                    .zip(rows[w[1]].chars())
                    .filter(|(a, b)| a != b)
                    .count();

                if diff_count > 1 {
                    return None;
                }

                if diff_count == 1 {
                    fixed = true;
                }
                if fixed || rows[w[0]] == rows[w[1]] {
                    let (mut top, mut bottom) = (i, i + 1);

                    let mut mirror = true;

                    while top > 0 && bottom < rows.len() - 1 {
                        if rows[top - 1] != rows[bottom as usize + 1] {
                            let diff_count = rows[top - 1]
                                .chars()
                                .zip(rows[bottom as usize + 1].chars())
                                .filter(|(a, b)| a != b)
                                .count();

                            if diff_count == 1 && !fixed {
                                fixed = true;
                            } else {
                                mirror = false;
                                break;
                            }
                        }
                        top -= 1;
                        bottom += 1;
                    }

                    if mirror && fixed{
                        return Some(w[0]);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        let columns: Vec<String> = rows.iter().fold(Vec::new(), |mut acc, row| {
            row.chars().enumerate().for_each(|(i, c)| {
                if acc.len() <= i {
                    acc.push(String::new());
                }

                acc[i].push(c);
            });

            acc
        });

        let cols_range = (0..columns.len()).collect::<Vec<_>>();
        let vertical = cols_range
            .windows(2)
            .enumerate()
            .filter_map(|(i, w)| {
                let mut fixed = false;
                let diff_count = columns[w[0]]
                    .chars()
                    .zip(columns[w[1]].chars())
                    .filter(|(a, b)| a != b)
                    .count();

                if diff_count > 1 {
                    return None;
                }

                if diff_count == 1 {
                    fixed = true;
                }
                if fixed || columns[w[0]] == columns[w[1]] {
                    let (mut top, mut bottom) = (i, i + 1);

                    let mut mirror = true;

                    while top > 0 && bottom < columns.len() - 1 {
                        if columns[top - 1] != columns[bottom as usize + 1] {
                            let diff_count = columns[top - 1]
                                .chars()
                                .zip(columns[bottom as usize + 1].chars())
                                .filter(|(a, b)| a != b)
                                .count();

                            if diff_count == 1 && !fixed {
                                fixed = true;
                            } else {
                                mirror = false;
                                break;
                            }
                        }
                        top -= 1;
                        bottom += 1;
                    }

                    if mirror && fixed{
                        return Some(w[0]);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        match horizontals.iter().min() {
            Some(&h) => {
                println!("H: {:?}", h);
                return acc + (h + 1) * 100;
            }
            None => match vertical.iter().min() {
                Some(&v) => {
                    println!("V: {:?}", v);
                    return acc + v + 1;
                }
                None => {
                    panic!("No solution found");
                }
            },
        }
    });

    sum
}
