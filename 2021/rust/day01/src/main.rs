use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let mut numbers: Vec<i32> = Vec::new();
    let lines_iter = reader.lock().lines().map(|l| l.unwrap());

    for line in lines_iter {
        numbers.push(line.parse::<i32>().unwrap())
    }

    //Part 1
    let count = numbers.windows(2).filter(|x| x[1] > x[0]).count();
    println!("{}", count);

    //Part 2
    let triplets = numbers.windows(3);
    let window_sums: Vec<i32> = triplets.map(|x| x.iter().sum()).collect();
    let count_2 = window_sums.windows(2).filter(|x| x[1] > x[0]).count();
    println!("{}", count_2);
}
