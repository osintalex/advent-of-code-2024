use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./puzzle_inputs/day_three_corrupted_memories.txt")
        .expect("Failed to read input file, maybe my memory corrupted too");

    let result = process_corrupted_memory(&input);
    println!("The sum of all multiplication results is: {}", result);
}

fn process_corrupted_memory(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("Bad regex what you smoking");
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let x: i32 = cap[1].parse().expect("Could not parse first number");
        let y: i32 = cap[2].parse().expect("Could not parse second number");
        sum += x * y;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_corrupted_memory() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_corrupted_memory(input), 161);
    }
}
