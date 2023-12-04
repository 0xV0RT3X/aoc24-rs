use std::collections::HashSet;
use std::time::Instant;

fn process_card(card: &str) -> u32 {
    let mut parts = card.split('|');

    let left = parts.next().unwrap();
    let right = parts.next().unwrap();

    let mut parts = left.split(':');

    let _card_id = parts.next().unwrap();
    let winning_numbers_part = parts.next().unwrap();

    let mut winning_numbers = HashSet::new();

    for winning_number in winning_numbers_part.split_whitespace() {
        winning_numbers.insert(winning_number.parse::<u32>().unwrap());
    }

    let mut winning_number_count = 0;
    for my_num in right.split_whitespace() {
        if winning_numbers.contains(&my_num.parse::<u32>().unwrap()) {
            winning_number_count += 1;
        }
    }

    if winning_number_count > 0 {
        1 << (winning_number_count - 1)
    } else {
        0
    }
}

fn main() {
    let start = Instant::now();
    let cards = include_str!("../input.txt").lines();

    let mut total_points = 0;
    for card in cards {
        total_points += process_card(card);
    }

    println!("{}", total_points);

    println!("{:?}", Instant::now() - start);
}
