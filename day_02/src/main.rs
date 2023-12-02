use regex::Regex;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

// Function to solve day 2
fn solve_day_2() -> (usize, u32) {
    // Read input file into lines
    let games = include_str!("../input.txt").lines();

    // Regular expression to extract cube count and color
    let re = Regex::new(r"(\d+) (\w+)").unwrap();

    // Initialize variables to store the sum of product sets and game IDs
    let mut sum_product_set = 0;
    let mut sum_game_ids = 0;

    // Iterate over each game
    for (game_idx, game) in games.enumerate() {
        // Check if the game is possible and initialize max counts for each color
        let (game_possible, max_red, max_green, max_blue) = process_game(&re, game);

        // Update sum_product_set with the product of max counts
        sum_product_set += max_red * max_green * max_blue;

        // If the game is possible, update sum_game_ids
        if game_possible {
            sum_game_ids += game_idx + 1;
        }
    }

    (sum_game_ids, sum_product_set)
}

// Function to process each game and return game_possible flag and max counts for each color
fn process_game(re: &Regex, game: &str) -> (bool, u32, u32, u32) {
    let mut game_possible = true;
    let mut max_red = 1;
    let mut max_green = 1;
    let mut max_blue = 1;

    // Iterate over each match in the game using the regular expression
    for (_, [cube_count, color]) in re.captures_iter(game).map(|c| c.extract()) {
        let (cube_count, color) = (cube_count.parse::<u32>().unwrap(), color);

        // Check if the game is still possible after processing the cube
        if game_possible && !is_game_possible((cube_count, color)) {
            game_possible = false;
        }

        // Update max counts for each color
        match color {
            "red" => max_red = max_red.max(cube_count),
            "green" => max_green = max_green.max(cube_count),
            "blue" => max_blue = max_blue.max(cube_count),
            _ => (),
        }
    }

    (game_possible, max_red, max_green, max_blue)
}

// Function to check if a game with the given cube count and color is possible
fn is_game_possible((cube_count, color): (u32, &str)) -> bool {
    match color {
        "blue" | "red" | "green" => {
            cube_count
                <= match color {
                    "blue" => MAX_BLUE_CUBES,
                    "red" => MAX_RED_CUBES,
                    "green" => MAX_GREEN_CUBES,
                    _ => 0,
                }
        }
        _ => false,
    }
}

fn main() {
    // Call the solve_day_2 function to get the results
    let (p1, p2) = solve_day_2();

    // Assertions to check if the results match the expected values
    assert_eq!(p1, 2_512);
    assert_eq!(p2, 67_335);

    // Print the results
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}
