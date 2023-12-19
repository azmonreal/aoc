use std::{arch::x86_64::__cpuid, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> u32 {
    let (workflows, parts) = contents
        .split_once("\n\n")
        .map(|(w, p)| {
            let ws = w
                .lines()
                .map(|l| {
                    let (name, rules) = l.split_at(l.find("{").unwrap());
                    (
                        name,
                        rules[1..rules.len() - 1].split(",").collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>();

            let ps = p
                .lines()
                .map(|l| {
                    let split = l[1..l.len() - 1]
                        .split(",")
                        .map(|s| s.split_once("=").unwrap().1.parse::<u32>().unwrap())
                        .collect::<Vec<_>>();
                    split
                })
                .collect::<Vec<_>>();
            (ws, ps)
        })
        .unwrap();

    // println!("{:?}", workflows);
    // println!("{:?}", parts);

    let mut sum = 0;

    for part in parts.iter() {
        // println!("Part: {:?}", part);
        let mut current = workflows.iter().find(|w| w.0 == "in").unwrap();
        let mut state = 0;

        while state == 0 {
            for rule in current.1.iter() {
                // println!("Rule: {}", rule);
                match rule.split_once(":") {
                    Some((cmp, dest)) => {
                        let c_cat = match cmp.chars().nth(0).unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!(),
                        };

                        let symb = match cmp.chars().nth(1).unwrap() {
                            '<' => 0,
                            '>' => 1,
                            _ => panic!(),
                        };

                        let val = cmp[2..].parse::<u32>().unwrap();

                        if match symb {
                            0 => part[c_cat] < val,
                            1 => part[c_cat] > val,
                            _ => panic!(),
                        } {
                            match dest {
                                "R" => state = 1,
                                "A" => state = 2,
                                _ => current = workflows.iter().find(|w| w.0 == dest).unwrap(),
                            }
                            break;
                        }
                    }
                    None => {
                        match *rule {
                            "R" => state = 1,
                            "A" => state = 2,
                            _ => current = workflows.iter().find(|w| w.0 == *rule).unwrap(),
                        };
                        break;
                    }
                }
            }
        }

        if state == 2 {
            sum += part.iter().sum::<u32>();
        }
    }

    sum
}
