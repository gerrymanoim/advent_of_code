use std::collections::HashMap;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<&str>>();
        let x = parts[0].parse::<i32>()?;
        let y = parts[1].parse::<i32>()?;

        Ok(Point {
            x: x as i32,
            y: y as i32,
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_input_string(s: String) -> Line {
        let parts = s.split(" -> ").collect::<Vec<&str>>();
        let start = Point::from_str(parts[0]).unwrap();
        let end = Point::from_str(parts[1]).unwrap();
        Line { start, end }
    }
    fn is_diagonal(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
    }
    fn points_covered(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        let step_x = if self.start.x == self.end.x {
            0
        } else if self.start.x < self.end.x {
            1
        } else {
            -1
        };
        let step_y = if self.start.y == self.end.y {
            0
        } else if self.start.y < self.end.y {
            1
        } else {
            -1
        };

        let mut cur_x = self.start.x;
        let mut cur_y = self.start.y;
        points.push(self.start);
        while cur_x != self.end.x || cur_y != self.end.y {
            cur_x += step_x;
            cur_y += step_y;
            points.push(Point { x: cur_x, y: cur_y });
        }
        points
    }
}

fn part_1(vent_lines: &Vec<Line>) -> usize {
    let mut seen_points = HashMap::new();
    for line in vent_lines.iter().filter(|p| !p.is_diagonal()) {
        for p in line.points_covered() {
            let counter = seen_points.entry(p).or_insert(0);
            *counter += 1;
        }
    }
    seen_points.values().filter(|c| **c > 1).count()
}

fn part_2(vent_lines: &Vec<Line>) -> usize {
    let mut seen_points = HashMap::new();
    for line in vent_lines.iter() {
        for p in line.points_covered() {
            let counter = seen_points.entry(p).or_insert(0);
            *counter += 1;
        }
    }
    seen_points.values().filter(|c| **c > 1).count()
}

fn main() {
    let vent_lines: Vec<Line> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(Line::from_input_string)
        .collect();

    println!("{}", part_1(&vent_lines));
    println!("{}", part_2(&vent_lines));
}

#[test]
fn examples() {
    let raw_input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let vent_lines: Vec<Line> = raw_input
        .lines()
        .map(|l| Line::from_input_string(l.trim().to_string()))
        .collect();

    assert_eq!(part_1(&vent_lines), 5);
    assert_eq!(part_2(&vent_lines), 12);
}
