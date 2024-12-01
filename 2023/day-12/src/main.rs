use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part2(&contents, 1));
    println!("Part 2: {}", part2(&contents, 5));
}

fn part1(contents: &String) -> usize {
    let sum = contents
        .lines()
        .map(|line| {
            // println!("*** {} ***", line);

            let (states, groups) = line.split_once(" ").unwrap();
            let groups = groups
                .split(",")
                .map(|g| g.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            // splitting does not take into account cases where there are multipe ending splits
            // where the last group could fit
            /* let states = states
            .split(".")
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>(); */

            let states = vec![states];

            let split_groups = states
                .iter()
                .fold((Vec::new(), 0), |mut acc, s| {
                    let i = (0..groups.len() - acc.1)
                        .find(|&i| groups[acc.1..=acc.1 + i].iter().sum::<usize>() + i > s.len())
                        .unwrap_or(groups.len() - acc.1);

                    acc.0.push(groups[acc.1..acc.1 + i].to_vec());
                    acc.1 += i;

                    acc
                })
                .0;

            let splits = states.iter().zip(split_groups.iter()).collect::<Vec<_>>();

            let arrangements = splits.into_iter().fold(1, |prod, (state, group)| {
                // println!("{} {:?}", state, group);

                let state_chars = state.chars().collect::<Vec<_>>();

                if group.len() == 0 {
                    return prod * 1;
                }

                let empty = state_chars.len() - (group.iter().sum::<usize>() + group.len() - 1);

                let possibilities = n_choose_j(group.len() + 1, empty)
                    .iter()
                    .map(|p| {
                        p.iter()
                            .enumerate()
                            .fold((String::new(), 0), |mut acc, (i, &x)| {
                                if i > 0 {
                                    acc.0.push_str(&"#".repeat(group[acc.1]));
                                    if acc.1 < group.len() - 1 {
                                        acc.0.push_str(&".");
                                    }
                                    acc.1 += 1;
                                }
                                if x > 0 {
                                    acc.0.push_str(&".".repeat(x));
                                }
                                acc
                            })
                            .0
                    })
                    .filter(|p| {
                        p.chars()
                            .enumerate()
                            .map(|(i, c)| {
                                if c == '.' && state_chars[i] == '#'
                                    || c == '#' && state_chars[i] == '.'
                                {
                                    'X'
                                } else {
                                    c
                                }
                            })
                            .all(|c| c != 'X')
                    })
                    .collect::<Vec<_>>();

                // println!("{:?}", possibilities);

                let combinations = possibilities.len();

                prod * combinations
            });

            // println!("Arrangements: {}", arrangements);

            // println!();

            arrangements
        })
        .sum::<usize>();

    sum
}

fn n_choose_j(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut table: HashMap<(usize, usize), Vec<Vec<usize>>> = HashMap::new();

    for j in 0..=k {
        table.insert((1, j), vec![vec![j]]);
    }

    for i in 2..=n {
        for j in 0..=k {
            let mut new_val = Vec::new();

            for nj in 0..=j {
                let temp = table.get(&(i - 1, nj)).unwrap().clone();

                for t in temp {
                    new_val.push(
                        vec![j - nj]
                            .iter()
                            .chain(t.iter())
                            .map(|&x| x)
                            .collect::<Vec<_>>(),
                    );
                }
            }

            table.insert((i, j), new_val);
        }
    }

    table.get(&(n, k)).unwrap().clone()
}

fn part2(contents: &String, repeats:usize ) -> usize {
    let sum = contents
        .lines()
        .map(|line| {
            // println!("*** {} ***", line);

            let (states, groups) = line.split_once(" ").unwrap();

            let groups = groups
                .split(",")
                .map(|g| g.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            let mut repeated = states.to_string();

            for _ in 0..repeats-1 {
                repeated.push('?');
                repeated.push_str(&states);
            }

            let groups = groups.repeat(repeats);

            let count = count_arrangements(&repeated, &groups);

            count
        })
        .sum::<usize>();

    sum
}

fn count_arrangements(state: &str, groups: &[usize]) -> usize {
    let mut cache: HashMap<(&str, &[usize]), usize> = HashMap::new();

    for i in 0..=state.len() {
        for j in 0..=groups.len() {
            let l_state = &state[state.len() - i..];
            let l_groups = &groups[groups.len() - j..];

            match cache.get(&(l_state, l_groups)) {
                Some(_) => {}
                None => {
                    let mut result = 0;

                    if l_state.len() == 0 {
                        if l_groups.len() == 0 {
                            result = 1;
                        } else {
                            result = 0;
                        }
                    } else if l_groups.len() == 0 {
                        if !l_state.chars().any(|c| c == '#') {
                            result = 1;
                        } else {
                            result = 0;
                        }
                    } else if l_groups.iter().sum::<usize>() + l_groups.len() - 1 > l_state.len() {
                        result = 0;
                    } else {
                        match l_state.chars().nth(0) {
                            Some(c) => {
                                if "?.".contains(c) {
                                    result += cache
                                        .get(&(&l_state[1..], l_groups))
                                        .unwrap();
                                }
                                if "?#".contains(c) {
                                    if l_groups[0] <= l_state.len()
                                        && !l_state[..l_groups[0]].contains('.')
                                    {
                                        match l_state.chars().nth(l_groups[0]) {
                                            Some(c) => {
                                                if c != '#' {
                                                    result += cache
                                                        .get(&(
                                                            &l_state[l_groups[0] + 1..],
                                                            &l_groups[1..],
                                                        ))
                                                        .unwrap();
                                                }
                                            }
                                            None => {
                                                result += cache
                                                    .get(&(&l_state[l_groups[0]..], &l_groups[1..]))
                                                    .unwrap();
                                            }
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                    }

                    cache.insert(
                        (
                            &l_state[l_state.len() - i..],
                            &l_groups[l_groups.len() - j..],
                        ),
                        result,
                    );
                }
            }
        }
    }

    cache.get(&(state, groups)).unwrap().clone()
}
