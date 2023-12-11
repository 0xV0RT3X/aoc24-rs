#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn empty_rows(image: &Vec<Vec<char>>) -> Vec<usize> {
    image
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            if row.iter().all(|&c| c == '.') {
                Some(row_idx)
            } else {
                None
            }
        })
        .collect()
}

fn empty_cols(image: &Vec<Vec<char>>) -> Vec<usize> {
    (0..image[0].len())
        .filter(|&col| image.iter().all(|row| row[col] == '.'))
        .collect()
}

fn galaxies(image: &Vec<Vec<char>>) -> Vec<Galaxy> {
    image
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|&(_, &col)| col == '#')
                .map(move |(col_idx, _)| Galaxy::new(row_idx, col_idx))
        })
        .collect()
}

fn galaxy_distance(g1: &Galaxy, g2: &Galaxy) -> usize {
    g1.row.saturating_sub(g2.row)
        + g2.row.saturating_sub(g1.row)
        + g1.col.saturating_sub(g2.col)
        + g2.col.saturating_sub(g1.col)
}

fn expand_galaxies(
    galaxies: &mut Vec<Galaxy>,
    empty_cols: &[usize],
    empty_rows: &[usize],
    scale: usize,
) {
    for galaxy in galaxies.iter_mut() {
        let mut col_expand = 0;
        for &empty_col in empty_cols {
            if empty_col < galaxy.col {
                col_expand += scale;
            } else {
                col_expand += 1;
            }
        }

        let mut row_expand = 0;
        for &empty_row in empty_rows {
            if empty_row < galaxy.row {
                row_expand += scale;
            } else {
                row_expand += 1;
            }
        }

        galaxy.col += col_expand;
        galaxy.row += row_expand;
    }
}

fn calculate_galaxy_distance_sum(galaxies: &[Galaxy]) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            sum += galaxy_distance(&galaxies[i], &galaxies[j]);
        }
    }
    sum
}

fn main() {
    let image = parse_input(include_str!("../input.txt"));
    let empty_cols = empty_cols(&image);
    let empty_rows = empty_rows(&image);

    let mut galaxies = galaxies(&image);

    expand_galaxies(&mut galaxies, &empty_cols, &empty_rows, 1_000_000);

    let sum = calculate_galaxy_distance_sum(&galaxies);

    println!("{}", sum);
}
