use std::convert::TryInto;
use std::io::{self, Read};

fn part_1(numbers: &Vec<i64>) -> i64 {
    let diffs: Vec<_> = numbers
        .iter()
        .zip(numbers[1..].iter())
        .map(|x| x.1 - x.0)
        .collect();
    let n_1 = diffs.iter().filter(|&x| *x == 1).count();
    let n_3 = diffs.iter().filter(|&x| *x == 3).count();

    ((n_1 * n_3) as i64).try_into().unwrap()
}

fn part_2(numbers: &Vec<i64>) -> i64 {
    let target_number = *numbers.last().unwrap() as usize;
    let mut memo = vec![0;(target_number+1) as usize];

    // bootstrap the memo
    memo[0] = 1;

    if numbers.contains(&1) {
        memo[1] = 1;
    }
    if numbers.contains(&2) {
        if numbers.contains(&1) {
            memo[2] = 2;
        } else {
            memo[2] = 1;
        }
    }

    for n in 3..target_number+1 {
        let check_num = n as i64;
        if !numbers.contains(&check_num) {
            continue
        }

        memo[n] = memo[n-3] + memo[n-2] + memo[n-1];
    }

    return memo[target_number]

}

fn main() {
    let mut data = String::new();
    io::stdin().read_to_string(&mut data).unwrap();

    let mut numbers: Vec<_> = data.lines().map(|i| i.parse::<i64>().unwrap()).collect();
    numbers.sort();
    numbers.insert(0, 0);
    numbers.insert(numbers.len(), numbers.last().unwrap() + 3);

    let answer_1 = part_1(&numbers);
    println!("Part 1: {:?}", answer_1);

    let answer_2 = part_2(&numbers);
    println!("Part 2: {:?}", answer_2);
}
