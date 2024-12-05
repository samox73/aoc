use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use nom::{
    bytes::complete::tag, character::complete::u64, multi::separated_list1,
    sequence::separated_pair, IResult,
};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 05);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 05);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_rule(rule: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(u64, tag("|"), u64)(rule)
}

fn parse_rules(input: &str) -> HashMap<u64, HashSet<u64>> {
    let mut m = HashMap::new();
    for line in input.lines() {
        if let Ok((_, (k, v))) = parse_rule(line) {
            m.entry(k).or_insert(HashSet::new()).insert(v);
        } else {
            println!("could not parse rule: {}", line);
            exit(1);
        }
    }
    m
}

fn parse_update(update: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), u64)(update)
}

fn parse_updates(input: &str) -> Vec<Vec<u64>> {
    let mut updates = Vec::new();
    for line in input.lines() {
        if let Ok((_, u)) = parse_update(line) {
            updates.push(u);
        } else {
            println!("could not parse update: {}", line);
            exit(1);
        }
    }
    updates
}

fn parse_input(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let (input_rules, input_updates) = input.split_once("\n\n").unwrap();
    (parse_rules(input_rules), parse_updates(input_updates))
}

fn separate_updates(input: &str) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let mut correct = Vec::new();
    let mut incorrect = Vec::new();
    let (rules, updates) = parse_input(input);
    for update in updates {
        if let Some(_) = get_incorrect_update_index(&rules, &update) {
            incorrect.push(update);
        } else {
            correct.push(update);
        }
    }
    (correct, incorrect)
}

fn get_incorrect_update_index(
    rules: &HashMap<u64, HashSet<u64>>,
    update: &Vec<u64>,
) -> Option<usize> {
    let empty = HashSet::new();
    let mut pages_before = HashSet::new();
    for (i, page) in update.iter().enumerate() {
        let page_rules = rules.get(page).unwrap_or(&empty);
        if pages_before.intersection(page_rules).count() != 0 {
            return Some(i);
        }
        pages_before.insert(*page);
    }
    None
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let (correct, _) = separate_updates(input);
    for update in correct {
        solution += update.get(update.len() / 2).unwrap();
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let (_, incorrect) = separate_updates(input);
    let (rules, _) = parse_input(input);
    for mut update in incorrect {
        while let Some(i) = get_incorrect_update_index(&rules, &update) {
            update.swap(i, i - 1);
        }
        solution += update.get(update.len() / 2).unwrap();
    }
    solution
}
