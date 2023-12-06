fn get_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect()
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let times = get_numbers(lines.next().unwrap());
    let distances = get_numbers(lines.next().unwrap());

    let mut sum = 1;
    for (time, distance) in times.iter().zip(distances) {
        let option_count = (1..*time)
            .filter(|&button_pressed_ms| (time - button_pressed_ms) * button_pressed_ms > distance)
            .count();
        sum *= option_count;
    }

    println!("{}", sum);
}
