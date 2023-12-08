use itertools::Itertools;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfKind = 7,
    FourOfKind = 6,
    FullHouse = 5,
    ThreeOfKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn parse_input(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap();
            let bid = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn hand_type(hand: &str) -> HandType {
    let card_count = hand.chars().counts();

    let mut sorted_counts: Vec<_> = card_count.into_values().collect();
    sorted_counts.sort();

    // 5         = 5 | Rank: 1 | Best
    // 1 4       = 5 | Rank: 2 |
    // 2 3       = 5 | Rank: 3 |
    // 1 1 3     = 5 | Rank: 4 |
    // 1 2 2     = 5 | Rank: 5 |
    // 1 1 1 2   = 5 | Rank: 6 |
    // 1 1 1 1 1 = 5 | Rank: 7 | Worst

    use HandType::*;
    match sorted_counts.as_slice() {
        [5] => FiveOfKind,
        [1, 4] => FourOfKind,
        [2, 3] => FullHouse,
        [1, 1, 3] => ThreeOfKind,
        [1, 2, 2] => TwoPair,
        [1, 1, 1, 2] => OnePair,
        [1, 1, 1, 1, 1] => HighCard,
        _ => panic!("Invalid hand: {}", hand),
    }
}

fn card_values(hand: &str) -> (u32, u32, u32, u32, u32) {
    hand.chars()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap()
}

fn hand_strength(hand: &str) -> (HandType, (u32, u32, u32, u32, u32)) {
    (hand_type(hand), card_values(hand))
}

fn main() {
    let mut hands_and_bids = parse_input(include_str!("../input.txt"));

    hands_and_bids.sort_by_key(|&(hand, _)| hand_strength(hand));

    let total_winnings: u32 = hands_and_bids
        .iter()
        .enumerate()
        .map(|(rank_idx, &(_hand, bid))| (rank_idx + 1) as u32 * bid)
        .sum();

    println!("Total winnings: {}", total_winnings);
}
