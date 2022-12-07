use std::collections::HashSet;

fn part_1(v: &Vec<char>) -> usize {
    v.windows(4)
        .take_while(|cs| {
            let chars: HashSet<&char> = HashSet::from_iter(*cs);
            chars.len() != cs.len()
        })
        .count()
        + 4
}

fn part_2(v: &Vec<char>) -> usize {
    v.windows(14)
        .take_while(|cs| {
            let chars: HashSet<&char> = HashSet::from_iter(*cs);
            chars.len() != cs.len()
        })
        .count()
        + 14
}

fn main() {
    let input = include_str!("../../../inputs/input06.txt")
        .chars()
        .collect::<Vec<char>>();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

#[test]
fn example() {
    let inputs = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ];
    for (input_str, expected_1, expected_2) in inputs {
        let input = input_str.chars().collect::<Vec<char>>();
        assert_eq!(part_1(&input), expected_1);
        assert_eq!(part_2(&input), expected_2);
    }
}
