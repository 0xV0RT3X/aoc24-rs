use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn process_card(card: &str, card_idx: usize, scratchcards: &mut HashMap<usize, u32>) -> u32 {
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

    for j in 0..winning_number_count {
        scratchcards.entry(card_idx + 1 + j).or_insert(1);
        let a = scratchcards.get(&(card_idx + 1 + j)).unwrap();
        scratchcards.insert(card_idx + 1 + j, a + scratchcards.get(&card_idx).unwrap());
    }

    if winning_number_count > 0 {
        1 << (winning_number_count - 1)
    } else {
        0
    }
}

fn main() {
    let cards = include_str!("../input.txt").lines();

    let mut total_points = 0;
    let mut scratchcards: HashMap<usize, u32> = HashMap::new();
    for (card_idx, card) in cards.enumerate() {
        scratchcards.entry(card_idx).or_insert(1);

        total_points += process_card(card, card_idx, &mut scratchcards);
    }

    println!("Part 1: {}", total_points);
    println!("Part 2: {}", scratchcards.values().sum::<u32>());
}
