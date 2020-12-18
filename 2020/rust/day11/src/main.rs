use std::collections::HashMap;
use std::io::{self, Read};

type Grid = Vec<Vec<char>>;

const OFFSETS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, -1),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

// I use i32s everywhere and now I'm not sure that is a good chocie

fn idxer(grid: &Grid, x: i32, y: i32) -> char {
    grid[y as usize][x as usize]
}

fn valid_idxs(x: i32, y: i32, n_rows: i32, n_cols: i32) -> bool {
    0 <= x && x < n_cols && 0 <= y && y < n_rows
}

fn next_value(grid: &Grid, x: i32, y: i32) -> char {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;
    let mut c = HashMap::new();

    // how do I write this more concisely in a rust way, not python
    for (dx, dy) in OFFSETS.iter() {
        let xdx = x + dx;
        let ydy = y + dy;
        if valid_idxs(xdx, ydy, n_rows, n_cols) {
            *c.entry(idxer(grid, xdx, ydy)).or_insert(0) += 1;
        }
    }

    let center = idxer(grid, x, y);

    let filled_count = c.get(&'#').unwrap_or(&0);
    if center == 'L' && *filled_count == 0 {
        '#'
    } else if center == '#' && *filled_count >= 4 {
        'L'
    } else {
        center
    }
}

fn next_value_extended(grid: &Grid, x: i32, y: i32) -> char {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;
    let mut c = HashMap::new();

    let first_seat = |x: i32, dx: i32, y: i32, dy: i32| {
        let mut nx = x.clone();
        let mut ny = y.clone();
        loop {
            nx += dx;
            ny += dy;
            if valid_idxs(nx, ny, n_rows, n_cols) {
                let v = idxer(grid, nx, ny);
                if v == '#' || v == 'L' {
                    return v;
                }
            } else {
                return '.'
            }
        }
    };
    for (dx, dy) in OFFSETS.iter() {
        *c.entry(first_seat(x, *dx, y, *dy)).or_insert(0) += 1;
    }

    let center = idxer(grid, x, y);

    let filled_count = c.get(&'#').unwrap_or(&0);
    if center == 'L' && *filled_count == 0 {
        '#'
    } else if center == '#' && *filled_count >= 5 {
        'L'
    } else {
        center
    }
}
fn step<F>(grid: &Grid, next: F) -> Grid
where
    F: Fn(&Grid, i32, i32) -> char,
{
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _c)| next(&grid, x as i32, y as i32))
                .collect()
        })
        .collect()
}

fn part_1(grid: &Grid) -> i32 {
    let mut curent_grid = grid.clone(); // can I not clone?
    let mut new_grid = step(grid, next_value);
    while new_grid != curent_grid {
        curent_grid = new_grid.clone();
        new_grid = step(&new_grid, next_value);
    }

    curent_grid.iter().flatten().filter(|&&c| c == '#').count() as i32
}

fn part_2(grid: &Grid) -> i32 {
    let mut curent_grid = grid.clone(); // can I not clone?
    let mut new_grid = step(grid, next_value_extended);
    while new_grid != curent_grid {
        curent_grid = new_grid.clone();
        new_grid = step(&new_grid, next_value_extended);
    }

    curent_grid.iter().flatten().filter(|&&c| c == '#').count() as i32
}

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();
    let grid: Grid = data.lines().map(|row| row.chars().collect()).collect();

    let answer_1 = part_1(&grid);
    println!("Part 1: {:?}", answer_1);

    let answer_2 = part_2(&grid);
    println!("Part 1: {:?}", answer_2);
}
