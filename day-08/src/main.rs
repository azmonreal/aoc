use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    let results = solve(&contents);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn solve(input: &String) -> (u64, u64) {
    let mut lines = input.lines();

    let instructions = lines.next().unwrap().chars();
    let mut inst_cycle = instructions.clone().cycle();

    let network: HashMap<_, (_, _)> = lines.skip(1).fold(HashMap::new(), |mut acc, line| {
        let (node, lr) = line.split_once(" = ").unwrap();
        let (l, r) = lr[1..lr.len() - 1].split_once(", ").unwrap();
        acc.insert(node, (l, r));
        acc
    });

    let mut count = 0;

    let mut curr = "AAA";

    while !curr.ends_with("Z") {
        curr = match inst_cycle.next().unwrap() {
            'L' => network.get(curr).unwrap().0,
            'R' => network.get(curr).unwrap().1,
            _ => panic!("Invalid instruction"),
        };

        count += 1;
    }

    let nodes = network
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    let mut counts = vec![0; nodes.len()];

    // println!("{:?}", nodes);

    for (i, node) in nodes.into_iter().enumerate() {
        let mut inst_cycle = instructions.clone().cycle();

        let mut current = node;

        while !current.ends_with('Z') {
            let next = inst_cycle.next().unwrap();

            current = match next {
                'L' => &network.get(*current).unwrap().0,
                'R' => &network.get(*current).unwrap().1,
                _ => panic!("Invalid instruction"),
            };

            counts[i] += 1;
        }
    }

    // println!("{:?}", counts);

    let counts_lcm = counts.iter().fold(counts[0], |acc, &count| {
        let mut a = u64::max(acc, count);
        let mut b = u64::min(acc, count);

        while b > 0 {
            a = a % b;
            std::mem::swap(&mut a, &mut b);
        }

        let gcd = a;

        acc * count / gcd
    });

    (count, counts_lcm)
}
