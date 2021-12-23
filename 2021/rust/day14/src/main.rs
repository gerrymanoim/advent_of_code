use std::{collections::HashMap};
use utils::{stdin_to_str_vec, vec_to_counts};

fn parse_vec_input(raw_input: Vec<String>) -> (String, HashMap<String, char>) {
    let split_idx = raw_input.iter().position(|s| s == "").unwrap();

    let polimer_template = raw_input[split_idx - 1].to_owned();
    let insertion_rules = raw_input[split_idx + 1..]
        .iter()
        .map(|s| {
            let (k, v) = s.split_once(" -> ").unwrap();
            (k.to_string(), v.to_string().chars().nth(0).unwrap())
        })
        .collect::<HashMap<String, char>>();
    (polimer_template, insertion_rules)
}

fn step(s: Vec<char>, rules: &HashMap<String, char>) -> Vec<char> {
    let mut polymer = s
        .windows(2)
        .flat_map(|pair| [pair[0], rules[&String::from_iter(pair)].try_into().unwrap()])
        .collect::<Vec<char>>();
    polymer.push(*s.last().unwrap());
    polymer
}

fn part_1(s: &String, rules: &HashMap<String, char>) -> i64 {
    let mut polymer = s.chars().collect();

    for _step in 0..10 {
        polymer = step(polymer, rules);
    }

    let counter = vec_to_counts(&polymer);
    counter.values().max().unwrap() - counter.values().min().unwrap()
}

fn step_2(
    pair_counter: HashMap<(char, char), i64>,
    letter_counter: &mut HashMap<char, i64>,
    rules: &HashMap<String, char>,
) -> HashMap<(char, char), i64> {
    let mut new_counts = HashMap::new();

    for ((l, r), v) in pair_counter {
        let c = rules[&String::from_iter([l, r])];
        *letter_counter.entry(c).or_insert(0) += v;
        *new_counts.entry((l, c)).or_insert(0) += v;
        *new_counts.entry((c, r)).or_insert(0) += v;
    }
    new_counts
}

fn part_2(s: &String, rules: &HashMap<String, char>) -> i64 {
    let letters = s.chars().collect::<Vec<char>>();
    let mut letter_counter = vec_to_counts(&letters);
    let pairs = letters
        .windows(2)
        .map(|cs| (cs[0], cs[1]))
        .collect::<Vec<(char, char)>>();
    let mut pair_counter = vec_to_counts(&pairs);
    for _step in 0..40 {
        pair_counter = step_2(pair_counter, &mut letter_counter, rules)
    }
    letter_counter.values().max().unwrap() - letter_counter.values().min().unwrap()
}

fn main() {
    let raw_input = stdin_to_str_vec();
    let (polimer_template, insertion_rules) = parse_vec_input(raw_input);
    println!("{:?}", part_1(&polimer_template, &insertion_rules));
    println!("{:?}", part_2(&polimer_template, &insertion_rules))
}

#[test]
fn examples() {
    let raw_input: Vec<String> = "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C "
        .lines()
        .map(|l| l.trim().to_string())
        .collect();
    let (polimer_template, insertion_rules) = parse_vec_input(raw_input);
    assert_eq!(part_1(&polimer_template, &insertion_rules), 1588);
    assert_eq!(part_2(&polimer_template, &insertion_rules), 2188189693529)
}
