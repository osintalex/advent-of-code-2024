use std::collections::{HashMap, HashSet};
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./puzzle_inputs/day_five_safety_manuals.txt")
        .expect("Failed to read input file, safety manuals also corrupted");
    let (rules, updates) = parse_input(&input);
    let mut sum = 0;
    for update in updates {
        if is_valid_order(&update, &rules) {
            let middle_index = update.len() / 2;
            sum += update[middle_index] as u32;
        }
    }
    println!("The sum of all middle numbers is: {}", sum);
}

fn parse_input(input: &str) -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    let mut parts = input.split("\n\n");
    let rules_str = parts.next().unwrap();
    let updates_str = parts.next().unwrap();
    let mut rules = HashMap::new();
    // now get all the pages that must come after the given page
    // key in rules is the page that must come first
    // value is the set of rules that must come after
    for rule in rules_str.lines() {
        let before_after_rules: Vec<&str> = rule.split('|').collect();
        let before = before_after_rules[0].parse().unwrap();
        let after = before_after_rules[1].parse().unwrap();
        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }
    let updates = updates_str
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<u8>().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid_order(update: &[u8], rules: &HashMap<u8, HashSet<u8>>) -> bool {
    for (count, &page) in update.iter().enumerate() {
        if let Some(after_pages) = rules.get(&page) {
            for &after_page in after_pages {
                // now basically check if it adheres to the rules by seeing if the page that
                // must come after is in the rest of the update entry; if it isn't return false
                if update[count + 1..].contains(&after_page) {
                    continue;
                }
                if update[..count].contains(&after_page) {
                    return false;
                }
            }
        }
    }
    true
}
