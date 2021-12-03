extern crate utils;
use utils::stdin_to_str_vec;

fn count_bits(v: &Vec<String>) -> Vec<i32> {
    let n_digits = v[0].len();
    let mut reg = vec![0; n_digits];
    for l in v {
        for (idx, c) in l.chars().enumerate() {
            match c {
                '1' => reg[idx] += 1,
                '0' => reg[idx] -= 1,
                _ => panic!("Unexpected char {}", c),
            }
        }
    }
    reg
}

fn part_1(v: &Vec<String>) -> i32 {
    let n_digits = v[0].len();
    let reg = count_bits(v);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (idx, n) in reg.iter().enumerate() {
        // yuck
        let exp = (n_digits - idx - 1) as u32;
        if *n > 0 {
            gamma += i32::pow(2, exp);
        } else {
            epsilon += i32::pow(2, exp);
        }
    }
    gamma * epsilon
}

fn count_bit_at_idx(v: &Vec<String>, idx: usize) -> i32 {
    let mut counter = 0;
    for l in v {
        match l.chars().nth(idx).unwrap() {
            '1' => counter += 1,
            '0' => counter -= 1,
            _ => unreachable!()
        }
    }
    counter
}

fn looping_filter(v: &Vec<String>, geq_zero: char, lt_zero: char) -> String {
    let mut candidates = v.clone();
    let mut looking_for;
    let mut counter = 0;
    while candidates.len() > 1 {
        let n = count_bit_at_idx(&candidates, counter);
        if n >= 0 {
            looking_for = geq_zero;
        } else {
            looking_for = lt_zero;
        }
        candidates = candidates
                .iter()
                .filter(|x| x.chars().nth(counter).unwrap() == looking_for)
                .map(|x| x.clone())
                .collect();
        counter+=1;
    }
    candidates[0].clone()
}

fn part_2(v: &Vec<String>) -> i32 {

    let ogr = looping_filter(v, '1', '0');
    let co2 = looping_filter(v, '0', '1');

    i32::from_str_radix(ogr.as_str(), 2).unwrap()
        * i32::from_str_radix(co2.as_str(), 2).unwrap()
}

fn main() {
    let diagnostic = stdin_to_str_vec();
    println!("{}", part_1(&diagnostic));
    println!("{}", part_2(&diagnostic));
}

#[test]
fn examples() {
    let raw_input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010"
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    assert_eq!(part_1(&raw_input), 198);
    assert_eq!(part_2(&raw_input), 230);
}
