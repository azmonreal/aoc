use std::{
    collections::{HashMap, VecDeque},
    env, fs,
};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
}

fn part1(contents: &String) -> usize {
    let mut adjecency_list =
        contents
            .lines()
            .fold(std::collections::HashMap::new(), |mut map, line| {
                let (parent, children) = line.split_once(": ").unwrap();
                let children = children.split(" ").collect::<Vec<_>>();

                map.entry(parent).or_insert(vec![]).extend(children.clone());

                children.iter().for_each(|child| {
                    map.entry(child).or_insert(vec![]).push(parent);
                });

                map
                // map.entry(parent).or_insert(vec![]).push(child);
                // map
            });

    let mut edges: HashMap<(&str, &str), usize> =
        adjecency_list
            .iter()
            .fold(HashMap::new(), |mut map, (k, v)| {
                v.iter().for_each(|child| {
                    map.entry((k.min(child), k.max(child))).or_insert(0);
                });
                map
            });

    // println!("{:?}{}", edges, edges.len());

    let mut c = 0;

    for node in adjecency_list.keys() {
        // if c > 0 {
        //     continue;
        // }
        let mut unvisited = VecDeque::new();

        unvisited.push_back((*node, vec![]));

        let mut visited = vec![];

        while let Some((next, mut path)) = unvisited.pop_front() {
            // println!("next: {}", next);
            if !visited.contains(&next) {
                visited.push(next);
            }

            path.push(next);

            for neighbor in adjecency_list.get(next).unwrap() {
                if !visited.contains(neighbor) {
                    unvisited.push_back((neighbor, path.clone()));
                    visited.push(neighbor);

                    for edge in path.windows(2) {
                        edges
                            .entry((edge[0].min(edge[1]), edge[0].max(edge[1])))
                            .and_modify(|e| *e += 1);
                    }
                    // edges
                    //     .entry((next.min(neighbor), next.max(neighbor)))
                    //     .and_modify(|e| *e += 1);
                }
            }
        }
        // c += 1;
    }

    let mut sorted = edges.iter().collect::<Vec<_>>();
    sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

    let cuts = &sorted[..3]
        .iter()
        .map(|((a, b), _)| (a, b))
        .collect::<Vec<_>>();

    // println!("{:?}", cuts);

    cuts.iter().for_each(|cut| {
        adjecency_list
            .entry(cut.0)
            .and_modify(|e| e.retain(|x| x != cut.1));
        adjecency_list
            .entry(cut.1)
            .and_modify(|e| e.retain(|x| x != cut.0));
    });

    let mut unvisited = VecDeque::new();
    unvisited.push_back(*adjecency_list.keys().next().unwrap());
    

    let mut visited = vec![];

    while let Some(next) = unvisited.pop_front() {
        if !visited.contains(&next) {
            visited.push(next);
        }

        for neighbor in adjecency_list.get(next).unwrap() {
            if !visited.contains(neighbor) {
                unvisited.push_back(*neighbor);
                visited.push(neighbor);
            }
        }
    }

    let count = adjecency_list.len() - visited.len();


    count * visited.len()
}
