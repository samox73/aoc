use std::collections::{BTreeMap, HashSet};

use regex::Regex;

pub fn solve_a() {
    let input = aoc_utils::get_input(2023, 04);
    let points = get_total_points(&input);
    println!("part a: {}", points);
}

pub fn solve_b() {
    let input = aoc_utils::get_input(2023, 04);
    let mut map = get_map_of_winning_stacks(&input);
    process_card_stacks(&mut map);
    let solution: u32 = map.values().map(|v| v.card_count).sum();
    println!("{}", solution);
}

fn process_card_stacks(map: &mut BTreeMap<u64, CardStack>) {
    let len = u64::try_from(map.len()).unwrap();
    for id in 0..len {
        let count = map.get(&id).unwrap().card_count;
        let matches = map.get(&id).unwrap().matches_per_card;
        for i in 0..u64::try_from(matches).unwrap() {
            let idx = (id + i + 1).clamp(0, len - 1);
            map.get_mut(&idx).unwrap().card_count += count;
        }
    }
}

struct CardStack {
    matches_per_card: u64,
    card_count: u32,
}

struct Card {
    id: u64,
    targets: HashSet<u64>,
    numbers: HashSet<u64>,
}

fn get_map_of_winning_stacks(input: &str) -> BTreeMap<u64, CardStack> {
    input
        .lines()
        .map(|line| {
            let card = parse_card(line);
            let matches_per_card = u64::try_from(get_card_matches_counct(&card)).unwrap();
            let stack = CardStack {
                matches_per_card,
                card_count: 1,
            };
            (card.id - 1, stack)
        })
        .collect()
}

fn get_total_points(input: &str) -> u64 {
    input.lines().map(|line| get_points_of_line(line)).sum()
}

fn get_points_of_line(line: &str) -> u64 {
    let card = parse_card(line);
    let matches_count = get_card_matches_counct(&card);
    if matches_count == 0 {
        return 0;
    }
    println!("{} -> {}", line, u64::pow(2, matches_count - 1));
    return u64::pow(2, matches_count - 1);
}

fn get_card_matches_counct(card: &Card) -> u32 {
    let winners: HashSet<_> = card.numbers.intersection(&card.targets).collect();
    u32::try_from(winners.len()).unwrap()
}

fn parse_card(s: &str) -> Card {
    let re = Regex::new(r"Card\s+(?<id>\d+):(?<winners>[\d\s]+)\|(?<numbers>[\d\s]+)").unwrap();
    if let Some(c) = re.captures(s) {
        let w = &c["winners"];
        let n = &c["numbers"];
        let id = c["id"].parse::<u64>().unwrap();
        return Card {
            id,
            targets: get_int_vector(w),
            numbers: get_int_vector(n),
        };
    }
    Card {
        id: 0,
        targets: HashSet::new(),
        numbers: HashSet::new(),
    }
}

fn get_int_vector(s: &str) -> HashSet<u64> {
    String::from(s)
        .split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect::<HashSet<u64>>()
}
