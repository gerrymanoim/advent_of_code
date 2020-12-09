use std::collections::VecDeque;
use std::io::{self, Read};

fn is_valid_entry(entry: i64, q: &VecDeque<i64>) -> bool {
    for i in q {
        for j in q {
            if i != j && i + j == entry {
                return true;
            }
        }
    }
    return false;
}

fn part_1(numbers: &Vec<i64>, lookback: usize) -> Option<i64> {
    let mut q: VecDeque<i64> = VecDeque::with_capacity(lookback);
    for number in numbers {
        if q.len() < lookback {
            q.push_back(*number);
        } else {
            if !is_valid_entry(*number, &q) {
                return Some(*number);
            }
            q.push_back(*number);
            q.pop_front();
        }
    }

    None
}

fn part_2(numbers: &Vec<i64>, needle: i64) -> Option<i64> {
    for (i, anchor) in numbers.iter().enumerate() {
        let mut j = i;
        let mut total = *anchor;
        while total < needle && j < numbers.len() {
            j += 1;
            total += numbers[j];
        }
        if total == needle && i < j {
            let smallest = numbers[i..j + 1].iter().min().unwrap();
            let largest = numbers[i..j + 1].iter().max().unwrap();
            return Some(smallest + largest);
        }
    }
    None
}

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let numbers: Vec<i64> = data.lines().map(|i| i.parse::<i64>().unwrap()).collect();

    let answer_1 = part_1(&numbers, 25).unwrap();
    println!("Part 1: {:?}", answer_1);

    let answer_2 = part_2(&numbers, answer_1).unwrap();
    println!("Part 2: {:?}", answer_2);
}
