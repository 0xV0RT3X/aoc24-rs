use std::collections::{HashSet, VecDeque};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct GridPosition {
    row: usize,
    col: usize,
}

impl GridPosition {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_starting_position(grid: &Vec<Vec<char>>) -> Option<GridPosition> {
    for (row_idx, row) in grid.iter().enumerate() {
        if let Some(col_idx) = row.iter().position(|&col| col == 'S') {
            return Some(GridPosition::new(row_idx, col_idx));
        }
    }

    None
}

fn steps_to_farthest_point(grid: &Vec<Vec<char>>) -> Option<usize> {
    if let Some(starting_position) = find_starting_position(grid) {
        let mut seen_position: HashSet<GridPosition> = HashSet::new();
        let mut position_queue: VecDeque<GridPosition> = VecDeque::new();

        seen_position.insert(starting_position);
        position_queue.push_back(starting_position);

        while let Some(current_position) = position_queue.pop_front() {
            let row = current_position.row;
            let col = current_position.col;
            let ch = grid[row][col];

            if row > 0
                && "S|JL".contains(ch)
                && "|7F".contains(grid[row - 1][col])
                && !seen_position.contains(&GridPosition::new(row - 1, col))
            {
                seen_position.insert(GridPosition::new(row - 1, col));
                position_queue.push_back(GridPosition::new(row - 1, col));
            }

            if row < grid.len() - 1
                && "S|7F".contains(ch)
                && "|JL".contains(grid[row + 1][col])
                && !seen_position.contains(&GridPosition::new(row + 1, col))
            {
                seen_position.insert(GridPosition::new(row + 1, col));
                position_queue.push_back(GridPosition::new(row + 1, col));
            }

            if col > 0
                && "S-J7".contains(ch)
                && "-LF".contains(grid[row][col - 1])
                && !seen_position.contains(&GridPosition::new(row, col - 1))
            {
                seen_position.insert(GridPosition::new(row, col - 1));
                position_queue.push_back(GridPosition::new(row, col - 1));
            }

            if col < grid[row].len() - 1
                && "S-LF".contains(ch)
                && "-J7".contains(grid[row][col + 1])
                && !seen_position.contains(&GridPosition::new(row, col + 1))
            {
                seen_position.insert(GridPosition::new(row, col + 1));
                position_queue.push_back(GridPosition::new(row, col + 1));
            }
        }
        return Some(seen_position.len() / 2);
    }

    None
}

fn main() {
    let grid = parse_input(include_str!("../input.txt"));

    if let Some(steps) = steps_to_farthest_point(&grid) {
        println!("{}", steps);
    }
}
