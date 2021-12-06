use std::io::{self, BufRead};

fn part_1_first_attempt(init_state: &Vec<i32>, days_to_simulate: i32) -> usize {
    let mut state = init_state.clone();
    for _ in 1..=days_to_simulate {
        for idx in 0..state.len() {
            match state[idx] {
                0 => {
                    state[idx] = 6;
                    state.push(8);
                }
                _ => state[idx] -= 1,
            }
        }
    }
    state.len()
}

fn part_1(init_state: &Vec<i32>, days_to_simulate: i32) -> usize {
    // thought - represent contiguous values as "cells"
    let mut counter = [0; 9];
    for state in init_state {
        counter[*state as usize] += 1;
    }

    for _ in 1..=days_to_simulate {
        let mut new_counter = [0; 9];
        for idx in 0..counter.len() {
            if idx == 0 {
                new_counter[6] = counter[0];
                new_counter[8] = counter[0];
            } else {
                new_counter[idx-1] += counter[idx]
            }
        }
        counter = new_counter;
    }
    counter.iter().sum()
}

fn part_2() {}
fn main() {
    let init_state: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|s| {
            s.split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    println!("{:?}", part_1(&init_state, 80));
    println!("{:?}", part_1(&init_state, 256));

}

#[test]
fn examples() {
    let raw_input = "3,4,3,1,2";
    let init_state = raw_input
        .split(',')
        .map(|s| s.parse::<i32>().expect("broken"))
        .collect::<Vec<i32>>();

    assert_eq!(part_1(&init_state, 80), 5934);
    assert_eq!(part_1(&init_state, 256), 26984457539);
}
