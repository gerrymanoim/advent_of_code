use std::{collections::VecDeque, num::ParseIntError, str::FromStr};
extern crate utils;

use utils::stdin_to_str_vec;
struct MoveInstruction {
    quantity: usize,
    from_idx: usize,
    to_idx: usize,
}

impl FromStr for MoveInstruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (quantity_str, rest_str) = s
            .strip_prefix("move ")
            .and_then(|s| s.split_once(" from "))
            .unwrap();
        let (from_str, to_str) = rest_str.split_once(" to ").unwrap();
        let quantity = quantity_str.parse()?;
        let from_idx = from_str.parse()?;
        let to_idx = to_str.parse()?;

        Ok(MoveInstruction {
            quantity,
            from_idx,
            to_idx,
        })
    }
}
//[T]     [D]         [L]
//[P] [H] [P] [Q] [P] [M] [P] [F] [D]
// 1   2   3   4   5   6   7   8   9
fn make_stacks(v: &[String]) -> Vec<Vec<char>> {
    let n_stacks = v[0].len() + 1 / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];

    for line in v.iter().rev() {
        for (idx, chars) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if chars[0] == '[' {
                stacks[idx].push(chars[1]);
            }
        }
    }
    assert_eq!(stacks.len(), n_stacks);
    stacks
}

fn part_1(mut stacks: Vec<Vec<char>>, moves: &Vec<MoveInstruction>) -> String {
    let mut results: Vec<char> = Vec::new();
    for move_instruction in moves {
        let from_idx = move_instruction.from_idx - 1;
        let to_idx = move_instruction.to_idx - 1;
        for _ in 0..move_instruction.quantity {
            let tmp = stacks[from_idx].pop().unwrap();
            stacks[to_idx].push(tmp);
        }
    }
    for stack in stacks {
        if stack.len() > 0 {
            results.push(stack[stack.len() - 1])
        }
    }

    results.iter().collect()
}

fn part_2(mut stacks: Vec<Vec<char>>, moves: &Vec<MoveInstruction>) -> String {
    let mut results: Vec<char> = Vec::new();
    for move_instruction in moves {
        let from_idx = move_instruction.from_idx - 1;
        let to_idx = move_instruction.to_idx - 1;
        // ew
        let mut tmp_stack = Vec::new();
        for _ in 0..move_instruction.quantity {
            tmp_stack.push(stacks[from_idx].pop().unwrap());
        }
        while let Some(tmp) = tmp_stack.pop() {
            stacks[to_idx].push(tmp);
        }
    }
    for stack in stacks {
        if stack.len() > 0 {
            results.push(stack[stack.len() - 1])
        }
    }

    results.iter().collect()
}

fn main() {
    let input_lines = stdin_to_str_vec();
    let mut input_iter = input_lines.split(|s| s.is_empty());
    let stacks = make_stacks(input_iter.next().unwrap());
    let move_instructions = input_iter
        .next()
        .unwrap()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<MoveInstruction>>();

    println!("{}", part_1(stacks.clone(), &move_instructions));
    println!("{}", part_2(stacks.clone(), &move_instructions));
}
