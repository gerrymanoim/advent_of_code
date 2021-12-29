use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use utils::{stdin_to_str_vec, Grid, Point};

fn manhattan_distance(p1: &Point, p2: &Point) -> usize {
    // TODO - do this with fewer/cleaner type conversions
    ((p1.x as i32 - p2.x as i32).abs() + (p1.y as i32 - p2.y as i32).abs()) as usize
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // make a min-heap by flipping
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(g: &Grid, start_point: &Point, end_point: &Point) -> usize {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: start_point.xy(),
    });

    let mut cost_so_far: HashMap<&Point, usize> = HashMap::new();

    cost_so_far.insert(start_point, 0);

    while let Some(State { cost: _, position }) = frontier.pop() {
        let current = g.get(position.0, position.1);
        if current == end_point {
            return cost_so_far[current];
        }

        for coord in g.manhattan_neighbors_of(current).iter().flatten() {
            let neighbor = g.get(coord.0, coord.1);
            let new_cost = cost_so_far[current] + neighbor.value;
            if !cost_so_far.contains_key(neighbor) || new_cost < cost_so_far[neighbor] {
                *cost_so_far.entry(neighbor).or_insert(0) = new_cost;
                frontier.push(State {
                    cost: new_cost + manhattan_distance(neighbor, end_point),
                    position: neighbor.xy(),
                });
            }
        }
    }
    panic!("This shouldn't happen!")
}

fn part_1(g: &Grid) -> usize {
    let start_point = g.get(0, 0);
    let end_point = g.get(g.width - 1, g.height - 1);

    astar(g, start_point, end_point)
}

fn stretch_grid(g: &Grid, multiplier: usize) -> Grid {
    let stretch_width = g.width * multiplier;
    let stretch_height = g.height * multiplier;
    let mut data = Vec::with_capacity(stretch_width * stretch_height);

    for y in 0..stretch_height {
        for x in 0..stretch_width {
            let reference_point = g.get(x % g.width, y % g.height);
            let xshifts = x / g.width;
            let yshifts = y / g.height;
            let new_value = (reference_point.value + xshifts + yshifts) % 9;
            data.push(Point {
                x,
                y,
                value: if new_value == 0 { 9 } else { new_value },
            });
        }
    }
    Grid {
        data,
        width: stretch_width,
        height: stretch_height,
    }
}

fn part_2(g: &Grid) -> usize {
    let big_grid = stretch_grid(g, 5);
    let start_point = big_grid.get(0, 0);
    let end_point = big_grid.get(big_grid.width - 1, big_grid.height - 1);

    astar(&big_grid, start_point, end_point)
}
fn main() {
    let raw_input = stdin_to_str_vec();
    let grid = Grid::from_rows_vec(&raw_input);
    println!("{:?}", part_1(&grid));
    println!("{:?}", part_2(&grid));
}

#[test]
fn examples() {
    let raw_input = "1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581"
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let grid = Grid::from_rows_vec(&raw_input);

    assert_eq!(part_1(&grid), 40);
    assert_eq!(part_2(&grid), 315);
}
