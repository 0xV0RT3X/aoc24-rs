fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n_str| n_str.parse::<i32>().expect("Not a number"))
                .collect()
        })
        .collect()
}

fn extrapolate(history: &[i32]) -> i32 {
    if history.iter().all(|&n| n == 0) {
        return 0;
    }

    let diffs: Vec<i32> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let diff = extrapolate(&diffs);

    // Part 1
    //history[history.len() - 1] + diff

    // Part 2
    history[0] - diff
}

fn main() {
    let histories = parse_input(include_str!("../input.txt"));

    let sum = histories
        .iter()
        .map(|history| extrapolate(history))
        .sum::<i32>();
    println!("{:?}", sum);
}
