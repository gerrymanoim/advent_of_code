extern crate utils;
use std::collections::{HashMap, HashSet};
use utils::stdin_to_str_vec;

fn make_char_to_priority_map() -> HashMap<char, usize> {
    let mut char_to_priority_map = HashMap::new();
    for (idx, c) in ('a'..='z').into_iter().enumerate() {
        char_to_priority_map.insert(c, idx + 1);
    }
    for (idx, c) in ('A'..='Z').into_iter().enumerate() {
        char_to_priority_map.insert(c, idx + 1 + 26);
    }
    return char_to_priority_map;
}

fn part_1(packs: &Vec<String>, char_to_priority_map: &HashMap<char, usize>) -> usize {

    packs
        .iter()
        .map(|s| {
            let n_items = s.len();
            let pivot = n_items / 2;
            let mut left: HashSet<char> = HashSet::new();
            let mut right = HashSet::new();
            for (idx, c) in s.char_indices() {
                if idx < pivot {
                    left.insert(c);
                } else {
                    right.insert(c);
                }
            }
            let common_char = left.intersection(&right).next().unwrap();
            char_to_priority_map.get(common_char).unwrap()
        })
        .sum()
}

fn part_2(packs: &Vec<String>, char_to_priority_map: &HashMap<char, usize>) -> usize {
    packs.chunks(3).map(|chunk| {
        let mut h0: HashSet<char> = HashSet::from_iter(chunk[0].chars());
        let h1: HashSet<char> = HashSet::from_iter(chunk[1].chars());
        let h2: HashSet<char> = HashSet::from_iter(chunk[2].chars());

        h0 = h0.intersection(&h1).cloned().collect();
        h0 = h0.intersection(&h2).cloned().collect();
        char_to_priority_map.get(&h0.iter().next().unwrap()).unwrap()
    }).sum()
}

fn main() {
    let char_to_priority_map = make_char_to_priority_map();
    let packs = stdin_to_str_vec();
    println!("{}", part_1(&packs, &char_to_priority_map));
    println!("{}", part_2(&packs, &char_to_priority_map));
}
