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
    let map = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| match row.iter().position(|&c| c == 'S') {
            Some(x) => Some((y as i32, x as i32)),
            None => None,
        })
        .unwrap();

    // println!("Start: {:?}", start);

    let steps = 64;
    let mut next = vec![(start, steps)];

    let mut prev: HashSet<((i32, i32), i32)> = HashSet::new();
    let mut spots: HashSet<_> = HashSet::new();

    while let Some((pos, steps)) = next.pop() {
        if pos.0 < 0
            || pos.0 >= map.len() as i32
            || pos.1 < 0
            || pos.1 >= map[pos.0 as usize].len() as i32
            || prev.contains(&(pos, steps))
        {
            continue;
        }

        prev.insert((pos, steps));

        match map[pos.0 as usize][pos.1 as usize] {
            '#' => {}
            _ => {
                if steps == 0 {
                    spots.insert(pos);
                } else {
                    next.push(((pos.0 + 1, pos.1), steps - 1));
                    next.push(((pos.0 - 1, pos.1), steps - 1));
                    next.push(((pos.0, pos.1 + 1), steps - 1));
                    next.push(((pos.0, pos.1 - 1), steps - 1));
                }
            }
        }
    }

    // println!("Spots: {:?}({})", spots, spots.len());

    spots.len()
}

fn part2(contents: &String) -> usize {
    let map = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let size = map.len() as i32;

    // println!("Size: {}({})", size, size / 2);

    let e = fill_grid(&map, (size / 2, size / 2), size as usize * 2);
    let o = fill_grid(&map, (size / 2, size / 2), size as usize * 2 + 1);

    let t = fill_grid(&map, (size - 1, size / 2), size as usize - 1);
    let b = fill_grid(&map, (0, size / 2), size as usize - 1);
    let l = fill_grid(&map, (size / 2, size - 1), size as usize - 1);
    let r = fill_grid(&map, (size / 2, 0), size as usize - 1);

    let ssize = (size - 1 - size / 2 - 1) as usize;
    let ts = fill_grid(&map, (size - 1, 0), ssize);
    let bs = fill_grid(&map, (0, 0), ssize);
    let ls = fill_grid(&map, (0, size - 1), ssize);
    let rs = fill_grid(&map, (size - 1, size - 1), ssize);

    let lsize = ssize + size as usize;
    let tl = fill_grid(&map, (size - 1, 0), lsize);
    let bl = fill_grid(&map, (0, 0), lsize);
    let ll = fill_grid(&map, (0, size - 1), lsize);
    let rl = fill_grid(&map, (size - 1, size - 1), lsize);

    let length = (26501365 / size) as usize;

    e * length.pow(2)
        + o * (length - 1).pow(2)
        + (ts + bs + ls + rs) * length
        + (tl + bl + ll + rl) * (length - 1)
        + t
        + b
        + l
        + r
}

fn fill_grid(map: &Vec<Vec<char>>, start: (i32, i32), steps: usize) -> usize {
    let mut next: VecDeque<_> = vec![(start, steps)].into_iter().collect();

    let mut prev: HashSet<_> = HashSet::new();
    let mut spots: HashSet<_> = HashSet::new();

    let even_steps = steps % 2 == 0;
    let even_start = start.0 % 2 != start.1 % 2;

    while let Some((pos, remaining)) = next.pop_front() {
        if pos.0 < 0
            || pos.0 >= map.len() as i32
            || pos.1 < 0
            || pos.1 >= map[pos.0 as usize].len() as i32
        {
            continue;
        }

        match map[pos.0 as usize][pos.1 as usize] {
            '#' => {}
            _ => {
                if prev.contains(&pos) {
                    continue;
                } else {
                    prev.insert(pos);

                    let even_cell = pos.0 % 2 != pos.1 % 2;

                    if even_steps {
                        if even_start == even_cell {
                            spots.insert(pos);
                        }
                    } else {
                        if even_start != even_cell {
                            spots.insert(pos);
                        }
                    }
                }

                if remaining > 0 {
                    next.push_back(((pos.0 + 1, pos.1), remaining - 1));
                    next.push_back(((pos.0 - 1, pos.1), remaining - 1));
                    next.push_back(((pos.0, pos.1 + 1), remaining - 1));
                    next.push_back(((pos.0, pos.1 - 1), remaining - 1));
                }
            }
        }
    }
    spots.len()
}
