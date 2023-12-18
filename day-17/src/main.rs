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
}

#[derive(Debug, Eq, PartialEq)]
struct Cell {
    heat: u32,
    x: usize,
    y: usize,
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

impl Cell {
    fn new(
        heat: u32,
        x: usize,
        y: usize,
        up: usize,
        down: usize,
        left: usize,
        right: usize,
    ) -> Self {
        Self {
            heat,
            x,
            y,
            up,
            down,
            left,
            right,
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
    println!();

    loss_grid[0][0] = 0;

    let mut unchecked: binary_heap::BinaryHeap<Cell> = binary_heap::BinaryHeap::new();

    let mut checked: HashSet<(usize, usize, usize, usize, usize, usize)> = HashSet::new();

    unchecked.push(Cell::new(0, 0, 0, 3, 3, 3, 3));

    while let Some(cell) = unchecked.pop() {
        let (heat, current) = (
            cell.heat,
            (cell.x, cell.y, cell.up, cell.down, cell.left, cell.right),
        );

        let (x, y, up, down, left, right) = current;

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

        if x > 0 && left > 0 {
            unchecked.push(Cell::new(heat + grid[y][x - 1], x - 1, y, 3, 3, left - 1, 0));
        }

        if x < grid[0].len() - 1 && right > 0 {
            unchecked.push(Cell::new(heat + grid[y][x + 1], x + 1, y, 3, 3, 0, right - 1));
        }

        if y > 0 && up > 0 {
            unchecked.push(Cell::new(heat + grid[y - 1][x], x, y - 1, up - 1, 0, 3, 3));
        }

        if y < grid.len() - 1 && down > 0 {
            unchecked.push(Cell::new(heat + grid[y + 1][x], x, y + 1, 0, down - 1, 3, 3));
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
