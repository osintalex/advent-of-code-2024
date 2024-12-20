use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./puzzle_inputs/day_seven_calibrators.txt")?;
    let reader = BufReader::new(file);
    let mut total_calibration = 0;
    for line in reader.lines() {
        let read_line = line?;
        let parts: Vec<&str> = read_line.split(": ").collect();
        let test_value: i64 = parts[0].parse().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if can_make_equation(test_value, &numbers) {
            total_calibration += test_value;
        }
    }

    println!("Total calibration result: {}", total_calibration);
    Ok(())
}

fn can_make_equation(target: i64, numbers: &[i64]) -> bool {
    let combinations = generate_operator_combinations(numbers.len() - 1);
    for combination in combinations {
        if apply_operations(numbers, &combination) == target {
            return true;
        }
    }
    false
}

fn generate(current_combos: &mut Vec<char>, remaining: usize, combinations: &mut Vec<Vec<char>>) {
    if remaining == 0 {
        combinations.push(current_combos.clone());
        return;
    }

    current_combos.push('+');
    generate(current_combos, remaining - 1, combinations);
    current_combos.pop();

    current_combos.push('*');
    generate(current_combos, remaining - 1, combinations);
    current_combos.pop();
}

fn generate_operator_combinations(n: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let mut current_combos = Vec::new();
    generate(&mut current_combos, n, &mut combinations);
    combinations
}
fn apply_operations(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => unreachable!(),
        }
    }
    result
}
