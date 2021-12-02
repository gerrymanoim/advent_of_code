use std::{
    io::{self, BufRead},
    panic,
};

// Calculate the horizontal position and depth you would have after following
// the planned course. What do you get if you multiply your final horizontal
// position by your final depth?
fn part_1(directions: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for instruction in directions {
        let mut parts = instruction.split(' ');
        let direction = parts.next().unwrap();
        let delta = parts.next().unwrap().parse::<i32>().unwrap();
        match direction {
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
fn part_2(directions: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in directions {
        let mut parts = instruction.split(' ');
        let direction = parts.next().unwrap();
        let delta = parts.next().unwrap().parse::<i32>().unwrap();
        match direction {
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
    let directions: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    println!("{}", part_1(&directions));
    println!("{}", part_2(&directions));
}

#[test]
fn examples() {
    // let raw_input: Vec<(&str, i32)> = "forward 5
    // down 5
    // forward 8
    // up 3
    // down 8
    // forward 2"
    // .lines()
    // .map(|l| l.split_whitespace().collect(()))
    // .map(|x|(x[0], x[1].parse::<i32>.unwrap()))
    // .collect();

    // assert_eq!(part_1(&raw_input), 7);
    // assert_eq!(part_2(&raw_input), 5);
}
