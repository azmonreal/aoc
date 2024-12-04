#[test]
fn test() {
    let (p1, p2) = solve(String::from(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    ));
    assert_eq!(p1, "18");
    assert_eq!(p2, "9");
}

fn dir_search(grid: &[Vec<char>], i: usize, j: usize, dir: (i32, i32), s: &str) -> bool {
    let mut ii = i as i32;
    let mut jj = j as i32;

    for c in s.chars() {
        if ii < 0
            || jj < 0
            || ii >= grid.len() as i32
            || jj >= grid[ii as usize].len() as i32
            || grid[ii as usize][jj as usize] != c
        {
            return false;
        }
        ii += dir.0;
        jj += dir.1;
    }

    true
}

pub fn solve(data: String) -> (String, String) {
    let mut grid = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for dir in &[
                (0, 1),
                (1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
                (-1, 0),
                (0, -1),
            ] {
                if dir_search(&grid, i, j, *dir, "XMAS") {
                    count += 1;
                }
            }
        }
    }

    let mut xcount = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            // 3 directions must be searched to cover all cases
            for dir in &[(1, 1), (-1, 1), (1, -1)] {
                if dir_search(&grid, i, j, *dir, "MAS") {
                    for idir in &[(-dir.0, dir.1), (dir.0, -dir.1)] {
                        if dir_search(
                            &grid,
                            (i as i32 + dir.0 - idir.0) as usize,
                            (j as i32 + dir.1 - idir.1) as usize,
                            *idir,
                            "MAS",
                        ) {
                            xcount += 1;
                            grid[(i as i32 + dir.0) as usize][(j as i32 + dir.1) as usize] = '-';
                        }
                    }
                }
            }
        }
    }

    (count.to_string(), xcount.to_string())
}
