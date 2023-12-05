use utils::stdin_to_str_vec;

use std::collections::HashMap;

struct Cubes {
    r: usize,
    b: usize,
    g: usize,
}
type Games = Vec<Cubes>;

type GameRecord = HashMap<usize, Games>;

// Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red
// tp game struct
fn parse_game_log(s: &str) -> (usize, Games) {
    let (game_name, game_log) = s.split_once(": ").unwrap();
    let (_, game_id) = game_name.split_once(" ").unwrap();
    let games = game_log
        .split("; ")
        .map(|game| {
            let mut cubes = Cubes { r: 0, b: 0, g: 0 };
            game.split(", ").for_each(|cube| {
                let (count, color) = cube.split_once(" ").unwrap();
                let count = count.parse::<usize>().unwrap();
                match color {
                    "red" => cubes.r = count,
                    "blue" => cubes.b = count,
                    "green" => cubes.g = count,
                    _ => panic!("Unknown color: {}", color),
                }
            });
            cubes
        })
        .collect();
    (game_id.parse::<usize>().unwrap(), games)
}

fn part_1(game_record: &GameRecord, max_r: usize, max_b: usize, max_g: usize) -> usize {
    let mut id_sums = 0;
    for (game_id, games) in game_record {
        let mut valid_game = true;
        for game in games {
            if game.r > max_r || game.b > max_b || game.g > max_g {
                valid_game = false;
                break;
            }
        }
        if valid_game {
            id_sums += game_id;
        }
    }
    id_sums
}

fn part_2(game_record: &GameRecord) -> usize {
    let mut power_sum = 0;
    for (_game_id, games) in game_record {
        let mut min_r = 0;
        let mut min_b = 0;
        let mut min_g = 0;
        for game in games {
            min_r = min_r.max(game.r);
            min_b = min_b.max(game.b);
            min_g = min_g.max(game.g);
        }
        power_sum += min_r * min_b * min_g;
    }
    power_sum
}

fn main() {
    let lines = stdin_to_str_vec();
    let mut game_record: GameRecord = HashMap::new();
    lines.iter().for_each(|line| {
        let (game_id, games) = parse_game_log(line);
        game_record.insert(game_id, games);
    });

    println!("{}", part_1(&game_record, 12, 14, 13));
    println!("{}", part_2(&game_record));
}
