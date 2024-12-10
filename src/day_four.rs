use std::fs;

fn main() {
    let input = fs::read_to_string("./puzzle_inputs/day_four_xmas_scramble.txt")
        .expect("My brain got scrambled oh no oh no oh no");

    let result = extract_xmas_counts(&input);
    println!("XMAS appears {} times", result);
}

fn extract_xmas_counts(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_xmas_counts = 0;
    for row_number in 0..rows {
        for col_number in 0..cols {
            total_xmas_counts += check_xmas(
                &grid,
                row_number as i32,
                col_number as i32,
                rows as i32,
                cols as i32,
            );
        }
    }
    total_xmas_counts
}

/// This function implements a grid search algorithm that examines the surrounding
/// cells in all directions (horizontal, vertical, and diagonal) from a given
/// starting position. It searches for the sequence of characters 'X', 'M', 'A', 'S'
/// in any of these directions. The function is used to count the number of times
/// the word "XMAS" appears in the grid.
fn check_xmas(grid: &[Vec<char>], row_number: i32, col_number: i32, rows: i32, cols: i32) -> i32 {
    let directions = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let xmas_characters = ['X', 'M', 'A', 'S'];
    let mut xmas_counts = 0;
    for &(dx, dy) in &directions {
        let mut found_xmas = true;
        for k in 0..4 {
            let x_pos = row_number + k * dx;
            let y_pos = col_number + k * dy;
            // first check if x/y are out of bounds
            if x_pos < 0
                || x_pos >= rows
                || y_pos < 0
                || y_pos >= cols
                // then check if the character at x/y matches any XMAS character
                || grid[x_pos as usize][y_pos as usize] != xmas_characters[k as usize]
            {
                found_xmas = false;
                break;
            }
        }
        if found_xmas {
            xmas_counts += 1;
        }
    }
    xmas_counts
}

#[cfg(test)]
mod tests {
    use super::check_xmas;

    #[test]
    fn test_check_xmas_grid_search() {
        let grid = vec![
            vec!['X', 'M', 'A', 'S', 'X'],
            vec!['A', 'X', 'M', 'X', 'M'],
            vec!['M', 'A', 'S', 'A', 'A'],
            vec!['S', 'S', 'X', 'S', 'S'],
            vec!['X', 'M', 'A', 'S', 'X'],
        ];

        assert_eq!(check_xmas(&grid, 0, 0, 5, 5), 1); // Horizontal XMAS
        assert_eq!(check_xmas(&grid, 0, 4, 5, 5), 1); // Vertical XMAS
        assert_eq!(check_xmas(&grid, 4, 0, 5, 5), 1); // Diagonal XMAS
        assert_eq!(check_xmas(&grid, 2, 2, 5, 5), 0); // No XMAS
        assert_eq!(check_xmas(&grid, 1, 1, 5, 5), 0); // Partial XMAS (XM only)
    }
}
