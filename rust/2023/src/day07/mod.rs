use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use nom::{
    bytes::complete::{take_till, take_until},
    character::complete::{multispace0, multispace1},
    multi::many1,
    IResult,
};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 07);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 07);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let mut hands = parse_hands(input).unwrap().1;
    hands.sort();
    let winnings = get_total_winnings(hands);
    println!("part a: {}", winnings);
}

pub fn solve_b(input: &str) {
    let mut hands = parse_hands(input).unwrap().1;
    hands.sort_by(part_b_sort);
    let winnings = get_total_winnings(hands);
    println!("part b: {}", winnings);
}

#[derive(Eq, Debug)]
struct Hand {
    value: String,
    bid: u64,
}

fn part_b_sort(left: &Hand, right: &Hand) -> Ordering {
    let left_counts = get_distinct_counts(&left, true);
    let right_counts = get_distinct_counts(&right, true);
    if left_counts == right_counts {
        let left = get_value_mapping_2(&left.value);
        let right = get_value_mapping_2(&right.value);
        return left.cmp(&right);
    }
    return left_counts.cmp(&right_counts);
}

fn get_value_mapping(value: &str) -> Vec<u64> {
    let mapping: HashMap<char, u64> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    value.chars().map(|c| *mapping.get(&c).unwrap()).collect()
}

fn get_value_mapping_2(value: &str) -> Vec<u64> {
    let mapping: HashMap<char, u64> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    value.chars().map(|c| *mapping.get(&c).unwrap()).collect()
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_counts = get_distinct_counts(&self, false);
        let other_counts = get_distinct_counts(&other, false);
        if self_counts == other_counts {
            let left = get_value_mapping(&self.value);
            let right = get_value_mapping(&other.value);
            return left.cmp(&right);
        }
        return self_counts.cmp(&other_counts);
    }
}

fn get_distinct_counts(hand: &Hand, allow_wildcard: bool) -> Vec<u64> {
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    for c in hand.value.chars() {
        let counter = char_counts.entry(c).or_insert(0);
        *counter += 1;
    }
    let mut modifier = 0;
    if allow_wildcard && char_counts.keys().contains(&'J') {
        modifier = char_counts.remove(&'J').unwrap();
    }
    let mut counts: Vec<u64> = char_counts.values().cloned().collect();
    counts.sort();
    counts.reverse();
    if counts.is_empty() {
        counts.push(modifier);
    } else {
        counts[0] += modifier;
    }
    counts
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (&self.value, self.bid) == (&other.value, other.bid)
    }
}

fn get_total_winnings(hands: Vec<Hand>) -> u64 {
    let mut total = 0;
    for (rank, hand) in hands.iter().enumerate() {
        let winnings = hand.bid * (rank as u64 + 1);
        println!("{} -> {} -> {}", hand.value, hand.bid, winnings);
        total += winnings;
    }
    total
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, hands) = many1(|s| -> IResult<&str, Hand> {
        let (s, value) = take_until(" ")(s)?;
        let (s, _) = multispace1(s)?;
        let (s, bid) = nom::character::complete::u64(s)?;
        let (s, _) = multispace0(s)?;
        let hand = Hand {
            value: value.to_string(),
            bid,
        };
        Ok((s, hand))
    })(input)?;
    return Ok((input, hands));
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::solve_b;

    #[test]
    fn test_vec_sort() {
        let v1 = vec![2, 2, 1];
        let v2 = vec![3, 1, 1];
        let c = v1.cmp(&v2);
        match c {
            Ordering::Equal => println!("equal"),
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
        }
    }

    #[test]
    fn test_j_ordering() {
        let input = "JJJJJ 1
AAAA2 2
AAAAJ 3
TTTTT 4
22222 5";
        solve_b(&input);
    }
}
