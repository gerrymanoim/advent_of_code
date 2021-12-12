use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Hash, Eq)]
struct Point {
    x: usize,
    y: usize,
    height: usize,
}

impl Point {
    fn risk_level(&self) -> usize {
        &self.height + 1
    }
}
#[derive(Debug)]
pub struct Grid {
    data: Vec<Point>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_rows_vec(grid_rows: &Vec<String>) -> Grid {
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
                    height: data_points[y * width + x],
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
    fn get(&self, x: usize, y: usize) -> &Point {
        &self.data[y * self.width + x]
    }

    fn neighbors_of(&self, p: &Point) -> [Option<&Point>; 4] {
        let mut neighbors: [Option<&Point>; 4] = [None, None, None, None];

        if p.y > 0 {
            neighbors[0] = Some(self.get(p.x, p.y - 1));
        }
        if p.x < self.width - 1 {
            neighbors[1] = Some(self.get(p.x + 1, p.y));
        }
        if p.y < self.height - 1 {
            neighbors[2] = Some(self.get(p.x, p.y + 1));
        }
        if p.x > 0 {
            neighbors[3] = Some(self.get(p.x - 1, p.y));
        }

        neighbors
    }
}

fn find_low_points(g: &Grid) -> Vec<&Point> {
    let mut out = Vec::<&Point>::new();

    for p in g.data.iter() {
        let neighbors = g.neighbors_of(p);
        if neighbors
            .iter()
            .filter_map(|n| n.as_ref())
            .all(|n| n.height > p.height)
        {
            out.push(p);
        }
    }

    out
}

fn part_1(g: &Grid) -> usize {
    // find risk level of all low points

    find_low_points(g).iter().map(|p| p.risk_level()).sum()
}

fn find_basin_size(start_point: &Point, g: &Grid) -> usize {
    let mut basin_size = 1;
    let mut seen_points = HashSet::<&Point>::new();
    let mut edge_points = VecDeque::new();
    edge_points.push_back(start_point);
    // println!("Start point: {:?}", start_point);

    seen_points.insert(start_point);

    while !edge_points.is_empty() {
        let neighbors = g.neighbors_of(edge_points.pop_front().unwrap());
        for p in neighbors.into_iter().filter_map(|p| p) {
            if !seen_points.contains(p) {
                seen_points.insert(p);
                if p.height < 9 {
                    basin_size += 1;
                    edge_points.push_back(p)
                }
            }
        }
    }
    basin_size
}

fn part_2(g: &Grid) -> usize {
    let mut basin_sizes = find_low_points(g)
        .iter()
        .map(|p| find_basin_size(p, g))
        .collect::<Vec<usize>>();
    basin_sizes.sort();
    basin_sizes[basin_sizes.len() - 3..].iter().product()
}

fn main() {
    let grid_rows = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let grid = Grid::from_rows_vec(&grid_rows);
    println!("{}", part_1(&grid));
    println!("{}", part_2(&grid));
}

#[test]
fn examples() {
    let raw_input = "2199943210
    3987894921
    9856789892
    8767896789
    9899965678";

    let grid_rows = raw_input
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    let grid = Grid::from_rows_vec(&grid_rows);
    assert_eq!(part_1(&grid), 15);
    assert_eq!(part_2(&grid), 1134);
}
