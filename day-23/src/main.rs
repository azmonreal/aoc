use std::{
    collections::{HashMap, HashSet, VecDeque},
    env, fs,
};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
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

fn part2(contents: &String) -> i32 {
    let maze = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = (0, maze[0].iter().position(|c| *c == '.').unwrap() as i32);
    let end = (
        maze.len() as i32 - 1,
        maze[maze.len() - 1].iter().position(|c| *c == '.').unwrap() as i32,
    );

    let nodes = maze
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, col)| {
                    if maze[i][j] != '#'
                        && vec![(0, -1), (0, 1), (1, 0), (-1, 0)]
                            .iter()
                            .filter(|dir| {
                                let next = (i as i32 + dir.0, j as i32 + dir.1);
                                next.0 > 0
                                    && next.1 > 0
                                    && next.0 < maze.len() as i32 - 1
                                    && next.1 < maze[0].len() as i32 - 1
                                    && maze[next.0 as usize][next.1 as usize] != '#'
                            })
                            .count()
                            > 2
                    {
                        Some((i as i32, j as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .chain(vec![start, end])
        .collect::<HashSet<_>>();

    let graph = nodes
        .iter()
        .map(|&n| (n, find_neighbors(&maze, &nodes, n)))
        .collect::<HashMap<_, _>>();

    // println!(
    //     "{}",
    //     graph
    //         .iter()
    //         .map(|(k, v)| format!("{:?}: {:?}", k, v))
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );

    explore_graph(&graph, &start, &end)
        .iter()
        .max()
        .unwrap()
        .clone()
}

fn find_neighbors(
    maze: &Vec<Vec<char>>,
    nodes: &HashSet<(i32, i32)>,
    n: (i32, i32),
) -> Vec<((i32, i32), i32)> {
    let mut neighbors = Vec::new();
    let mut unvisited = vec![(n, 0)];
    let mut visited = HashSet::new();

    while let Some((curr, dist)) = unvisited.pop() {
        if curr != n && nodes.contains(&curr) {
            neighbors.push((curr, dist));
            continue;
        }

        if visited.contains(&curr)
            || curr.0 < 0
            || curr.1 < 0
            || curr.0 >= maze.len() as i32
            || curr.1 >= maze[0].len() as i32
            || maze[curr.0 as usize][curr.1 as usize] == '#'
        {
            continue;
        }

        visited.insert(curr);

        vec![
            (curr.0, curr.1 - 1),
            (curr.0, curr.1 + 1),
            (curr.0 - 1, curr.1),
            (curr.0 + 1, curr.1),
        ]
        .iter()
        .for_each(|&next| unvisited.push((next, dist + 1)));
    }
    neighbors
}

fn explore_graph(
    graph: &HashMap<(i32, i32), Vec<((i32, i32), i32)>>,
    start: &(i32, i32),
    end: &(i32, i32),
) -> Vec<i32> {
    let mut exits = Vec::new();

    let mut unvisited = vec![(start, 0, HashSet::new())]
        .iter()
        .cloned()
        .collect::<VecDeque<_>>();

    while let Some((curr, count, mut visited)) = unvisited.pop_front() {
        visited.insert(*curr);

        graph[curr].iter().for_each(|(next, dist)| {
            if visited.contains(next) {
                return;
            }
            if next == end {
                exits.push(count + dist);
                return;
            }

            unvisited.push_back((next, count + dist, visited.clone()));
        });
    }

    exits
}
