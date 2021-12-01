use std::io::{self, BufRead};

// To do this, count the number of times a depth measurement increases from
// the previous measurement. (There is no measurement before the first
// measurement.) In the example above, the changes are as follows
fn part_1(v: &Vec<i32>) -> usize {
    v.windows(2).filter(|x| x[1] > x[0]).count()
}

// consider sums of a three-measurement sliding window
fn part_2(v: &Vec<i32>) -> usize {
    v.windows(3)
    .map(|x| x.iter().sum())
    .collect::<Vec<i32>>()
    .windows(2)
    .filter(|x| x[1] > x[0])
    .count()
}

fn main() {
    let numbers: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    println!("{}", part_1(&numbers));
    println!("{}", part_2(&numbers));
}

#[test]
fn examples() {
    let raw_input : Vec<i32> = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263".split_whitespace().map(|l|l.parse::<i32>().unwrap()).collect();
    assert_eq!(part_1(&raw_input), 7);
    assert_eq!(part_2(&raw_input), 5);
}
