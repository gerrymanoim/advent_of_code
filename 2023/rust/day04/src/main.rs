use utils::stdin_to_str_vec;

struct Card {
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl Card {
    fn matched_count(&self) -> usize {
        self.card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
    fn score_part_1(&self) -> usize {
        let matched_count = self.matched_count();
        if matched_count == 0 {
            0
        } else {
            2_usize.pow(matched_count as u32 - 1)
        }
    }
}

// Card   1: 34 55 49 53 46  7 82 22 59 33 | 33 29  7 66 22 51 59 21 55 85 53 26 94 46 24 82  6 47 38  2 34 89 49 41 76
fn parse_card_log(s: &str) -> (usize, Card) {
    let (card_id, card_log) = s.split_once(": ").unwrap();
    let mut card_id_iter = card_id.split_whitespace();
    card_id_iter.next();

    let card_number = card_id_iter.next().unwrap().parse::<usize>().unwrap();
    let (winning_numbers_log, card_numbers_log) = card_log.split_once(" | ").unwrap();
    // dbg!(winning_numbers_log, card_numbers_log);
    let winning_numbers = winning_numbers_log
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let card_numbers_log = card_numbers_log
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let card = Card {
        winning_numbers,
        card_numbers: card_numbers_log,
    };
    (card_number, card)
}

fn part_1(record: &Vec<Card>) -> usize {
    record.iter().map(|c| c.score_part_1()).sum()
}
fn part_2(record: &Vec<Card>) -> usize {
    let mut match_counts = vec![1; record.len()];
    let mut cards_seen = match_counts.len();
    for idx in 0..match_counts.len() {
        let copies = record[idx].matched_count();
        for offset in 1..copies + 1 {
            cards_seen += match_counts[idx];
            match_counts[idx + offset] += match_counts[idx];
        }
    }
    cards_seen
}
fn main() {
    let lines = stdin_to_str_vec();
    let record = lines.iter().map(|line| parse_card_log(line).1).collect();
    println!("{}", part_1(&record));
    println!("{}", part_2(&record));
}
