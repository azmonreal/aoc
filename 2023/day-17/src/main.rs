use std::{
    cmp::Ordering,
    collections::{binary_heap, HashSet},
    env, fs,
};

fn main() {
    let file_path = env::args().nth(1).expect("Please provide a file path");
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");

    println!("Part 1: {}", part1(&contents));
    println!("Part 2: {}", part2(&contents));
}

#[derive(Debug, Eq, PartialEq)]
struct Cell {
    heat: u32,
    x: usize,
    y: usize,
    dx: i32,
    dy: i32,
    r: usize,
}

impl Cell {
    fn new(heat: u32, x: usize, y: usize, dx: i32, dy: i32, r: usize) -> Self {
        Self {
            heat,
            x,
            y,
            dx,
            dy,
            r,
        }
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(contents: &String) -> u32 {
    let grid = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut loss_grid = vec![vec![u32::MAX; grid[0].len()]; grid.len()];

    // print_grid(&grid);
    // println!();

    loss_grid[0][0] = 0;

    let mut unchecked: binary_heap::BinaryHeap<Cell> = binary_heap::BinaryHeap::new();

    let mut checked: HashSet<(usize, usize, i32, i32, usize)> = HashSet::new();

    unchecked.push(Cell::new(0, 0, 0, 0, 0, 0));

    while let Some(cell) = unchecked.pop() {
        let (heat, current) = (cell.heat, (cell.x, cell.y, cell.dx, cell.dy, cell.r));

        let (x, y, dx, dy, r) = current;

        if checked.contains(&current) {
            continue;
        }

        checked.insert(current);

        if heat < loss_grid[y][x] {
            loss_grid[y][x] = heat;
        }

        if x == grid[0].len() - 1 && y == grid.len() - 1 {
            break;
        }

        if r < 3 {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && (nx as usize) < grid[0].len() && ny >= 0 && (ny as usize) < grid.len() {
                unchecked.push(Cell::new(
                    heat + grid[ny as usize][nx as usize],
                    nx as usize,
                    ny as usize,
                    dx,
                    dy,
                    r + 1,
                ));
            }
        }

        let mut next_dirs = Vec::new();

        if dx == 0 {
            next_dirs.push((1, 0));
            next_dirs.push((-1, 0));
        }
        if dy == 0 {
            next_dirs.push((0, 1));
            next_dirs.push((0, -1));
        };

        for (ndx, ndy) in next_dirs {
            let nx = x as i32 + ndx;
            let ny = y as i32 + ndy;

            if nx >= 0 && (nx as usize) < grid[0].len() && ny >= 0 && (ny as usize) < grid.len() {
                unchecked.push(Cell::new(
                    heat + grid[ny as usize][nx as usize],
                    nx as usize,
                    ny as usize,
                    ndx,
                    ndy,
                    1,
                ));
            }
        }
    }

    // print_grid(&loss_grid);

    loss_grid[grid.len() - 1][grid[0].len() - 1]
}

fn part2(contents: &String) -> u32 {
    let grid = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut loss_grid = vec![vec![u32::MAX; grid[0].len()]; grid.len()];

    // print_grid(&grid);
    // println!();

    loss_grid[0][0] = 0;

    let mut unchecked: binary_heap::BinaryHeap<Cell> = binary_heap::BinaryHeap::new();

    let mut checked: HashSet<(usize, usize, i32, i32, usize)> = HashSet::new();

    unchecked.push(Cell::new(0, 0, 0, 0, 0, 10));

    while let Some(cell) = unchecked.pop() {
        let (heat, current) = (cell.heat, (cell.x, cell.y, cell.dx, cell.dy, cell.r));

        let (x, y, dx, dy, r) = current;

        if checked.contains(&current) {
            continue;
        }

        checked.insert(current);

        if heat < loss_grid[y][x] {
            loss_grid[y][x] = heat;
        }

        if x == grid[0].len() - 1 && y == grid.len() - 1 {
            // break;
        }

        if r < 10 {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && (nx as usize) < grid[0].len() && ny >= 0 && (ny as usize) < grid.len() {
                unchecked.push(Cell::new(
                    heat + grid[ny as usize][nx as usize],
                    nx as usize,
                    ny as usize,
                    dx,
                    dy,
                    r + 1,
                ));
            }
        }

        if r >= 4 {
            let mut next_dirs = Vec::new();

            if dx == 0 {
                next_dirs.push((1, 0));
                next_dirs.push((-1, 0));
            }
            if dy == 0 {
                next_dirs.push((0, 1));
                next_dirs.push((0, -1));
            };

            for (ndx, ndy) in next_dirs {
                let nx = x as i32 + ndx;
                let ny = y as i32 + ndy;

                if nx >= 0 && (nx as usize) < grid[0].len() && ny >= 0 && (ny as usize) < grid.len()
                {
                    unchecked.push(Cell::new(
                        heat + grid[ny as usize][nx as usize],
                        nx as usize,
                        ny as usize,
                        ndx,
                        ndy,
                        1,
                    ));
                }
            }
        }
    }

    // print_grid(&loss_grid);

    loss_grid[grid.len() - 1][grid[0].len() - 1]
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for col in row {
            print!("{}\t", col);
        }
        println!();
    }
}
