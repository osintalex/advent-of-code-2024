use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./puzzle_inputs/day_six_guard_path.txt")
        .expect("Guard nowhere to be found ?!?!£(£$)@£$)(");
    let result = get_positions(&input);
    println!("The guard visited {} positions", result);
}

fn get_positions(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
    let mut visited = HashSet::new();
    let mut position = find_start(&grid);
    let mut direction = (0, -1);
    visited.insert(position);
    loop {
        let (y, x) = position;
        let (dx, dy) = direction;
        let next_pos = (y + dy, x + dx);
        if next_pos.0 >= rows
            || next_pos.1 >= cols
            || grid[next_pos.0 as usize][next_pos.1 as usize] == '#'
        {
            // Turn 90 degrees clockwise
            direction = match direction {
                (0, -1) => (1, 0),  // Up to Right
                (1, 0) => (0, 1),   // Right to Down
                (0, 1) => (-1, 0),  // Down to Left
                (-1, 0) => (0, -1), // Left to Up
                _ => unreachable!(),
            };
        } else {
            position = next_pos;
            visited.insert(position);
            // Check if guard has left the mapped area
            // i.e. see if over/under max rows and/or cols
            if position.0 == 0
                || position.0 == rows - 1
                || position.1 == 0
                || position.1 == cols - 1
            {
                break;
            }
        }
    }
    visited.len() as i32
}

fn find_start(grid: &[Vec<char>]) -> (i32, i32) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("No starting position found - wuut");
}
