use std::collections::HashSet;

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    ));
    assert_eq!(p1, "36");
    assert_eq!(p2, "81");
}

pub fn solve(data: String) -> (String, String) {
    let map = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let trailheads = map.iter().enumerate().fold(Vec::new(), |mut acc, (i, r)| {
        acc.append(
            &mut r
                .iter()
                .enumerate()
                .filter_map(|(j, &c)| if c == 0 { Some((i, j)) } else { None })
                .collect::<Vec<_>>(),
        );
        acc
    });

    // NOTE: msut be function, closures cant be recursive (easily)
    fn follow((i, j): (usize, usize), map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
        let curr = map[i][j];
        let next = [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter_map(|(x, y)| {
                let (nx, ny) = (i as i32 + x, j as i32 + y);
                if nx >= 0
                    && ny >= 0
                    && nx < map.len() as i32
                    && ny < map[0].len() as i32
                    && map[nx as usize][ny as usize] == curr + 1
                {
                    Some((nx as usize, ny as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        match next.is_empty() {
            true => {
                if curr == 9 {
                    Vec::from([(i, j)])
                } else {
                    Vec::new()
                }
            }
            false => next.iter().fold(Vec::new(), |mut acc, &n| {
                acc.extend(follow(n, map));
                acc
            }),
        }
    }

    let scores = trailheads.iter().fold((0, 0), |(a, b), &th| {
        let te = follow(th, &map);
        (
            a + HashSet::<(usize, usize)>::from_iter(te.clone()).len(),
            b + te.len(),
        )
    });

    (scores.0.to_string(), scores.1.to_string())
}
