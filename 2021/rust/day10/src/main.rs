use utils::stdin_to_str_vec;

fn char_to_score_p1(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_line_p1(s: &String) -> i64 {
    let mut chars_iter = s.chars();
    let mut stack = Vec::new();
    stack.push(chars_iter.next().unwrap());
    for c in chars_iter {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                let expected = match stack.last().unwrap() {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!(),
                };
                if expected != c {
                    return char_to_score_p1(c);
                } else {
                    stack.pop();
                }
            }
        }
    }

    0
}

fn part_1(v: &Vec<String>) -> i64 {
    v.iter().map(score_line_p1).sum()
}

fn score_for_stack(v: &mut Vec<char>) -> i64 {
    let mut score = 0;
    v.reverse();
    for c in v.iter() {
        score *= 5;
        score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        }
    }
    score
}

fn score_line_p2(s: &String) -> Option<i64> {
    let mut chars_iter = s.chars();
    let mut stack = Vec::new();
    stack.push(chars_iter.next().unwrap());
    for c in chars_iter {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => {
                let expected = match stack.last().unwrap() {
                    '(' => ')',
                    '[' => ']',
                    '{' => '}',
                    '<' => '>',
                    _ => unreachable!(),
                };
                if expected != c {
                    // filtered out
                    return None;
                } else {
                    stack.pop();
                }
            }
        }
    }
    Some(score_for_stack(&mut stack))
}

fn part_2(v: &Vec<String>) -> i64 {
    let mut scores: Vec<i64> = v.iter().filter_map(score_line_p2).collect();
    scores.sort();
    let mid = scores.len() / 2;
    scores[mid]
}

fn main() {
    let raw_input = stdin_to_str_vec();
    println! {"{}", part_1(&raw_input)};
    println! {"{}", part_2(&raw_input)};
}

#[test]
fn examples() {
    let raw_input = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";
    let rows = raw_input
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    assert_eq!(part_1(&rows), 26397);
    assert_eq!(part_2(&rows), 288957);
}
