use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn main() {
    match run() {
        Ok(safe_reports) => println!("Number of safe reports: {}", safe_reports),
        Err(e) => eprintln!("Them elves got it wrong: {}", e),
    }
}

fn run() -> io::Result<usize> {
    let path = Path::new("./puzzle_inputs/day_two_reports.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let safe_reports = reader
        .lines()
        .map_while(|line| line.ok())
        .filter(|line| is_safe_report(line))
        .count();

    Ok(safe_reports)
}
fn is_safe_report(report: &str) -> bool {
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut all_increasing = true;
    let mut all_decreasing = true;
    for window in levels.windows(2) {
        let difference = window[1] - window[0];
        if difference <= 0 {
            all_increasing = false;
        }
        if difference >= 0 {
            all_decreasing = false;
        }
        if !(1..=3).contains(&difference.abs()) {
            return false;
        }
    }
    all_increasing || all_decreasing
}
