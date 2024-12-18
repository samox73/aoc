use std::collections::HashMap;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 11);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 11);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn get_new_values(v: u64) -> (u64, Option<u64>) {
    if v == 0 {
        (1, None)
    } else if (v).to_string().len() % 2 == 0 {
        let len = (v).to_string().len();
        let v1 = v.to_string()[..len / 2].parse().unwrap();
        let v2 = v.to_string()[len / 2..].parse().unwrap();
        (v1, Some(v2))
    } else {
        (v * 2024, None)
    }
}

fn blink(mut stones: HashMap<u64, u64>, blinkies: usize) -> HashMap<u64, u64> {
    for _ in 0..blinkies {
        let mut new_stones = HashMap::new();
        for (stone, count) in &stones {
            let (left, right) = get_new_values(*stone);
            *new_stones.entry(left).or_insert(0) += *count;
            if let Some(new) = right {
                *new_stones.entry(new).or_insert(0) += *count;
            }
        }
        stones = new_stones;
    }
    stones
}

fn get_stones(input: &str) -> HashMap<u64, u64> {
    input
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect()
}

fn solve_a(input: &str) -> u64 {
    let mut stones = get_stones(input);
    stones = blink(stones, 25);
    stones.values().sum::<u64>()
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut stones = get_stones(input);
    stones = blink(stones, 75);
    stones.values().sum::<u64>()
}
