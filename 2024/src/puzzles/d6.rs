use std::collections::HashSet;

#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    ));
    assert_eq!(p1, "41");
    assert_eq!(p2, "");
}

pub fn solve(data: String) -> (String, String) {
    let map: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let (mut guard, obstructions): ((i32, i32), Vec<(i32, i32)>) =
        map.iter()
            .enumerate()
            .fold(((0, 0), vec![]), |(mut g, mut o), (y, r)| {
                r.iter().enumerate().for_each(|(x, c)| match c {
                    '^' => g = (x as i32, y as i32),
                    '#' => o.push((x as i32, y as i32)),
                    _ => {}
                });
                (g, o)
            });

    let mut dir = (0, -1);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    fn in_bounds(p: (i32, i32), map: &[Vec<char>]) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < map[0].len() as i32 && p.1 < map.len() as i32
    }

    while in_bounds(guard, &map) {
        visited.insert(guard);
        let next = (guard.0 + dir.0, guard.1 + dir.1);
        if obstructions.contains(&next) {
            dir = (-dir.1, dir.0)
        }
        let next = (guard.0 + dir.0, guard.1 + dir.1);
        guard = next;
    }

    (visited.len().to_string(), String::from(""))
}
