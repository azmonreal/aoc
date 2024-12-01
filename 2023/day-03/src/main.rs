use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let result = solve(&contents);

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn solve(input: &str) -> (u32, u32) {
    let mut part_num: u32 = 0;

    let mut gear_sum: u32 = 0;

    let cols = input.lines().nth(0).unwrap().chars().count() + 1;

    let mut parts: Vec<(usize, usize, u32, bool)> = Vec::new();
    let mut gears: Vec<usize> = Vec::new();

    let mut start: i32 = -1;

    let mut has_symbol = false;

    let boundry = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (i, character) in input.chars().enumerate() {
        let x = (i / cols) as i32;
        let y = (i % cols) as i32;

        if character.is_digit(10) {
            if start == -1 {
                start = i as i32;
            }
            part_num = part_num * 10 + character.to_digit(10).unwrap() as u32;

            for cell in boundry {
                let x = x + cell.0;
                let y = y + cell.1;

                if x < 0 || y < 0 || x >= input.lines().count() as i32 || y + 1 >= cols as i32 {
                    continue;
                }

                let char_i = x as usize * cols + y as usize;

                let cell_char = input.chars().nth(char_i).unwrap();

                if !cell_char.is_digit(10) && cell_char != '.' {
                    has_symbol = true;
                    break;
                }
            }
        } else {
            if start != -1 {
                parts.push((start as usize, i, part_num, has_symbol));
                start = -1;
            }
            part_num = 0;
            has_symbol = false;

            if character == '*' {
                gears.push(i);
            }
        }
    }

    let symbol_sum = parts
        .iter()
        .fold(0, |acc, part| if part.3 { acc + part.2 } else { acc });

    for gear in gears {
        let x = (gear / cols) as i32;
        let y = (gear % cols) as i32;

        let mut p1 = -1;
        let mut p2 = -1;

        for cell in boundry {
            let char_i = (x + cell.0) as usize * cols + (y + cell.1) as usize;

            for (i, part) in parts.iter().enumerate() {
                if (part.0..part.1).contains(&char_i) {
                    if p1 == -1 {
                        p1 = i as i32;
                    } else if p1 != i as i32 {
                        p2 = i as i32;
                        break;
                    }
                }
            }
        }

        if p1 != -1 && p2 != -1 {
            gear_sum += parts[p1 as usize].2 * parts[p2 as usize].2;
        }
    }

    (symbol_sum, gear_sum)
}
