use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(input: &str) -> i32 {
    let mut part_sum: i32 = 0;

    let mut part_num: i32 = 0;
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

    let cols = input.lines().nth(0).unwrap().chars().count() + 1;

    println!("chars: {}, cols:{}", input.chars().count(), cols);

    for (i, line) in input.lines().enumerate() {
        let chars = line.chars();
        for (j, character) in chars.enumerate() {
            if character.is_digit(10) {
                part_num = part_num * 10 + character.to_digit(10).unwrap() as i32;

                for cell in boundry {
                    let x = i as i32 + cell.0;
                    let y = j as i32 + cell.1;

                    if x < 0 || y < 0 || x >= input.lines().count() as i32 || y+1 >= cols as i32 {
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
                if has_symbol {
                    part_sum += part_num;
                }
                has_symbol = false;
                part_num = 0;
            }
        }
        if has_symbol {
            part_sum += part_num;
        }
        has_symbol = false;
        part_num = 0;
    }

    part_sum
}
