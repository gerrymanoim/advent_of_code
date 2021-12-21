use utils::{Point, stdin_to_str_vec};
use std::collections::HashSet;

struct Fold {
    direction: char,
    coordinate: usize,
}

impl Fold {
    fn from_string(s: &String) -> Fold {
        //only ascii input so we should be able to do this
        let equals_idx = s.find('=').unwrap();
        // TODO double iter?
        let direction = s.chars().nth(equals_idx-1).unwrap();
        let coordinate = s[equals_idx+1..].parse().unwrap();
        Fold { direction, coordinate}
    }
}

fn perform_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut out = HashSet::new();

    for e in points.iter() {
        if fold.direction == 'x' {
            if e.x < fold.coordinate {
                out.insert(e.to_owned());
            } else {
                out.insert(Point { x: 2* fold.coordinate - e.x, y: e.y, value: e.value });
            }
        } else {
            if e.y < fold.coordinate {
                out.insert(e.to_owned());
            } else {
                out.insert(Point { x: e.x, y: 2* fold.coordinate - e.y, value: e.value });
            }
        }
    }

    out
}

fn part_1(points: &HashSet<Point>, fold: &Fold) -> usize {
    let single_fold = perform_fold(points, fold);
    single_fold.len()
}

fn part_2(points: &HashSet<Point>, folds: &Vec<Fold>) -> String {
    let mut single_fold = points.clone();
    for fold in folds {
        single_fold = perform_fold(&single_fold, fold);
    }
    // now we need to figure out how to display this
    //ew
    let coords = single_fold.iter().map(|p|p.xy()).collect::<HashSet<(usize, usize)>>();
    let width = single_fold.iter().map(|p|p.x).max().unwrap();
    let height = single_fold.iter().map(|p|p.y).max().unwrap();
    let mut display: String = "".to_string();
    for y in 0..height+1 {
        for x in 0..width+1 {
            if coords.contains(&(x, y)) {
                display += "X";
            } else {
                display += " ";
            }
        }
        display += "\n";
    }

    display
}

fn main() {
    let raw_input = stdin_to_str_vec();
    let split_idx = raw_input.iter().position(|s| s=="").unwrap();
    let points = raw_input[..split_idx].iter().map(|s| {
        let (x, y) = s.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            value: 0,
        }
    }).collect::<HashSet<Point>>();
    let folds = raw_input[split_idx+1..].iter().map(|s|{
        Fold::from_string(s)
    }).collect::<Vec<Fold>>();
    println!("{:?}", part_1(&points, folds.first().unwrap()));
    println!("{}", part_2(&points, &folds))

}

#[test]
fn examples() {
    let raw_input: Vec<String> = "6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5".lines().map(|l|l.trim().to_string()).collect();
    let split_idx = raw_input.iter().position(|s| s=="").unwrap();
    let points = raw_input[..split_idx].iter().map(|s| {
        let (x, y) = s.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            value: 0,
        }
    }).collect::<HashSet<Point>>();
    let folds = raw_input[split_idx+1..].iter().map(|s|{
        Fold::from_string(s)
    }).collect::<Vec<Fold>>();
    assert_eq!(part_1(&points, folds.first().unwrap()), 17)
}
