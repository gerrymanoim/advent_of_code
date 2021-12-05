use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Board {
    // TODO make these array?
    numbers: HashSet<i32>,
    winning_sets: Vec<HashSet<i32>>,
}

impl Board {
    fn from_str_array(s: &[String]) -> Self {
        let mut winning_sets = Vec::<HashSet<i32>>::new();

        let numbers: Vec<i32> = s
            .iter()
            .flat_map(|l| {
                l.split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        //rows
        for row in numbers.chunks(5) {
            winning_sets.push(row.iter().cloned().collect::<HashSet<i32>>());
        }
        //cols
        for i in 0..=4 {
            let mut winning_set = HashSet::new();
            for offset in 0..=4 {
                winning_set.insert(numbers[i + (offset * 5)]);
            }
            winning_sets.push(winning_set);
        }

        Board {
            numbers: numbers.iter().cloned().collect::<HashSet<i32>>(),
            winning_sets: winning_sets,
        }
    }

    fn has_won(&self, called_numbers: &HashSet<i32>) -> bool {
        for winning_set in &self.winning_sets {
            if winning_set.is_subset(called_numbers) {
                return true;
            }
        }
        false
    }

    fn unmakred_numbers(&self, called_numbers: &HashSet<i32>) -> i32 {
        self.numbers.difference(called_numbers).sum()
    }
}

fn part_1(number_draws: &[i32], boards: &Vec<Board>) -> i32 {
    let mut called_numbers = HashSet::<i32>::new();
    for draw in number_draws {
        called_numbers.insert(*draw);
        for board in boards {
            if board.has_won(&called_numbers) {
                return draw * board.unmakred_numbers(&called_numbers);
            }
        }
    }
    unreachable!()
}

fn part_2(number_draws: &[i32], boards: &Vec<Board>) -> i32 {
    let mut called_numbers = HashSet::<i32>::new();
    let mut remaining_boards = (0..boards.len()).collect::<HashSet<usize>>();
    for draw in number_draws {
        called_numbers.insert(*draw);
        for (idx, board) in boards.iter().enumerate() {
            if remaining_boards.contains(&idx) && board.has_won(&called_numbers) {
                remaining_boards.remove(&idx);
                if remaining_boards.len() == 0 {
                    return draw * board.unmakred_numbers(&called_numbers);
                }
            }
        }
    }
    unreachable!()

}

fn part_2_alternative(number_draws: &[i32], boards: &Vec<Board>) -> i32 {
    let mut called_numbers = HashSet::<i32>::new();
    let mut remaining_boards = boards.clone();
    let mut last_score = 0;
    for draw in number_draws {
        called_numbers.insert(*draw);
        for board in &remaining_boards {
            last_score = draw * board.unmakred_numbers(&called_numbers);
        }

        remaining_boards.retain(|board| !board.has_won(&called_numbers))
    }
    last_score

}

fn main() {
    let raw_input: Vec<String> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let number_draws = raw_input[0]
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::<Board>::new();
    for board in raw_input[1..].chunks(6) {
        // Gah
        let v = board.iter().map(|l| l.to_string()).collect::<Vec<_>>();
        boards.push(Board::from_str_array(&v));
    }
    println!("{:?}", part_1(&number_draws, &boards));
    println!("{:?}", part_2(&number_draws, &boards));


}

#[test]
fn examples() {
    let raw_input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7"
        .lines()
        .collect::<Vec<_>>();
    let number_draws = raw_input[0]
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = Vec::<Board>::new();
    for board in raw_input[1..].chunks(6) {
        // Gah
        let v = board.iter().map(|l| l.to_string()).collect::<Vec<_>>();
        boards.push(Board::from_str_array(&v));
    }

    assert_eq!(part_1(&number_draws, &boards), 4512);

    assert_eq!(part_2(&number_draws, &boards), 1924);
}
