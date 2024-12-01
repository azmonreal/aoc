use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2_2(&contents));
}

fn part1(contents: &String) -> usize {
    let platform = contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let mut cubes: Vec<(usize, usize)> = Vec::new();
    let mut rocks: Vec<(usize, usize)> = Vec::new();

    for (i, l) in platform.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    cubes.push((i, j));
                }
                'O' => {
                    rocks.push((i, j));
                }
                _ => {}
            }
        }
    }

    // print_platform((platform.len(), platform[0].len()), &rocks, &cubes);

    for i in 0..rocks.len() {
        while rocks[i].0 > 0
            && !cubes.contains(&(rocks[i].0 - 1, rocks[i].1))
            && !rocks.contains(&(rocks[i].0 - 1, rocks[i].1))
        {
            rocks[i].0 -= 1;
        }
    }

    // print_platform((platform.len(), platform[0].len()), &rocks, &cubes);

    let load = rocks.iter().map(|rock| platform.len() - rock.0).sum();

    load
}

fn part2(contents: &String) -> usize {
    let platform = contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    let mut cubes: Vec<(usize, usize)> = Vec::new();
    let mut rocks: Vec<(usize, usize)> = Vec::new();

    for (i, l) in platform.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    cubes.push((i, j));
                }
                'O' => {
                    rocks.push((i, j));
                }
                _ => {}
            }
        }
    }

    // print_platform((platform.len(), platform[0].len()), &rocks, &cubes);

    let cycles = 200;
    let mut loads: Vec<usize> = Vec::new();

    let mut start = 0;
    let mut end = 0;
    let mut in_cyle = false;
    let mut cycle = 0;

    loop {
        for dir in 0..4 {
            tilt(&mut rocks, &cubes, (platform.len(), platform[0].len()), dir);
            // print_platform((platform.len(), platform[0].len()), &rocks, &cubes);
        }
        let load: usize = rocks.iter().map(|rock| platform.len() - rock.0).sum();

        if !in_cyle {
            let prev = loads.iter().enumerate().find(|(i, &l)| l == load);

            match prev {
                Some(p) => {
                    in_cyle = true;
                    end = cycle;
                    start = p.0;
                }
                None => {}
            }
        } else {
            let diff = cycle - end;

            if loads[start + diff] != load {
                in_cyle = false;
            } else if cycle == end + (end - start) + 1 {
                break;
            }
        }
        loads.push(load);
        cycle += 1;
    }

    // println!("loads: {:?}", loads);

    let pattern = loads[start..end].to_vec();

    // println!("start: {}, end: {}", start, end);
    // println!("pattern: {:?}", pattern);

    let final_load = pattern[(1000000000 - 1 - start) % (end - start)];

    final_load
}
fn tilt(
    rocks: &mut Vec<(usize, usize)>,
    cubes: &Vec<(usize, usize)>,
    size: (usize, usize),
    dir: i32,
) {
    match dir {
        0 => {
            rocks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for i in 0..rocks.len() {
                while rocks[i].0 > 0
                    && !cubes.contains(&(rocks[i].0 - 1, rocks[i].1))
                    && !rocks.contains(&(rocks[i].0 - 1, rocks[i].1))
                {
                    rocks[i].0 -= 1;
                }
            }
        }
        2 => {
            rocks.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            for i in 0..rocks.len() {
                while rocks[i].0 < size.0 - 1
                    && !cubes.contains(&(rocks[i].0 + 1, rocks[i].1))
                    && !rocks.contains(&(rocks[i].0 + 1, rocks[i].1))
                {
                    rocks[i].0 += 1;
                }
            }
        }
        1 => {
            rocks.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            for i in 0..rocks.len() {
                while rocks[i].1 > 0
                    && !cubes.contains(&(rocks[i].0, rocks[i].1 - 1))
                    && !rocks.contains(&(rocks[i].0, rocks[i].1 - 1))
                {
                    rocks[i].1 -= 1;
                }
            }
        }
        3 => {
            rocks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            for i in 0..rocks.len() {
                while rocks[i].1 < size.1 - 1
                    && !cubes.contains(&(rocks[i].0, rocks[i].1 + 1))
                    && !rocks.contains(&(rocks[i].0, rocks[i].1 + 1))
                {
                    rocks[i].1 += 1;
                }
            }
        }
        _ => {}
    }
}

fn part2_2(contents: &String) -> usize {
    let mut platform = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let v = platform;

    let len = v[0].len();
    let mut iters= v.into_iter().rev().map(|n| n.into_iter()).collect::<Vec<_>>();

    platform = (0..len)
        .map(|_| {
             iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "{}",
        platform
            .iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );

    let temp = platform
        .iter()
        .map(|row| {
            row.split(|&c| c == '#')
                .map(|s| {
                    let mut temp = s.to_vec();
                    temp.sort();
                    temp
                })
                .collect::<Vec<_>>()
                .join(&'#')
        })
        .collect::<Vec<_>>();

    println!("{}", temp.iter().map(|s| s.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));

    0
}

fn print_platform(size: (usize, usize), rocks: &Vec<(usize, usize)>, cubes: &Vec<(usize, usize)>) {
    let mut platform = vec![vec!['.'; size.1]; size.0];

    for (i, j) in cubes {
        platform[*i][*j] = '#';
    }

    for (i, j) in rocks {
        platform[*i][*j] = 'O';
    }

    for l in platform {
        println!("{:?}", l);
    }
    println!();
}
