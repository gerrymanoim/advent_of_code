use std::io::{self, BufRead};

#[derive(Debug)]
struct Instruction {
    op: String,
    arg: i32,
}

fn part_1(tape: &Vec<Instruction>) -> i32 {
    let mut pntr: i32 = 0;
    let mut accumulator = 0;
    let n_ixs = tape.len();
    let mut seen_ixs = vec![false; n_ixs];

    loop {
        if seen_ixs[pntr as usize] {
            return accumulator;
        } else {
            seen_ixs[pntr as usize] = true;
        }
        let ix = &tape[pntr as usize];
        match ix.op.as_str() {
            "acc" => {
                accumulator += ix.arg;
                pntr += 1
            }
            "nop" => pntr += 1,
            "jmp" => pntr += ix.arg,
            _ => panic!("Unknown operations: {:?}", ix.op),
        }
    }
}

fn _run_machine(tape: &Vec<Instruction>, flip_ix: usize) -> Option<i32> {
    let mut pntr: i32 = 0;
    let mut accumulator = 0;
    let n_ixs = tape.len();
    let mut seen_ixs = vec![false; n_ixs];
    let tape_len = tape.len() as i32;

    loop {
        if pntr >= tape_len {
            return Some(accumulator);
        }
        if seen_ixs[pntr as usize] {
            return None;
        } else {
            seen_ixs[pntr as usize] = true;
        }
        let ix = &tape[pntr as usize];
        match ix.op.as_str() {
            "acc" => {
                accumulator += ix.arg;
                pntr += 1
            }
            "nop" => {
                if pntr == flip_ix as i32 {
                    pntr += ix.arg
                } else {
                    pntr += 1
                }
            }
            "jmp" => {
                if pntr == flip_ix as i32 {
                    pntr += 1
                } else {
                    pntr += ix.arg
                }
            }
            _ => panic!("Unknown operations: {:?}", ix.op),
        }
    }
}

fn part_2(tape: &Vec<Instruction>) -> i32 {
    let mut probe_locations = Vec::new();
    for (i, _) in tape.iter().enumerate() {
        probe_locations.push(i);
    }

    for probe in probe_locations {
        match _run_machine(tape, probe) {
            Some(x) => return x,
            None => continue,
        }
    }
    panic!("No solution found");
}

fn main() {
    //parse some things.
    let mut tape: Vec<Instruction> = Vec::new();
    for line in io::stdin().lock().lines() {
        let ix = line.unwrap();
        let mut ix_iter = ix.split_whitespace();
        let (op, arg_str) = (ix_iter.next().unwrap(), ix_iter.next().unwrap());
        let arg = arg_str.parse::<i32>().unwrap();
        // Note - I should be able to use a static &str but cannot figure out the borrow of ix
        tape.push(Instruction {
            op: op.to_string(),
            arg: arg,
        });
    }

    println!("Part 1: {}", part_1(&tape));

    println!("Part 2: {}", part_2(&tape));
}
