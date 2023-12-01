fn extract_digits(line: &str) -> (u32, u32) {
    let mut p1_digits = Vec::new();
    let mut p2_digits = Vec::new();

    for (line_char_idx, line_char) in line.chars().enumerate() {
        if char::is_digit(line_char, 10) {
            p1_digits.push(line_char);
            p2_digits.push(line_char);
        }

        for (spelled_digit_idx, spelled_digit) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            if line[line_char_idx..].starts_with(spelled_digit) {
                p2_digits.push((spelled_digit_idx + 1).to_string().parse().unwrap());
            }
        }
    }

    (parse_digits(p1_digits), parse_digits(p2_digits))
}

fn parse_digits(digits: Vec<char>) -> u32 {
    if !digits.is_empty() {
        format!("{}{}", digits[0], digits.last().unwrap())
            .parse::<u32>()
            .unwrap()
    } else {
        0
    }
}

fn main() {
    let lines = include_str!("../input.txt").lines();

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;

    for line in lines {
        let (part_1_number, part_2_number) = extract_digits(line);
        part_1_sum += part_1_number;
        part_2_sum += part_2_number;
    }

    assert_eq!(part_1_sum, 54_916);
    assert_eq!(part_2_sum, 54_728);

    println!("Part 1: {}\nPart 2: {}", part_1_sum, part_2_sum);
}
