use std::io::{self, BufRead};

fn rule_parser(rule: &str) -> (usize, usize, char) {
    let mut rule_split = rule.split(" ");

    let mut char_range_split = rule_split.next().unwrap().split("-");

    let left_number = char_range_split.next().unwrap().parse::<usize>().unwrap();
    let right_number = char_range_split.next().unwrap().parse::<usize>().unwrap();
    let rule_char = rule_split.next().unwrap().chars().next().unwrap();


    (left_number, right_number, rule_char)
}

fn part_1(v: &Vec<String>) -> i32 {
    let mut valid_counter = 0;
    for item in v {
        let mut split = item.split(": ");
        let (min_char, max_char, rule_char) = rule_parser(split.next().unwrap());

        let char_count = split.next().unwrap().matches(rule_char).count();

        if (min_char <= char_count) & ( char_count <= max_char) {
            valid_counter += 1;
        }
    }

    valid_counter
}


fn part_2(v: &Vec<String>) -> i32 {
    let mut valid_counter = 0;
    for item in v {
        let mut split = item.split(": ");
        let (left_char, right_char, rule_char) = rule_parser(split.next().unwrap());

        let chars: Vec<char> = split.next().unwrap().chars().collect();

        if (chars[left_char - 1] == rule_char) ^ ( chars[right_char - 1] == rule_char) {
            valid_counter += 1;
        }
    }

    valid_counter
}

fn main() {
    // Parse some things
    let mut data = Vec::new();
    for line in io::stdin().lock().lines() {
        data.push(line.unwrap());
    }

    // Part 1
    println!("{}", part_1(&data));

    // Part 2
    println!("{}", part_2(&data));
}
