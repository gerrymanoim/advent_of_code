extern crate utils;

use std::{num::ParseIntError, str::FromStr};

use utils::stdin_to_str_vec;

struct Assignment {
    start: usize,
    stop: usize,
}

impl FromStr for Assignment {
    type Err = ParseIntError; // TODO better error?
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, stop) = s.split_once("-").unwrap();
        let start_from_str = start.parse::<usize>()?;
        let stop_from_str = stop.parse::<usize>()?;

        Ok(Assignment {
            start: start_from_str,
            stop: stop_from_str,
        })
    }
}

type AssignmentPair = (Assignment, Assignment);

fn str_to_assignment_pair(s: &str) -> AssignmentPair {
    let (left, right) = s.split_once(",").unwrap();
    let left_from_str = left.parse().unwrap();
    let right_from_str = right.parse().unwrap();

    (left_from_str, right_from_str)
}

fn part_1(v: &Vec<AssignmentPair>) -> usize {
    v.iter()
        .map(|(left, right)| {
            if left.start <= right.start && right.stop <= left.stop {
                1
            } else if right.start <= left.start && left.stop <= right.stop {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part_2(v: &Vec<AssignmentPair>) -> usize {
    v.iter()
    .map(|(left, right)| {
        if left.start <= right.start && right.start <= left.stop {
            1
        } else if right.start <= left.start && left.start <= right.stop {
            1
        } else {
            0
        }
    })
    .sum()
}

fn main() {
    let assignments = stdin_to_str_vec()
        .iter()
        .map(|s| str_to_assignment_pair(s.as_str()))
        .collect();
    println!("{}", part_1(&assignments));
    println!("{}", part_2(&assignments));
}

#[test]
fn example() {
    let raw_input = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8"
        .split_whitespace()
        .map(str_to_assignment_pair)
        .collect();
    assert_eq!(part_1(&raw_input), 2);
    assert_eq!(part_2(&raw_input), 4);
}
