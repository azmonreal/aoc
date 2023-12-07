use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 1: {}", part2(&contents));
}

fn part1(input: &String) -> u64 {
    let card_order: Vec<&str> = [
        "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
    ]
    .to_vec();

    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let hand = split.next().unwrap();
            let bet = split.next().unwrap().parse::<u64>().unwrap();
            let cards: HashMap<_, _> = hand.chars().fold(HashMap::new(), |mut acc, c| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

            let rank = match cards.len() {
                1 => &1,
                2 => {
                    if cards.values().any(|&x| x == 4) {
                        &2
                    } else {
                        &3
                    }
                }
                3 => {
                    if cards.values().any(|&x| x == 3) {
                        &4
                    } else {
                        &5
                    }
                }
                4 => &6,
                _ => &7,
            };
            (hand, bet, rank)
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.2 == b.2 {
            let mut a_o = 0;
            let mut b_o = 0;

            let mut index = 0;

            while a_o == b_o {
                a_o = card_order
                    .iter()
                    .position(|&x| x == a.0.chars().nth(index).unwrap().to_string())
                    .unwrap();
                b_o = card_order
                    .iter()
                    .position(|&x| x == b.0.chars().nth(index).unwrap().to_string())
                    .unwrap();

                index += 1;
            }

            b_o.cmp(&a_o)
        } else {
            b.2.cmp(a.2)
        }
    });

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, j)| acc + (i as u64 + 1) * j.1);

    // println!("{:?}", hands);

    winnings
}

fn part2(input: &String) -> u64 {
    let card_order: Vec<&str> = [
        "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
    ]
    .to_vec();

    let mut hands: Vec<_> = input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let hand = split.next().unwrap();
            let bet = split.next().unwrap().parse::<u64>().unwrap();
            let cards: HashMap<_, _> = hand.chars().fold(HashMap::new(), |mut acc, c| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

            let max = cards
                .clone()
                .into_iter()
                .filter(|(k, _)| k != &'J')
                .max_by(|a, b| a.1.cmp(&b.1));

            let j_count = cards.get(&'J').unwrap_or(&0);

            let real_max = match max {
                None => ('J', *j_count),
                Some((_, _)) => {
                    let old_max = max.unwrap();

                    (old_max.0, old_max.1 + j_count)
                }
            };

            let rank;

            match real_max.1 {
                5 => rank = 1,
                4 => rank = 2,
                3 => {
                    rank = match cards.len() {
                        2 => 3,
                        3 => 4-j_count,
                        _ => 4,
                    }
                }
                2 => {
                    if cards.len() == 3 {
                        rank = 5;
                    } else {
                        rank = 6;
                    }
                }
                _ => rank = 7,
            }

            (hand, bet, rank)
        })
        .collect();

    hands.sort_by(|a, b| {
        if a.2 == b.2 {
            let mut a_o = 0;
            let mut b_o = 0;

            let mut index = 0;

            while a_o == b_o {
                a_o = card_order
                    .iter()
                    .position(|&x| x == a.0.chars().nth(index).unwrap().to_string())
                    .unwrap();
                b_o = card_order
                    .iter()
                    .position(|&x| x == b.0.chars().nth(index).unwrap().to_string())
                    .unwrap();

                index += 1;
            }

            b_o.cmp(&a_o)
        } else {
            b.2.cmp(&a.2)
        }
    });

    let winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, j)| acc + (i as u64 + 1) * j.1);

    println!("{:?}", hands);

    winnings
}
