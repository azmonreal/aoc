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

fn solve(contents: &String) -> (i32, i32) {
    let pipe_connections = HashMap::from([
        ('|', [(0, -1), (0, 1)]),
        ('-', [(-1, 0), (1, 0)]),
        ('L', [(0, -1), (1, 0)]),
        ('J', [(0, -1), (-1, 0)]),
        ('7', [(-1, 0), (0, 1)]),
        ('F', [(1, 0), (0, 1)]),
    ]);

    let mut pipes: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut pipes_loop = pipes.clone();

    let start_row = pipes_loop
        .iter()
        .enumerate()
        .find(|(_, line)| line.contains(&'S'))
        .unwrap()
        .0;

    let start_col = pipes_loop[start_row]
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    let start = (start_col as i32, start_row as i32);

    // println!("Start: {:?}", start);

    let mut positions = [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .map(|p| (start.0 + p.0, start.1 + p.1))
        .into_iter()
        .filter(|p| {
            p.0 >= 0
                && p.1 >= 0
                && p.0 < pipes_loop[0].len() as i32
                && p.1 < pipes_loop.len() as i32
                && pipes_loop[p.1 as usize][p.0 as usize] != '.'
                && pipe_connections
                    .get(&pipes_loop[p.1 as usize][p.0 as usize])
                    .unwrap()
                    .iter()
                    .map(|(x, y)| (p.0 + x, p.1 + y))
                    .any(|c| c == start)
        })
        .collect::<Vec<(i32, i32)>>();

    pipes[start.1 as usize][start.0 as usize] = match positions
        .iter()
        .map(|p| (p.0 - start.0, p.1 - start.1))
        .collect::<Vec<(i32, i32)>>()
        .as_slice()
    {
        [(0, -1), (0, 1)] => '|',
        [(-1, 0), (1, 0)] => '-',
        [(0, -1), (1, 0)] => 'L',
        [(0, -1), (-1, 0)] => 'J',
        [(-1, 0), (0, 1)] => '7',
        [(1, 0), (0, 1)] => 'F',
        _ => ' ',
    };

    // for i in 0..pipes.len() {
    //     for j in 0..pipes[0].len() {
    //         print!("{}", pipes[i][j]);
    //     }
    //     println!();
    // }

    while positions[0] != positions[1] {
        positions.iter_mut().for_each(|p| {
            let connections = pipe_connections
                .get(&pipes_loop[p.1 as usize][p.0 as usize])
                .unwrap()
                .iter()
                .map(|(x, y)| (p.0 + x, p.1 + y))
                .filter(|c| pipes_loop[c.1 as usize][c.0 as usize] != 'S')
                .collect::<Vec<(i32, i32)>>();

            pipes_loop[p.1 as usize][p.0 as usize] = 'S';

            p.0 = connections[0].0;
            p.1 = connections[0].1;
        });
    }

    pipes_loop[positions[0].1 as usize][positions[0].0 as usize] = 'S';

    // for i in 0..pipes_loop.len() {
    //     for j in 0..pipes_loop[0].len() {
    //         print!("{}", pipes_loop[i][j]);
    //     }
    //     println!();
    // }

    let loop_lenght = pipes_loop
        .iter()
        .map(|line| line.iter().filter(|c| **c == 'S').count())
        .sum::<usize>();

    let enclosed_tiles = pipes.iter().enumerate().fold(0, |acc, (i, row)| {
        let t = row.iter().enumerate().fold(
            ('|', false, 0),
            |(mut last, mut inside, mut count), (j, pipe)| {
                if pipes_loop[i][j] != 'S' {
                    count += if inside { 1 } else { 0 };
                } else if *pipe != '-' {
                    match pipe {
                        '|' | 'L' | 'F' => inside = !inside,
                        'J' => match last {
                            'L' => inside = !inside,
                            'F' => inside = inside,
                            _ => {}
                        },
                        '7' => match last {
                            'L' => inside = inside,
                            'F' => inside = !inside,
                            _ => {}
                        },
                        _ => {}
                    }
                    last = *pipe;
                }

                (last, inside, count)
            },
        );
        acc + t.2
    });

    ((loop_lenght / 2) as i32, enclosed_tiles)
}
