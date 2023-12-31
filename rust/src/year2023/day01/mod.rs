use std::collections::HashMap;

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let numbers = extract_numbers(input);
    let sum: u64 = numbers.iter().sum();
    println!("{}", sum);
    sum
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let numbers = extract_real_numbers(input);
    let sum: u64 = numbers.iter().sum();
    println!("{}", sum);
    sum
}

fn extract_numbers(input: &str) -> Vec<u64> {
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

fn extract_real_numbers(input: &str) -> Vec<u64> {
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

fn find_fake(line: &str, reverse: bool) -> u64 {
    let mut line = String::from(line);
    if reverse {
        line = line.chars().rev().collect();
    }
    for c in line.chars() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as u64;
        }
    }
    return 0;
}

fn find_real(line: &str, reverse: bool) -> u64 {
    let mapping: HashMap<&str, u64> = HashMap::from([
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
            return c.to_digit(10).unwrap() as u64;
        }
    }
    return 0;
}
