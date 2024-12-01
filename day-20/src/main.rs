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

fn part1(contents: &String) -> u32 {
    let mut m_types: HashMap<&str, char> = HashMap::new();

    let forward = contents
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();

            let m_type = p1.chars().nth(0).unwrap();

            let name = match m_type {
                'b' => p1,
                _ => &p1[1..],
            };

            let dest = p2.split(", ").collect::<Vec<_>>();

            m_types.insert(name, m_type);

            (name, dest)
        })
        .collect::<HashMap<_, _>>();

    let mut backward: HashMap<&str, Vec<&str>> = HashMap::new();

    forward.iter().for_each(|(name, dest)| {
        dest.iter().for_each(|d| {
            backward.entry(d).or_insert(vec![]).push(name);
        })
    });

    // println!("Types: {:?}", m_types);
    // println!("Forward: {:?}", forward);
    // println!("Backward: {:?}", backward);

    let mut flipflops = m_types
        .iter()
        .filter(|(_, v)| **v == '%')
        .map(|(&k, _)| (k, false))
        .collect::<HashMap<_, _>>();

    let mut conj = m_types
        .iter()
        .filter(|(_, v)| **v == '&')
        .map(|(&k, _)| {
            (
                k,
                backward
                    .get(k)
                    .unwrap()
                    .iter()
                    .map(|&b| (b, false))
                    .collect(),
            )
        })
        .collect::<HashMap<&str, HashMap<&str, bool>>>();

    // println!("Flipflops: {:?}", flipflops);
    // println!("Conjunctions: {:?}", conj);

    let mut next: VecDeque<_> = VecDeque::new();

    let (mut low, mut high) = (0, 0);

    for i in 1..=1000 {
        // println!("Cycle {}", i);
        next.push_back(("button", "broadcaster", false));

        while let Some((sender, dest, mut msg)) = next.pop_front() {
            if msg {
                high += 1;
            } else {
                low += 1;
            }

            // println!(
            //     "{} -{}-> {}",
            //     sender,
            //     if msg { "high" } else { "low" },
            //     dest
            // );

            match m_types.get(dest) {
                Some(t) => {
                    match t {
                        '%' => {
                            if msg {
                                continue;
                            }
                            flipflops.entry(dest).and_modify(|v| *v = !*v);
                            msg = flipflops.get(dest).unwrap().clone();
                        }
                        '&' => {
                            conj.get_mut(dest)
                                .unwrap()
                                .entry(sender)
                                .and_modify(|v| *v = msg);
                            msg = conj
                                .get(dest)
                                .unwrap()
                                .iter()
                                .any(|(_, &input)| input == false);
                        }
                        _ => {}
                    }
                    next.append(
                        &mut forward
                            .get(dest)
                            .unwrap()
                            .iter()
                            .map(|&v| (dest, v, msg))
                            .collect::<VecDeque<_>>(),
                    );
                }
                None => {}
            }
        }
    }

    // println!("Low: {}", low);
    // println!("High: {}", high);

    low * high
}

fn part2(contents: &String) -> u64 {
    let mut m_types: HashMap<&str, char> = HashMap::new();

    let forward = contents
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();

            let m_type = p1.chars().nth(0).unwrap();

            let name = match m_type {
                'b' => p1,
                _ => &p1[1..],
            };

            let dest = p2.split(", ").collect::<Vec<_>>();

            m_types.insert(name, m_type);

            (name, dest)
        })
        .collect::<HashMap<_, _>>();

    let mut backward: HashMap<&str, Vec<&str>> = HashMap::new();

    forward.iter().for_each(|(name, dest)| {
        dest.iter().for_each(|d| {
            backward.entry(d).or_insert(vec![]).push(name);
        })
    });

    let mut flipflops = m_types
        .iter()
        .filter(|(_, v)| **v == '%')
        .map(|(&k, _)| (k, false))
        .collect::<HashMap<_, _>>();

    let mut conj = m_types
        .iter()
        .filter(|(_, v)| **v == '&')
        .map(|(&k, _)| {
            (
                k,
                backward
                    .get(k)
                    .unwrap()
                    .iter()
                    .map(|&b| (b, false))
                    .collect(),
            )
        })
        .collect::<HashMap<&str, HashMap<&str, bool>>>();

    let mut next: VecDeque<_> = VecDeque::new();

    let magic_modules = forward
        .iter()
        .filter(|(_, ms)| ms.contains(forward.iter().find(|(_, m)| m.contains(&"rx")).unwrap().0))
        .count();

    let mut freq: HashMap<&str, u64> = HashMap::new();

    let mut count = 0;

    while freq.len() != magic_modules {
        count += 1;

        next.push_back(("button", "broadcaster", false));

        while let Some((sender, dest, mut msg)) = next.pop_front() {
            if forward.get(dest).unwrap_or(&vec![""]).contains(&"rx") && msg {
                // println!("{}:{} - {} ->  {}", count, sender, msg, dest);
                if !freq.contains_key(sender) {
                    freq.insert(sender, count);
                }
            }

            match m_types.get(dest) {
                Some(t) => {
                    match t {
                        '%' => {
                            if msg {
                                continue;
                            }
                            flipflops.entry(dest).and_modify(|v| *v = !*v);
                            msg = flipflops.get(dest).unwrap().clone();
                        }
                        '&' => {
                            conj.get_mut(dest)
                                .unwrap()
                                .entry(sender)
                                .and_modify(|v| *v = msg);
                            msg = conj
                                .get(dest)
                                .unwrap()
                                .iter()
                                .any(|(_, &input)| input == false);
                        }
                        _ => {}
                    }
                    next.append(
                        &mut forward
                            .get(dest)
                            .unwrap()
                            .iter()
                            .map(|&v| (dest, v, msg))
                            .collect::<VecDeque<_>>(),
                    );
                }
                None => {}
            }
        }
    }

     freq.values().map(|&v| v).reduce(|acc, f| { let var_name = acc * f; var_name }).unwrap()
}
