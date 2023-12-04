extern crate test;
use nom::IResult;
use std::collections::{BTreeMap, HashSet};

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 04);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 04);
    b.iter(|| solve_b(&input));
}

pub fn solve_a(input: &str) {
    let solution = get_total_points(input);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let mut map = get_map_of_winning_stacks(input);
    process_card_stacks(&mut map);
    let solution: u32 = map.values().map(|v| v.card_count).sum();
    println!("part b: {}", solution);
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

#[derive(Debug, PartialEq)]
struct Card {
    id: u64,
    targets: HashSet<u64>,
    numbers: HashSet<u64>,
}

fn get_map_of_winning_stacks(input: &str) -> BTreeMap<u64, CardStack> {
    input
        .lines()
        .map(|line| {
            let card = parse_card(line).unwrap().1;
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
    let card = parse_card(line).unwrap().1;
    let matches_count = get_card_matches_counct(&card);
    if matches_count == 0 {
        return 0;
    }
    return u64::pow(2, matches_count - 1);
}

fn get_card_matches_counct(card: &Card) -> u32 {
    let winners: HashSet<_> = card.numbers.intersection(&card.targets).collect();
    u32::try_from(winners.len()).unwrap()
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, _) = nom::character::complete::alpha1(s)?;
    let (s, _) = nom::character::complete::multispace1(s)?;
    let (s, id) = nom::character::complete::u64(s)?;
    let (s, _) = nom::character::complete::char(':')(s)?;
    let (s, t) = nom::bytes::complete::take_until("|")(s)?;
    let (n, _) = nom::character::complete::char('|')(s)?;
    Ok((
        s,
        Card {
            id: id,
            targets: get_int_vector(t),
            numbers: get_int_vector(n),
        },
    ))
}

fn get_int_vector(s: &str) -> HashSet<u64> {
    String::from(s)
        .split_whitespace()
        .map(|n| n.trim().parse::<u64>().unwrap())
        .collect::<HashSet<u64>>()
}
