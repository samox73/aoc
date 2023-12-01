use std::collections::HashMap;

pub fn solve_a() {
    let input = aoc_utils::get_input(2023, 1);
    let numbers = extract_numbers(input);
    let sum: u32 = numbers.iter().sum();
    println!("{}", sum);
}

pub fn solve_b() {
    let input = aoc_utils::get_input(2023, 1);
    let numbers = extract_real_numbers(input);
    let sum: u32 = numbers.iter().sum();
    println!("{}", sum);
}

fn extract_numbers(input: String) -> Vec<u32> {
    let numbers = input
        .lines()
        .map(|line| {
            let first = find_fake(line, false);
            let last = find_fake(line, true);
            first * 10 + last
        })
        .collect();
    numbers
}

fn extract_real_numbers(input: String) -> Vec<u32> {
    let numbers = input
        .lines()
        .map(|line| {
            let first = find_real(line, false);
            let last = find_real(line, true);
            first * 10 + last
        })
        .collect();
    numbers
}

fn find_fake(line: &str, reverse: bool) -> u32 {
    let mut line = String::from(line);
    if reverse {
        line = line.chars().rev().collect();
    }
    for c in line.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    return 0;
}

fn find_real(line: &str, reverse: bool) -> u32 {
    let mapping: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut line = String::from(line);
    if reverse {
        line = line.chars().rev().collect();
    }
    for (i, c) in line.chars().enumerate() {
        for (word, digit) in &mapping {
            let mut word = String::from(*word);
            if reverse {
                word = word.chars().rev().collect();
            }
            if line[i..].starts_with(&word) {
                return *digit;
            }
        }
        if c.is_numeric() {
            return c.to_digit(10).unwrap();
        }
    }
    return 0;
}
