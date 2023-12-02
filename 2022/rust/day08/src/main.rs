extern crate utils;
use std::collections::HashSet;

use utils::{Grid, Point};

fn check_coordinate<'a>(
    x: usize,
    y: usize,
    g: &'a Grid,
    visible_points: &mut HashSet<&'a Point>,
    max_value: i32,
) -> i32 {
    let p = g.get(x, y);
    if !visible_points.contains(p) {
        if p.value as i32 > max_value {
            visible_points.insert(p);
            return p.value as i32;
        }
    }
    if p.value as i32 > max_value {
        p.value as i32
    } else {
        max_value
    }
}

fn part_1<'a>(g: &'a Grid) -> usize {
    let mut visible_points: HashSet<&'a Point> = HashSet::new();

    for x in 0..g.width {
        let mut max_value = -1;
        for y in 0..g.height {
            max_value = check_coordinate(x, y, g, &mut visible_points, max_value);
            if max_value == 9 {
                break;
            }
        }
        max_value = -1;
        for y in (0..g.height).rev() {
            max_value = check_coordinate(x, y, g, &mut visible_points, max_value);
            if max_value == 9 {
                break;
            }
        }
    }
    for y in 0..g.height {
        let mut max_value = -1;
        for x in 0..g.width {
            max_value = check_coordinate(x, y, g, &mut visible_points, max_value);
            if max_value == 9 {
                break;
            }
        }
        max_value = -1;
        for x in (0..g.width).rev() {
            max_value = check_coordinate(x, y, g, &mut visible_points, max_value);
            if max_value == 9 {
                break;
            }
        }
    }
    visible_points.len()
}

fn calculate_score_for_point(x: usize, y: usize, g: &Grid) -> usize {
    let p = g.get(x, y);
    let mut north_score = 0;
    let mut east_score = 0;
    let mut south_score = 0;
    let mut west_score = 0;

    for dx in (0..x).rev() {
        west_score += 1;
        if g.get(dx, y).value >= p.value {
            break;
        }
    }
    for dx in x + 1..g.width {
        east_score += 1;
        if g.get(dx, y).value >= p.value {
            break;
        }
    }
    for dy in (0..y).rev() {
        north_score += 1;
        if g.get(x, dy).value >= p.value {
            break;
        }
    }
    for dy in y + 1..g.height {
        south_score += 1;
        if g.get(x, dy).value >= p.value {
            break;
        }
    }
    north_score * south_score * east_score * west_score
}

fn part_2(g: &Grid) -> usize {
    let mut tree_scores = Vec::new();
    for x in 0..g.width {
        for y in 0..g.height {
            let point_score = calculate_score_for_point(x, y, g);
            tree_scores.push(point_score);
        }
    }

    tree_scores.into_iter().max().expect("a max value")
}

fn main() {
    let input = include_str!("../../../inputs/input08.txt")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let grid = Grid::from_rows_vec(&input);
    println!("{}", part_1(&grid));
    println!("{}", part_2(&grid));
}

#[test]
fn test_1() {
    let input = "30373
    25512
    65332
    33549
    35390"
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let grid = Grid::from_rows_vec(&input);

    assert_eq!(calculate_score_for_point(2, 3, &grid), 8);
    assert_eq!(part_1(&grid), 21);
    assert_eq!(part_2(&grid), 8);
}
