extern crate utils;

use utils::stdin_to_str_vec;

fn choice_to_score(c: char) -> usize {
    match c {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("something went wrong"),
    }
}
// A X - rock
// B Y - paper
// C Z - scissors
fn match_to_score(l: char, r: char) -> usize {
    match (l, r) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!("Unexpected input"),
    }
}

// X lose
// Y draw
// Z win
fn match_to_choice(l: char, r: char) -> char {
    match (l, r) {
        ('A', 'X') => 'Z',
        ('A', 'Y') => 'X',
        ('A', 'Z') => 'Y',
        ('B', 'X') => 'X',
        ('B', 'Y') => 'Y',
        ('B', 'Z') => 'Z',
        ('C', 'X') => 'Y',
        ('C', 'Y') => 'Z',
        ('C', 'Z') => 'X',
        _ => panic!("Unexpected input"),
    }
}

fn part_1(v: &Vec<String>) -> usize {
    v.iter()
        .map(|s| {
            let chars = s.as_bytes();
            match_to_score(chars[0] as char, chars[2] as char) + choice_to_score(chars[2] as char)
        })
        .sum()
}

fn part_2(v: &Vec<String>) -> usize {
    v.iter()
        .map(|s| {
            let chars = s.as_bytes();
            let you_play = match_to_choice(chars[0] as char, chars[2] as char);
            match_to_score(chars[0] as char, you_play) + choice_to_score(you_play)
        })
        .sum()
}

fn main() {
    let moves = stdin_to_str_vec();
    println!("{}", part_1(&moves));
    println!("{}", part_2(&moves));
}
