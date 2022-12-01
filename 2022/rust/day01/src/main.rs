extern crate utils;

use utils::stdin_to_str_vec;

fn part_1(v: &Vec<String>) -> usize {
    v.split(|c| c.is_empty())
        .map(|v| v.iter().map(|s| s.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part_2(v: &Vec<String>) -> usize {
    let mut totals: Vec<usize> = v
        .split(|c| c.is_empty())
        .map(|v| v.iter().map(|s| s.parse::<usize>().unwrap()).sum())
        .collect();
    totals.sort_by(|a, b| b.cmp(a));
    totals.iter().take(3).sum()
}

fn main() {
    let calories = stdin_to_str_vec();
    println!("{}", part_1(&calories));
    println!("{}", part_2(&calories));
}
