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
    let adjecency_list =
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

    // println!("{:?}", adjecency_list);

    let names = adjecency_list.keys().map(|k| *k).collect::<Vec<_>>();

    // println!("{:?}", names);

    let mut adjecency_list: Vec<Vec<usize>> = adjecency_list
        .iter()
        .map(|(k, v)| {
            v.iter()
                .map(|child| names.iter().position(|x| x == child).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", adjecency_list);

    let mut edges: Vec<Vec<usize>> = vec![vec![0; names.len()]; names.len()];

    for node in 0..names.len() {
        // count_edges(node, &adjecency_list, &mut edges);
        count_edges_recursive(node, &adjecency_list, &mut edges, &mut vec![], &vec![]);
    }

    // println!("{:?}", edges);

    let mut sorted = edges
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, e)| ((i, j), e))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    sorted.sort_by(|(_, a), (_, b)| b.cmp(a));

    let cuts = &sorted[..3]
        .iter()
        .map(|((a, b), _)| (*a, *b))
        .collect::<Vec<_>>();

    println!(
        "{:?}",
        cuts.iter()
            .map(|(a, b)| (names[*a], names[*b]))
            .collect::<Vec<_>>()
    );

    cuts.iter().for_each(|cut| {
        adjecency_list
            .get_mut(cut.0)
            .unwrap()
            .retain(|x| *x != cut.1);
        adjecency_list
            .get_mut(cut.1)
            .unwrap()
            .retain(|x| *x != cut.0);
    });
    let mut unvisited: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    unvisited.push_back((0, vec![]));

    let mut visited = vec![];

    while let Some((next, mut path)) = unvisited.pop_front() {
        if !visited.contains(&next) {
            visited.push(next);
        }

        path.push(next);

        for neighbor in adjecency_list[next].iter() {
            if !visited.contains(neighbor) {
                unvisited.push_back((*neighbor, path.clone()));
                visited.push(*neighbor);
            }
        }
    }

    let count = adjecency_list.len() - visited.len();

    count * visited.len()
}

fn count_edges(start: usize, adjecency_list: &Vec<Vec<usize>>, edges: &mut Vec<Vec<usize>>) {
    let mut unvisited: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    unvisited.push_back((start, vec![]));

    let mut visited = vec![];

    while let Some((next, mut path)) = unvisited.pop_front() {
        if !visited.contains(&next) {
            visited.push(next);
        }

        path.push(next);

        for neighbor in adjecency_list[next].iter() {
            if !visited.contains(neighbor) {
                unvisited.push_back((*neighbor, path.clone()));
                visited.push(*neighbor);

                for edge in path.windows(2) {
                    edges[edge[0].min(edge[1])][edge[0].max(edge[1])] += 1;
                }
            }
        }
    }
}

fn count_edges_recursive(
    start: usize,
    adjecency_list: &Vec<Vec<usize>>,
    edges: &mut Vec<Vec<usize>>,
) {
}
