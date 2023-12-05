use utils::{stdin_to_str_vec, Grid, Point};
use colored::*;
use std::collections::HashMap;

const DIGITS: [char; 10] =  ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn is_near_symbol<'a>(g: &'a Grid, number_points: &Vec<&'a Point>) -> Option<&'a Point> {
    for number_point in number_points {
        let neighbors = g.neighbors_of(number_point);
        for neighbor in neighbors {
            if neighbor.is_some() {
                let (nx, ny) = neighbor.unwrap();
                let np = g.get(nx, ny);
                if (np.value != '.') & !DIGITS.contains(&np.value) {
                    return Some(np);
                }
            }
        }
    }
    None
}

fn part_1(g: &Grid) -> usize {
    let mut out = 0;
    for y in 0..g.height {
        print!("\n");
        let mut x = 0;
        while x < g.width {
            let base_point = g.get(x, y);
            if DIGITS.contains(&base_point.value) {
                let mut number_points = vec![base_point];
                while x < g.width {
                    x += 1;
                    let p = g.get(x, y);
                    if DIGITS.contains(&p.value) {
                        number_points.push(p);
                    } else {
                        break;
                    }
                }
                if is_near_symbol(&g, &number_points).is_some() {
                    print!("{}", number_points.iter().map(|p|p.value).collect::<String>().red());
                    number_points.reverse();
                    for (i, number_point) in number_points.iter().enumerate() {
                        out += (number_point.value.to_digit(10).unwrap() as usize)
                            * 10_usize.pow(i as u32);
                    }
                } else {
                    print!("{}", number_points.iter().map(|p|p.value).collect::<String>().blue());
                }
            } else {
                x += 1;
                if base_point.value == '.' {
                    print!("{}", base_point.value);
                } else {
                    print!("{}", base_point.value.to_string().yellow());
                }
            }
        }
    }
    print!("\n");
    out
}

fn part_2(g: &Grid) -> usize {
    let mut out = 0;
    let mut paired_symbols = HashMap::new();
    for y in 0..g.height {
        let mut x = 0;
        while x < g.width {
            let base_point = g.get(x, y);
            if DIGITS.contains(&base_point.value) {
                let mut number_points = vec![base_point];
                while x < g.width {
                    x += 1;
                    let p = g.get(x, y);
                    if DIGITS.contains(&p.value) {
                        number_points.push(p);
                    } else {
                        break;
                    }
                }
                if let Some(symbol) = is_near_symbol(&g, &number_points){
                    let mut number_value = 0;
                    number_points.reverse();
                    for (i, number_point) in number_points.iter().enumerate() {
                        number_value += (number_point.value.to_digit(10).unwrap() as usize)
                            * 10_usize.pow(i as u32);
                    }
                    if symbol.value == '*' {
                        if paired_symbols.contains_key(symbol) {
                            out += number_value * paired_symbols.get(symbol).unwrap();
                        } else {
                            paired_symbols.insert(symbol, number_value);
                        }

                    }                    
                }
            } else {
                x += 1;
            }
        }
    }
    out

}

fn main() {
    let lines = stdin_to_str_vec();
    let grid = Grid::from_rows_vec(&lines);
    println!("{}", part_1(&grid));
    println!("{}", part_2(&grid));
}

#[test]
fn test_1() {
    let input = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."
        .lines()
        .map(|l| l.trim().to_string())
        .collect();
    let grid = Grid::from_rows_vec(&input);
    assert_eq!(part_1(&grid), 4361);
}
