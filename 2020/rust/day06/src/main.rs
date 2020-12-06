use std::io::{self, Read};
use std::collections::HashSet;
use std::iter::FromIterator;

fn count_all_in_groups(group: Vec<&str>) -> usize {
    let mut base: HashSet<_> = group[0].chars().collect();
    for next_group in group[1..].iter().filter(|x| x != &&"") {
        let next_set: HashSet<_> = next_group.chars().collect();
        base = base.intersection(&next_set).cloned().collect();
    }
    base.len()
}


fn main() {
    // Parse some things
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();
    let groups = data.split("\n\n");

    let part_1 : usize = groups.clone().map(|group| {
        HashSet::<char>::from_iter(group.replace("\n", "").chars()).len()
    }).sum();

    println!("{:?}", part_1);

    let part_2 : usize = groups.map(|group| {
        group.split("\n").collect()
    }).map(|x| count_all_in_groups(x)).sum();

    println!("{:?}", part_2);

}
