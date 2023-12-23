use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> usize {
    let maze = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut next: Vec<(i32, i32, usize, Vec<(i32, i32)>)> = Vec::new();
    next.push((0, 1, 0, Vec::new()));

    let mut visited: HashMap<(i32, i32), usize> = HashMap::new();

    let mut end = 0;

    while let Some((row, col, count, path)) = next.pop() {
        if row < 0
            || col < 0
            || maze[row as usize][col as usize] == '#'
            || path.contains(&(row, col))
        {
            continue;
        }

        if visited.contains_key(&(row, col)) {
            if visited[&(row, col)] > count {
                continue;
            }
            visited.entry((row, col)).and_modify(|e| *e = count);
        } else {
            visited.insert((row, col), count);
        }

        if row == maze.len() as i32 - 1 && col == maze[0].len() as i32 - 2 {
            end = end.max(count);
            continue;
        }

        let mut t_visited = path;
        t_visited.push((row, col));

        let dirs = match maze[row as usize][col as usize] {
            '^' => vec![(0, -1), (0, 1), (-1, 0)],
            'v' => vec![(0, -1), (0, 1), (1, 0)],
            '>' => vec![(0, 1), (1, 0), (-1, 0)],
            '<' => vec![(0, -1), (1, 0), (-1, 0)],
            _ => vec![(0, -1), (0, 1), (1, 0), (-1, 0)],
        };

        for dir in dirs {
            next.push((
                row as i32 + dir.0,
                col as i32 + dir.1,
                count + 1,
                t_visited.clone(),
            ));
        }
    }

    end
}
