use std::{
    io::{self, BufRead},
    panic,
};

// Calculate the horizontal position and depth you would have after following
// the planned course. What do you get if you multiply your final horizontal
// position by your final depth?
fn part_1(directions: &Vec<(String, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for (direction, delta) in directions {
        match direction.as_str() {
            "forward" => horizontal += delta,
            "down" => depth += delta,
            "up" => depth -= delta,
            _ => panic!("No match for {}", direction),
        }
    }
    horizontal * depth
}

// Using this new interpretation of the commands, calculate the horizontal
// position and depth you would have after following the planned course. What
// do you get if you multiply your final horizontal position by your final
// depth?
fn part_2(directions: &Vec<(String, i32)>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (direction, delta) in directions {
        match direction.as_str() {
            "forward" => {
                horizontal += delta;
                depth += aim * delta;
            }
            "down" => aim += delta,
            "up" => aim -= delta,
            _ => panic!("No match for {}", direction),
        }
    }
    horizontal * depth
}

fn main() {
    let directions: Vec<(String, i32)> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|s| {
            let (l, r) = s.split_at(s.find(' ').unwrap());
            (l.to_string(), r[1..].parse::<i32>().unwrap())
        })
        .collect();

    println!("{}", part_1(&directions));
    println!("{}", part_2(&directions));
}

#[test]
fn examples() {
    let raw_input: Vec<(String, i32)> = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2"
        .lines()
        .map(|s| s.trim())
        .map(|s| s.split_at(s.find(' ').unwrap()))
        .map(|(l, r)| (l.to_string(), r[1..].parse::<i32>().unwrap()))
        .collect();

    assert_eq!(part_1(&raw_input), 150);
    assert_eq!(part_2(&raw_input), 900);
}
