use std::io::{self, BufRead};

fn binary_partition(
    mut left_bound: i32,
    mut right_bound: i32,
    direction: char,
    remaining_steps: &[char],
) -> i32 {
    if ['F', 'L'].contains(&direction) {
        right_bound = (left_bound + right_bound) / 2;
    } else {
        left_bound = ((left_bound as f32 + right_bound as f32) / 2f32).ceil() as i32;
    }

    if left_bound == right_bound {
        return right_bound;
    } else {
        return binary_partition(
            left_bound,
            right_bound,
            remaining_steps[0],
            &remaining_steps[1..],
        );
    }
}

fn part_1(v: &Vec<Vec<char>>) -> Option<i32> {
    return v
        .iter()
        .map(|x| {
            binary_partition(0, 127, x[0], &x[1..7]) * 8 + binary_partition(0, 7, x[7], &x[8..])
        })
        .max();
}

fn part_2(v: &Vec<Vec<char>>) -> Option<i32> {
    let known_ids: Vec<i32> = v
        .iter()
        .map(|x| {
            binary_partition(0, 127, x[0], &x[1..7]) * 8 + binary_partition(0, 7, x[7], &x[8..])
        })
        .collect();
    let min_id = known_ids.iter().min().unwrap();
    let max_id = known_ids.iter().max().unwrap();

    return (*min_id..*max_id).filter(|x| !known_ids.contains(x)).next();
}

fn main() {
    // Parse some things
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lock().lines() {
        data.push(line.unwrap().chars().collect());
    }

    // Part 1
    let answer_1 = part_1(&data);
    match answer_1 {
        Some(x) => println!("Part 1: {}", x),
        None => println!("Something went' wrong in part 1"),
    }

    // Part 2
    let answer_2 = part_2(&data);
    match answer_2 {
        Some(x) => println!("Part 2: {}", x),
        None => println!("Something went' wrong in part 2"),
    }
}
