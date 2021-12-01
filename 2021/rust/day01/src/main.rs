use std::io::{self, BufRead};

fn main() {
    let numbers: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    /* Part 1
    To do this, count the number of times a depth measurement increases from
    the previous measurement. (There is no measurement before the first
    measurement.) In the example above, the changes are as follows:
    */
    let count = numbers.windows(2).filter(|x| x[1] > x[0]).count();
    println!("{}", count);

    // Part 2
    // consider sums of a three-measurement sliding window
    let count_2 = numbers
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count();
    println!("{}", count_2);
}
