use utils::line_of_stdin_to_int_vec;

fn part_1(v: &Vec<i32>) -> i32 {
    let mut tmp_v = v.clone();
    let mid = v.len() / 2;
    tmp_v.sort();
    let target = tmp_v[mid];
    // median
    tmp_v.iter().map(|n| (n - target).abs()).sum()
}

fn part_2(v: &Vec<i32>) -> i32 {
    let sum = v.iter().sum::<i32>() as f32;
    let count = v.len() as f32;
    let mean = sum / count;
    let distance = |target: i32| {
        v.iter()
            .map(|n| (n - target).abs())
            .map(|n| n * (n + 1) / 2)
            .sum()
    };
    let upper = distance(mean.ceil() as i32);
    let lower = distance(mean.floor() as i32);

    if upper < lower {
        upper
    } else {
        lower
    }
}
fn main() {
    let raw_input = line_of_stdin_to_int_vec();
    println!("{}", part_1(&raw_input));
    println!("{}", part_2(&raw_input));
}

#[test]
fn examples() {
    let raw_input = "16,1,2,0,4,2,7,1,2,14";
    let init_state = raw_input
        .split(',')
        .map(|s| s.parse::<i32>().expect("broken"))
        .collect::<Vec<i32>>();

    assert_eq!(part_1(&init_state), 37);
    assert_eq!(part_2(&init_state), 168);
}
