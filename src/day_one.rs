use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() -> io::Result<()> {
    let path = Path::new("./puzzle_inputs/day_one_lists.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        left_list.push(numbers[0]);
        right_list.push(numbers[1]);
    }
    left_list.sort();
    right_list.sort();
    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!(
        "The total distance between the lists is: {}",
        total_distance
    );

    Ok(())
}
