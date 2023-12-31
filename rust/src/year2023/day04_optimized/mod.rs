extern crate test;
use nom::IResult;
use std::collections::BTreeMap;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 04);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 04);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let solution = get_total_points(input);
    solution as u64
}

fn solve_b(input: &str) -> u64 {
    let mut map = get_map_of_winning_stacks(input);
    process_card_stacks(&mut map);
    let solution: u32 = map.values().map(|v| v.card_count).sum();
    solution as u64
}

fn process_card_stacks(map: &mut BTreeMap<u32, CardStack>) {
    let len = u32::try_from(map.len()).unwrap();
    for id in 0..len {
        let count = map.get(&id).unwrap().card_count;
        let matches = map.get(&id).unwrap().matches_per_card;
        for i in 0..u32::try_from(matches).unwrap() {
            let idx = (id + i + 1).clamp(0, len - 1);
            map.get_mut(&idx).unwrap().card_count += count;
        }
    }
}

struct CardStack {
    matches_per_card: u32,
    card_count: u32,
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u32,
    targets: Vec<u32>,
    numbers: Vec<u32>,
}

fn get_map_of_winning_stacks(input: &str) -> BTreeMap<u32, CardStack> {
    input
        .lines()
        .map(|line| {
            let card = parse_card(line).unwrap().1;
            let matches_per_card = get_card_matches_count(&card);
            let stack = CardStack {
                matches_per_card,
                card_count: 1,
            };
            (card.id - 1, stack)
        })
        .collect()
}

fn get_total_points(input: &str) -> u32 {
    input.lines().map(|line| get_points_of_line(line)).sum()
}

fn get_points_of_line(line: &str) -> u32 {
    let card = parse_card(line).unwrap().1;
    let matches_count = get_card_matches_count(&card);
    if matches_count == 0 {
        return 0;
    }
    return u32::pow(2, matches_count - 1);
}

fn get_card_matches_count(card: &Card) -> u32 {
    let mut c = 0;
    for t in &card.targets {
        if card.numbers.contains(&t) {
            c += 1
        }
    }
    return c;
}

fn parse_card(s: &str) -> IResult<&str, Card> {
    let (s, _) = nom::character::complete::alpha1(s)?;
    let (s, _) = nom::character::complete::multispace1(s)?;
    let (s, id) = nom::character::complete::u32(s)?;
    let (s, _) = nom::character::complete::char(':')(s)?;
    let (s, t) = nom::bytes::complete::take_until("|")(s)?;
    let (n, _) = nom::character::complete::char('|')(s)?;
    Ok((
        s,
        Card {
            id,
            targets: get_int_vector(t, 20).unwrap().1,
            numbers: get_int_vector(n, 50).unwrap().1,
        },
    ))
}

#[inline(always)]
fn get_int_vector(s: &str, c: usize) -> IResult<&str, Vec<u32>> {
    let (s, v) = nom::multi::many_m_n(c, c, |s| -> IResult<&str, u32> {
        let (s, _) = nom::character::complete::multispace0(s)?;
        let (s, id) = nom::character::complete::u32(s)?;
        return Ok((s, id));
    })(s)?;
    return Ok((s, v));
}
