use std::collections::HashSet;
use utils::{stdin_to_str_vec, Grid};

fn step(g: &mut Grid) -> usize {
    let mut flashed = HashSet::<(usize, usize)>::new();
    let mut to_flash = Vec::<(usize, usize)>::new();

    // Add one to every element
    for p in g.data.iter_mut() {
        p.value += 1;
        if p.value > 9 {
            to_flash.push(p.xy());
        }
    }

    // Flash
    while to_flash.len() > 0 {
        let coord = to_flash.pop().unwrap();
        let p = g.get(coord.0, coord.1);
        // flash
        if !flashed.contains(&coord) {
            flashed.insert(coord);
            // get neighbors and add value
            let neighbors = g.neighbors_of(p);
            for neighbor in neighbors.iter() {
                match neighbor {
                    Some(n) => {
                        let np = *g.get(n.0, n.1);
                        g.set(n.0, n.1, np.value + 1);
                        if np.value + 1 > 9 && !flashed.contains(&n) {
                            to_flash.push(*n)
                        }
                    }
                    None => continue,
                }
            }
        }
    }
    // Set to zero
    for p in &flashed {
        g.set(p.0, p.1, 0);
    }

    // return flash count
    flashed.len()
}

fn part_1(in_grid: &Grid) -> usize {
    let mut g = in_grid.to_owned();
    let mut flash_count = 0;
    for _step in 0..100 {
        flash_count += step(&mut g);
    }

    flash_count
}

fn part_2(in_grid: &Grid) -> usize {
    let mut g = in_grid.to_owned();
    let desired_flash_count = g.data.len();
    let mut step_count = 0;
    loop {
        step_count += 1;
        if step(&mut g) == desired_flash_count {
            return step_count
        }
    }

}

fn main() {
    let rows = stdin_to_str_vec();
    let g = Grid::from_rows_vec(&rows);
    println!("{:?}", part_1(&g));
    println!("{:?}", part_2(&g));
}

#[test]
fn examples() {
    let raw_input = "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526";
    let rows = raw_input
        .lines()
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();
    let g = Grid::from_rows_vec(&rows);
    assert_eq!(part_1(&g), 1656);
    assert_eq!(part_2(&g), 195);
}
