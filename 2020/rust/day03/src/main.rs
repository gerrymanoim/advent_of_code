use std::io::{self, BufRead};

fn trees_encountered(input_map: &Vec<Vec<char>>, slope_x: usize, slope_y: usize) -> usize {
    let mut cx = 0;
    let mut cy = 0;

    let max_x = input_map[0].len();
    let max_y = input_map.len();

    let mut points_to_check = Vec::new();

    while cy < max_y {
        points_to_check.push((cx, cy));
        cx = (cx + slope_x) % max_x;
        cy += slope_y;
    }

    points_to_check.iter()
        .map(|x| input_map[x.1][x.0])
        .filter(|x| *x == '#')
        .count()
}

fn part_1(v: &Vec<Vec<char>>) -> usize {
    trees_encountered(v, 3, 1)
}

fn part_2(v: &Vec<Vec<char>>) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|x| trees_encountered(v, x.0, x.1))
        .product()
}

fn main() {
    // Parse some things
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lock().lines() {
        data.push(line.unwrap().chars().collect());
    }

    // Part 1
    println!("{}", part_1(&data));

    // Part 2
    println!("{}", part_2(&data));
}
