use std::io::{self, BufRead};
use std::{collections::HashMap, hash::Hash};

#[inline(always)]
pub fn stdin_to_int_vec() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

#[inline(always)]
pub fn stdin_to_str_vec() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
}

pub fn line_of_stdin_to_int_vec() -> Vec<i32> {
    io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .flat_map(|s| {
            s.split(',')
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub value: usize,
}

impl Point {
    pub fn xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub data: Vec<Point>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from_rows_vec(grid_rows: &Vec<String>) -> Grid {
        let width = grid_rows[0].len();
        let height = grid_rows.len();
        let data_points = grid_rows
            .iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect::<Vec<usize>>();
        let mut data: Vec<Point> = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let p = Point {
                    x,
                    y,
                    value: data_points[y * width + x],
                };
                data.push(p);
            }
        }
        Grid {
            data,
            width,
            height,
        }
    }
    pub fn get(&self, x: usize, y: usize) -> &Point {
        &self.data[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: usize) {
        self.data[y * self.width + x].value = value;
    }

    /// Neighbors clockwise
    ///
    /// 0 1 2
    /// 7 p 3
    /// 6 5 4
    pub fn neighbors_of(&self, p: &Point) -> [Option<(usize, usize)>; 8] {

        let mut neighbors: [Option<(usize, usize)>; 8] = [None, None, None, None, None, None, None, None];
        if p.y > 0 && p.x > 0 {
            neighbors[0] = Some((p.x - 1, p.y -1));
        }
        if p.y > 0 {
            neighbors[1] = Some((p.x, p.y - 1));
        }
        if p.y > 0 && p.x < self.width - 1 {
            neighbors[2] = Some((p.x + 1, p.y -1));
        }
        if p.x < self.width - 1 {
            neighbors[3] = Some((p.x + 1, p.y));
        }
        if p.y < self.height - 1 && p.x < self.width - 1 {
            neighbors[4] = Some((p.x + 1, p.y + 1));
        }
        if p.y < self.height - 1 {
            neighbors[5] = Some((p.x, p.y + 1));
        }
        if p.y < self.height - 1 && p.x > 0 {
            neighbors[6] = Some((p.x - 1, p.y +1));
        }
        if p.x > 0 {
            neighbors[7] = Some((p.x - 1, p.y));
        }

        neighbors
    }
}

pub fn vec_to_counts<T: Eq + Hash + Clone>(v: &Vec<T>) -> HashMap<T, i64> {
    let mut letters: HashMap<T, i64> = HashMap::new();

    for ch in v {
        let counter = letters.entry(ch.to_owned()).or_insert(0);
        *counter += 1;
    }
    letters
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
