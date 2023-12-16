use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn part1(contents: &String) -> usize {
    let mirrors = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let mut splits: Vec<(Direction, (i32, i32))> = Vec::new();
    let mut history: HashSet<(Direction, (i32, i32))> = HashSet::new();
    let mut energyzed: HashSet<(i32, i32)> = HashSet::new();

    splits.push((Direction::Right, (0, -1)));

    while let Some((dir, pos)) = splits.pop() {
        // println!(
        //     "Split: {:?}, {:?} ({})",
        //     dir, pos, mirrors[pos.0 as usize][pos.1 as usize]
        // );

        if !history.insert((dir, pos)) {
            continue;
        }

        let next: Option<(char, (i32, i32))>;

        match dir {
            Direction::Up => {
                next = match mirrors[..pos.0 as usize]
                    .iter()
                    .map(|r| r[pos.1 as usize])
                    .collect::<Vec<_>>()
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, c)| "-\\/".contains(**c))
                {
                    Some((i, c)) => Some((*c, (pos.0 - (i + 1) as i32, pos.1))),
                    None => Some(('x', (0, pos.1)))
                }
            }
            Direction::Down => {
                next = match mirrors[pos.0 as usize + 1..]
                next = match mirrors[(pos.0 + 1) as usize..]
                    .iter()
                    .map(|r| r[pos.1 as usize])
                    .collect::<Vec<_>>()
                    .iter()
                    .enumerate()
                    .find(|(_, c)| "-\\/".contains(**c))
                {
                    Some((i, c)) => Some((*c, (i as i32 + pos.0 + 1, pos.1))),
                    None => Some(('x', (mirrors.len() as i32 - 1, pos.1)))
                }
            }
            Direction::Left => {
                next = match mirrors[pos.0 as usize][0..pos.1 as usize]
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_, c)| "|\\/".contains(**c))
                {
                    Some((i, c)) => Some((*c, (pos.0, pos.1 - (i+1) as i32))),
                    None => Some(('x', (pos.0, 0)))
                }
            }
            Direction::Right => {
                next = match mirrors[pos.0 as usize][(pos.1 + 1) as usize..]
                    .iter()
                    .enumerate()
                    .find(|(_, c)| "|\\/".contains(**c))
                {
                    Some((i, c)) => Some((*c, (pos.0, i as i32 + pos.1 + 1))),
                    None => Some(('x', (pos.0, mirrors[pos.0 as usize].len() as i32 - 1)))
                }
            }
        }

        match next {
            Some((c, pos)) => match c {
                '|' => {
                    splits.push((Direction::Up, pos));
                    splits.push((Direction::Down, pos));
                }
                '-' => {
                    splits.push((Direction::Right, pos));
                    splits.push((Direction::Left, pos));
                }
                '\\' => match dir {
                    Direction::Up => splits.push((Direction::Left, pos)),
                    Direction::Down => splits.push((Direction::Right, pos)),
                    Direction::Left => splits.push((Direction::Up, pos)),
                    Direction::Right => splits.push((Direction::Down, pos)),
                },
                '/' => match dir {
                    Direction::Up => splits.push((Direction::Right, pos)),
                    Direction::Down => splits.push((Direction::Left, pos)),
                    Direction::Left => splits.push((Direction::Down, pos)),
                    Direction::Right => splits.push((Direction::Up, pos)),
                },
                _ => {}
            },
            None => {}
        }

        let row_s = i32::min(pos.0, next.unwrap().1.0);
        let row_e = i32::max(pos.0, next.unwrap().1.0);

        let col_s = i32::min(pos.1, next.unwrap().1.1);
        let col_e = i32::max(pos.1, next.unwrap().1.1);

        for r in row_s..=row_e {
            for c in col_s..=col_e {
                energyzed.insert((r, c));
            }
        }

        // println!("{:?} - {:?}", (row_s, col_s), (row_e, col_e));


        // println!("Next: {:?}", next);
    }

    // println!("History: {:?}", history);

    energyzed.len()-1
}
