fn extract_digits(line: &str) -> (u32, u32) {
    let mut p1_digits = Vec::new();
    let mut p2_digits = Vec::new();

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            p1_digits.push(c);
            p2_digits.push(c);
        }

        for (d, val) in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
            .iter()
            .enumerate()
        {
            if line[i..].starts_with(val) {
                p2_digits.push((d + 1).to_string().parse().unwrap());
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
        let (current_p1, current_p2) = extract_digits(line);
        part_1_sum += current_p1;
        part_2_sum += current_p2;
    }

    println!("Part I: {}\nPart II: {}", part_1_sum, part_2_sum);
}
