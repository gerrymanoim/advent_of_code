use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let mut numbers: Vec<i32> = Vec::new();
    let lines_iter = reader.lock().lines().map(|l| l.unwrap());

    for line in lines_iter {
        numbers.push(line.parse::<i32>().unwrap())
    }
    // Part 1
    let get_to = 2020;
    let mut store = HashSet::new();
    for number in &numbers {
        if store.contains(number) {
            println!("{}", number * (get_to - number));
            break;
        } else {
            store.insert(get_to - number);
        }
    }
    // part 2
    numbers.sort();
    let max_idx = numbers.len() - 1;
    for (i, val) in numbers.iter().enumerate() {
        let mut l_pntr = i + 1;
        let mut r_pntr = max_idx;
        while l_pntr < r_pntr {
            let s = val + numbers[l_pntr] + numbers[r_pntr];
            if s == get_to {
                println!("{}", val * numbers[l_pntr] * numbers[r_pntr]);
                break;
            } else if s < get_to {
                l_pntr += 1;
            } else if s > get_to {
                r_pntr -= 1;
            }
        }
    }
}
