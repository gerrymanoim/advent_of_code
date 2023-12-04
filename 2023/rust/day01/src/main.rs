extern crate utils;

use utils::stdin_to_str_vec;

fn part_1(v: &Vec<String>) -> u32 {
    v.iter()
        .map(|s| {
            let mut first_found: Option<u32> = None;
            let mut last_found: Option<u32> = None;
            s.chars().filter_map(|c| c.to_digit(10)).for_each(|d| {
                if first_found.is_none() {
                    first_found = Some(d);
                }
                last_found = Some(d);
            });
            first_found.unwrap() * 10 + last_found.unwrap()
        })
        .sum()
}

fn part_2(v: &Vec<String>) -> u32 {
    v.iter()
        .map(|s| {
            let cs = s.chars().collect::<Vec<char>>();
            let mut idx = 0;
            let mut first_found: Option<u32> = None;
            let mut last_found: Option<u32> = None;
            while idx < cs.len() {
                let parse_result = try_parsing_chars(idx, &cs);
                idx = parse_result.0;
                if let Some(i) = parse_result.1 {
                    if first_found.is_none() {
                        first_found = Some(i);
                    }
                    last_found = Some(i);
                }
            }
            dbg!(first_found.unwrap() * 10 + last_found.unwrap());
            first_found.unwrap() * 10 + last_found.unwrap()
        })
        .sum()
}

fn try_parsing_chars(idx: usize, cs: &Vec<char>) -> (usize, Option<u32>) {
    if let Some(i) = cs[idx].to_digit(10) {
        return (idx + 1, Some(i));
    }

    let remaining = cs.len() - idx - 1;
    if remaining < 2 {
        return (idx + 1, None);
    }
    if remaining >= 4 {
        // dbg!(&cs[idx..idx + 5]);
        if cs[idx..idx + 5] == ['t', 'h', 'r', 'e', 'e'] {
            return (idx + 5, Some(3));
        }
        if cs[idx..idx + 5] == ['s', 'e', 'v', 'e', 'n'] {
            return (idx + 5, Some(7));
        }
        if cs[idx..idx + 5] == ['e', 'i', 'g', 'h', 't'] {
            return (idx + 5, Some(8));
        }
    }
    if remaining >= 3 {
        if cs[idx..idx + 4] == ['f', 'o', 'u', 'r'] {
            return (idx + 4, Some(4));
        }
        if cs[idx..idx + 4] == ['f', 'i', 'v', 'e'] {
            return (idx + 4, Some(5));
        }
        if cs[idx..idx + 4] == ['n', 'i', 'n', 'e'] {
            return (idx + 4, Some(9));
        }
    }
    if remaining >= 2 {
        if cs[idx..idx + 3] == ['o', 'n', 'e'] {
            return (idx + 3, Some(1));
        }
        if cs[idx..idx + 3] == ['t', 'w', 'o'] {
            return (idx + 3, Some(2));
        }
        if cs[idx..idx + 3] == ['s', 'i', 'x'] {
            return (idx + 3, Some(6));
        }
    }

    return (idx + 1, None);
}

fn main() {
    let lines = stdin_to_str_vec();
    println!("{}", part_1(&lines));
    println!("{}", part_2(&lines));
}

#[test]
fn test_2() {
    let input = "Rtwo1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"
        .lines()
        .map(|l| l.trim().to_string())
        .collect();

    assert_eq!(part_2(&input), 281);
}
