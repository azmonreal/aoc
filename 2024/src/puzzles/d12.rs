use std::collections::{HashMap, HashSet};

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    ));
    assert_eq!(p1, "1930");
    assert_eq!(p2, "1206");
}

fn print_map(map: &[Vec<char>]) {
    let height = map.len();
    let width = map[0].len();

    let w_height = width.ilog(10) + 1;
    let h_height = height.ilog(10) + 1;

    for i in (0..w_height).rev() {
        print!("{}|", " ".repeat(h_height as usize + 1));
        for j in 0..width {
            let s = j.to_string();
            let digit = if i < s.len() as u32 {
                s.chars().nth(s.len() - i as usize - 1).unwrap()
            } else {
                ' '
            };
            print!("{}", digit);
        }
        println!();
    }

    println!("{}", "-".repeat(width + h_height as usize + 2));

    for (i, r) in map.iter().enumerate() {
        let leading_spaces = h_height - i.checked_ilog10().unwrap_or(0);
        println!(
            "{}{}|{}",
            " ".repeat(leading_spaces as usize),
            i,
            r.iter().collect::<String>()
        );
    }
}

pub fn solve(data: String) -> (String, String) {
    let map = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // print_map(&map);

    let plots: HashMap<char, Vec<(i32, i32)>> =
        map.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, l)| {
                l.iter().enumerate().for_each(|(j, c)| {
                    acc.entry(*c).or_default().push((i as i32, j as i32));
                });
                acc
            });

    let costs = plots.iter().fold((0, 0), |acc, (label, coords)| {
        let res = coords.iter().enumerate().fold(
            Vec::new(),
            |mut acc: Vec<(HashSet<(i32, i32)>, usize)>, (i, coord)| {
                let connections = coords[i..]
                    .iter()
                    .filter(|coord2| (coord.0 - coord2.0).abs() + (coord.1 - coord2.1).abs() == 1)
                    .collect::<Vec<_>>();

                let mut g = acc
                    .iter()
                    .position(|(group, _)| group.contains(coord))
                    .unwrap_or(acc.len());
                if g == acc.len() {
                    acc.push((HashSet::new(), 0));
                    acc[g].0.insert(*coord);
                }

                if let Some(g2) = acc
                    .iter()
                    .position(|(group, _)| connections.iter().any(|c| group.contains(c)))
                {
                    if g != g2 {
                        let combine = acc[g2].0.clone();
                        acc[g].0.extend(combine);
                        acc[g].1 += acc[g2].1;
                        acc.remove(g2);
                        if g > g2 {
                            g -= 1;
                        }
                    }
                }
                acc[g].1 += connections.len();
                acc[g].0.extend(connections.clone());

                acc
            },
        );

        let c = res.iter().fold((0, 0), |acc, (group, connections)| {
            let sides = group.iter().fold(0, |acc, coord| {
                /*
                 * check for corner pattern
                 *
                 *
                 * xx xx ox xo
                 * xo ox xx xx
                 *
                 *
                 * o. .o x. .x
                 * .x x. .o o.
                 *
                 * ox xo .x x.
                 * x. .x xo ox
                 *
                 * each found amounts to a side in the final shape
                 */
                let corners = [(-1, -1), (-1, 1), (1, -1), (1, 1)]
                    .iter()
                    .filter(|cd| {
                        (!group.contains(&(coord.0 + cd.0, coord.1))
                            && !group.contains(&(coord.0, coord.1 + cd.1))
                            && !group.contains(&(coord.0 + cd.0, coord.1 + cd.1)))
                            || (group.contains(&(coord.0 + cd.0, coord.1))
                                && group.contains(&(coord.0, coord.1 + cd.1))
                                && !group.contains(&(coord.0 + cd.0, coord.1 + cd.1)))
                            || (!group.contains(&(coord.0 + cd.0, coord.1))
                                && !group.contains(&(coord.0, coord.1 + cd.1))
                                && group.contains(&(coord.0 + cd.0, coord.1 + cd.1)))
                    })
                    .count();
                acc + corners
            });
            let a = group.len();
            if sides == 3 {
                println!(
                    "{}:{} area {} connections, {} sides",
                    label, a, connections, sides
                );
                println!("{:?}", group);
            }
            (acc.0 + (a * (a * 4 - connections * 2)), acc.1 + (a * sides))
        });
        (acc.0 + c.0, acc.1 + c.1)
    });

    (costs.0.to_string(), costs.1.to_string())
}
