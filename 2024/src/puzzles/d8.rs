use std::{
    collections::{HashMap, HashSet},
    iter::successors,
};

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    ));
    assert_eq!(p1, "14");
    assert_eq!(p2, "34");
}

pub fn solve(data: String) -> (String, String) {
    let map = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let freqs: HashMap<char, HashSet<(i32, i32)>> =
        map.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, l)| {
                l.iter().enumerate().for_each(|(j, &c)| {
                    if c != '.' {
                        acc.entry(c).or_default().insert((i as i32, j as i32));
                    }
                });
                acc
            });

    fn in_bounds(p: (i32, i32), map: &[Vec<char>]) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < map[0].len() as i32 && p.1 < map.len() as i32
    }

    let antinodes = freqs
        .iter()
        .fold(HashSet::<_>::new(), |mut acc, (_, positions)| {
            let pos_vec = positions.iter().collect::<Vec<_>>();
            pos_vec.iter().enumerate().for_each(|(i, p)| {
                pos_vec[i + 1..].iter().for_each(|p2| {
                    let d = (p.0 - p2.0, p.1 - p2.1);
                    let a1 = (p.0 + d.0, p.1 + d.1);
                    let a2 = (p2.0 - d.0, p2.1 - d.1);
                    if in_bounds(a1, &map) {
                        acc.insert(a1);
                    }
                    if in_bounds(a2, &map) {
                        acc.insert(a2);
                    }
                })
            });
            acc
        });

    let resonance = freqs
        .iter()
        .fold(HashSet::<_>::new(), |mut acc, (_, positions)| {
            let pos_vec = positions.iter().collect::<Vec<_>>();
            pos_vec.iter().enumerate().for_each(|(i, p)| {
                pos_vec[i + 1..].iter().for_each(|p2| {
                    let d = (p.0 - p2.0, p.1 - p2.1);

                    // let mut next = (p.0, p.1);
                    // while in_bounds(next, &map) {
                    //     acc.insert(next);
                    //     next = (next.0 + d.0, next.1 + d.1);
                    // }
                    // let mut prev = (p2.0, p2.1);
                    // while in_bounds(prev, &map) {
                    //     acc.insert(prev);
                    //     prev = (next.0 - d.0, next.1 - d.1);
                    // }

                    let next = successors(Some(**p), |(x, y)| Some((x + d.0, y + d.1))).take_while(|p| in_bounds(*p, &map)).collect::<HashSet<_>>();
                    acc.extend(next);

                    let prev = successors(Some(**p2), |(x, y)| Some((x - d.0, y - d.1))).take_while(|p| in_bounds(*p, &map)).collect::<HashSet<_>>();
                    acc.extend(prev);

                })
            });
            acc
        });

    (antinodes.len().to_string(), resonance.len().to_string())
}
