use std::{collections::HashMap, num::ParseIntError, str::FromStr};

enum Flavor {
    Addx,
    Noop,
}
struct Instruction {
    flavor: Flavor,
    value: Option<i32>,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((_, value_str)) => Ok(Instruction {
                flavor: Flavor::Addx,
                value: Some(value_str.parse::<i32>()?),
            }),
            None => Ok(Instruction {
                flavor: Flavor::Noop,
                value: None,
            }),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Status {
    Busy,
    Ready,
}

struct State {
    register_x: i32,
    cycle: usize,
    status: Status,
    scheduled: HashMap<usize, i32>,
}

impl State {
    fn new() -> Self {
        State {
            register_x: 1,
            cycle: 1,
            status: Status::Ready,
            scheduled: HashMap::new(),
        }
    }
    fn tick(&mut self) {
        self.cycle += 1;
        match self.scheduled.get(&self.cycle) {
            Some(x) => {
                self.register_x = *x;
                self.status = Status::Ready;
            }
            None => {}
        }
    }
    fn schedule_set(&mut self, cycle: usize, value: i32) {
        self.scheduled.insert(cycle, value);
        self.status = Status::Busy;
    }
}

fn part_1(instructions: &Vec<Instruction>) -> i32 {
    let mut state = State::new();
    let mut instruction_pntr: usize = 0;
    let mut result = 0;
    let sample_points: [usize; 6] = [20, 60, 100, 140, 180, 220];
    for i in 1..221 {
        if sample_points.contains(&state.cycle) {
            result += i as i32 * state.register_x
        }
        if Status::Ready == state.status {
            let instruction = &instructions[instruction_pntr];
            match instruction.flavor {
                Flavor::Addx => {
                    let new_value = state.register_x + instruction.value.unwrap();
                    state.schedule_set(i + 2, new_value);
                }
                Flavor::Noop => {}
            }
            instruction_pntr += 1;
        }

        state.tick();
    }
    result
}

fn part_2(instructions: &Vec<Instruction>) {
    let mut state = State::new();
    let mut instruction_pntr: usize = 0;
    let mut screen = [' '; 40 * 6];
    for i in 1..241 {
        let row_loc = (state.cycle as i32 % 40) - 1;
        if row_loc >= (state.register_x - 1) && row_loc <= (state.register_x + 1) {
            screen[state.cycle - 1] = '#';
        }
        if Status::Ready == state.status {
            let instruction = &instructions[instruction_pntr];
            match instruction.flavor {
                Flavor::Addx => {
                    let new_value = state.register_x + instruction.value.unwrap();
                    state.schedule_set(i + 2, new_value);
                }
                Flavor::Noop => {}
            }
            instruction_pntr += 1;
        }

        state.tick();
    }
    for row in screen.chunks(40) {
        println!("{}", String::from_iter(row));
    }
}

fn main() {
    let instructions = include_str!("../../../inputs/input10.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Instruction>>();
    println!("{}", part_1(&instructions));
    part_2(&instructions);
}

#[test]
fn test_1() {
    let input = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop"
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<Instruction>>();

    assert_eq!(part_1(&input), 13140);
    part_2(&input);
}
