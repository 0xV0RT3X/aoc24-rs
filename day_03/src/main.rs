use std::collections::HashSet;

/// Represents the schematic of the engine.
type EngineSchematic = Vec<Vec<char>>;

/// Represents the position of an element in the engine schematic.
type Position = (usize, usize);

/// Retrieves the engine schematic from the input file.
fn get_engine_schematic() -> EngineSchematic {
    include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

/// Finds the start index of a number given a digit position.
fn get_number_start_idx(engine_schematic: &EngineSchematic, digit_position: Position) -> Position {
    let (row, mut col) = digit_position;

    while col > 0 && char::is_digit(engine_schematic[row][col - 1], 10) {
        col -= 1;
    }

    (row, col)
}

/// Parses a number from the given start index.
fn get_number_from_start_idx(engine_schematic: &EngineSchematic, start_idx: Position) -> u32 {
    let digits: String = engine_schematic[start_idx.0]
        .iter()
        .skip(start_idx.1)
        .take_while(|&&c| char::is_digit(c, 10))
        .collect();

    digits.parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse number");
        0
    })
}

/// Macro to insert a digit into the result vector if it satisfies certain conditions.
macro_rules! insert_if_digit {
    ($engine_schematic:ident, $seen_positions:ident, $inserted_elements:ident, $max_rows:expr, $max_cols:expr, $r:expr, $c:expr) => {
        if $r < $max_rows && $c < $max_cols && char::is_digit($engine_schematic[$r][$c], 10) {
            let position = get_number_start_idx($engine_schematic, ($r, $c));
            if $seen_positions.insert(position) {
                $inserted_elements.push(get_number_from_start_idx($engine_schematic, position));
            }
        }
    };
}

/// Retrieves adjacent numbers from the given position.
fn get_adjacent_numbers(
    engine_schematic: &EngineSchematic,
    seen_positions: &mut HashSet<Position>,
    symbol_position: Position,
) -> Vec<u32> {
    let (row, col) = symbol_position;
    let max_rows = engine_schematic.len();
    let max_cols = engine_schematic[0].len();
    let mut inserted_elements = Vec::new();

    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row,
        col - 1
    ); // Left
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row,
        col + 1
    ); // Right
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row + 1,
        col
    ); // Below
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row - 1,
        col
    ); // Above

    inserted_elements
}

/// Retrieves diagonal numbers from the given position.
fn get_diagonal_numbers(
    engine_schematic: &EngineSchematic,
    seen_positions: &mut HashSet<Position>,
    symbol_position: Position,
) -> Vec<u32> {
    let (row, col) = symbol_position;
    let max_rows = engine_schematic.len();
    let max_cols = engine_schematic[0].len();
    let mut inserted_elements = Vec::new();

    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row - 1,
        col + 1
    ); // Top Right
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row - 1,
        col - 1
    ); // Top Left
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row + 1,
        col + 1
    ); // Bottom Right
    insert_if_digit!(
        engine_schematic,
        seen_positions,
        inserted_elements,
        max_rows,
        max_cols,
        row + 1,
        col - 1
    ); // Bottom Left

    inserted_elements
}

/// Main function to calculate gear ratio and sum of numbers in the engine schematic.
fn main() {
    let engine_schematic = get_engine_schematic();
    let mut seen_positions: HashSet<Position> = HashSet::new();
    let mut gear_ratio = 0;

    for (row_idx, row) in engine_schematic.iter().enumerate() {
        for (col_idx, &element) in row.iter().enumerate() {
            if char::is_digit(element, 10) || element == '.' {
                continue;
            }

            let adjacent =
                get_adjacent_numbers(&engine_schematic, &mut seen_positions, (row_idx, col_idx));
            let diagonal =
                get_diagonal_numbers(&engine_schematic, &mut seen_positions, (row_idx, col_idx));

            if element == '*' {
                let combined: Vec<u32> = adjacent.iter().chain(diagonal.iter()).cloned().collect();

                if combined.len() == 2 {
                    gear_ratio += combined.iter().product::<u32>();
                }
            }
        }
    }

    let p1_sum = seen_positions
        .iter()
        .map(|&pos| get_number_from_start_idx(&engine_schematic, pos))
        .sum::<u32>();

    assert_eq!(p1_sum, 530_849);
    assert_eq!(gear_ratio, 84_900_879);

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", gear_ratio);
}
