extern crate utils;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn translate(&self, x_delta: i32, y_delta: i32) -> Self {
        Point {
            x: self.x + x_delta,
            y: self.y + y_delta,
        }
    }
}

fn maybe_move_tail(tail_loc: &Point, head_loc: &Point) -> Point {
    if head_loc.x == tail_loc.x - 2 {
        if head_loc.y == tail_loc.y {
            tail_loc.translate(-1, 0)
        } else if head_loc.y == tail_loc.y - 1 {
            tail_loc.translate(-1, -1)
        } else if head_loc.y == tail_loc.y + 1 {
            tail_loc.translate(-1, 1)
        } else if head_loc.y == tail_loc.y - 2 {
            // weird special case for part 2?
            tail_loc.translate(-1, -1)
        } else if head_loc.y == tail_loc.y + 2 {
            // weird special case for part 2?
            tail_loc.translate(-1, 1)
        } else {
            tail_loc.translate(0, 0)
        }
    } else if head_loc.x == tail_loc.x + 2 {
        if head_loc.y == tail_loc.y {
            tail_loc.translate(1, 0)
        } else if head_loc.y == tail_loc.y - 1 {
            tail_loc.translate(1, -1)
        } else if head_loc.y == tail_loc.y + 1 {
            tail_loc.translate(1, 1)
        } else if head_loc.y == tail_loc.y - 2 {
            // weird special case for part 2?
            tail_loc.translate(1, -1)
        } else if head_loc.y == tail_loc.y + 2 {
            // weird special case for part 2?
            tail_loc.translate(1, 1)
        } else {
            tail_loc.translate(0, 0)
        }
    } else if head_loc.y == tail_loc.y + 2 {
        if head_loc.x == tail_loc.x {
            tail_loc.translate(0, 1)
        } else if head_loc.x == tail_loc.x - 1 {
            tail_loc.translate(-1, 1)
        } else if head_loc.x == tail_loc.x + 1 {
            tail_loc.translate(1, 1)
        } else {
            tail_loc.translate(0, 0)
        }
    } else if head_loc.y == tail_loc.y - 2 {
        if head_loc.x == tail_loc.x {
            tail_loc.translate(0, -1)
        } else if head_loc.x == tail_loc.x - 1 {
            tail_loc.translate(-1, -1)
        } else if head_loc.x == tail_loc.x + 1 {
            tail_loc.translate(1, -1)
        } else {
            tail_loc.translate(0, 0)
        }
    } else {
        tail_loc.translate(0, 0)
    }
}

fn move_head_single_step(head_loc: &Point, direction: &str) -> Point {
    match direction {
        "L" => head_loc.translate(-1, 0),
        "R" => head_loc.translate(1, 0),
        "U" => head_loc.translate(0, 1),
        "D" => head_loc.translate(0, -1),
        _ => unreachable!("This shouldn't happen"),
    }
}

fn make_move(
    mut head_loc: Point,
    mut tail_loc: Point,
    instruction: &str,
    tracker: &mut HashSet<Point>,
) -> (Point, Point) {
    let (direction, magnitude_str) = instruction.split_once(' ').unwrap();
    let magnitude = magnitude_str.parse::<i32>().unwrap();
    (0..magnitude).for_each(|_step| {
        head_loc = move_head_single_step(&head_loc, direction);
        tail_loc = maybe_move_tail(&tail_loc, &head_loc);
        tracker.insert(tail_loc);
    });
    (head_loc, tail_loc)
}

fn part_1(instructions: &Vec<&str>) -> usize {
    let mut tracker: HashSet<Point> = HashSet::new();
    let mut head_loc = Point { x: 0, y: 0 };
    let mut tail_loc = Point { x: 0, y: 0 };
    tracker.insert(tail_loc);
    for instruction in instructions.iter() {
        (head_loc, tail_loc) = make_move(head_loc, tail_loc, instruction, &mut tracker);
    }
    tracker.len()
}

fn part_2(instructions: &Vec<&str>) -> usize {
    let mut tracker: HashSet<Point> = HashSet::new();
    let mut head_loc = Point { x: 0, y: 0 };
    let mut tail_locs = [Point { x: 0, y: 0 }; 9];
    tracker.insert(tail_locs.last().unwrap().clone());
    for instruction in instructions.iter() {
        let (direction, magnitude_str) = instruction.split_once(' ').unwrap();
        let magnitude = magnitude_str.parse::<i32>().unwrap();
        (0..magnitude).for_each(|_step| {
            head_loc = move_head_single_step(&head_loc, direction);
            let mut head_pntr = head_loc;
            tail_locs.iter_mut().for_each(|tail_loc| {
                *tail_loc = maybe_move_tail(tail_loc, &head_pntr);
                head_pntr = *tail_loc;
            });
            tracker.insert(tail_locs.last().unwrap().clone());
        });
    }
    tracker.len()
}

fn main() {
    let instructions = include_str!("../../../inputs/input09.txt")
        .lines()
        .collect::<Vec<&str>>();
    println!("{}", part_1(&instructions));
    println!("{}", part_2(&instructions));
}

#[test]
fn test_1() {
    let input = "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2"
    .lines()
    .map(|l| l.trim())
    .collect();

    assert_eq!(part_1(&input), 13);
    assert_eq!(part_2(&input), 1);
}

#[test]
fn test_2() {
    let input = "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20"
        .lines()
        .map(|l| l.trim())
        .collect();

    assert_eq!(part_2(&input), 36);
}
