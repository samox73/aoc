use std::collections::HashMap;

use itertools::Itertools;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 12);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 12);
    b.iter(|| solve_b(&input));
}
#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    println!("'{input}'");
    let input = input.trim();
    let solution = input
        .split('\n')
        .map(|l| {
            let (slots, counts) = l.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut cache = HashMap::new();
            count_solutions(&mut cache, slots.as_bytes(), None, &counts)
        })
        .sum::<usize>();
    solution as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let input = input.trim();
    let solution = input
        .split('\n')
        .map(|l| {
            let (slots, counts) = l.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|count| count.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let slots = (0..5).map(|_| slots).join("?");
            let counts = (0..5)
                .flat_map(|_| &counts)
                .copied()
                .collect::<Vec<usize>>();
            let mut cache = HashMap::new();
            count_solutions(&mut cache, slots.as_bytes(), None, &counts)
        })
        .sum::<usize>();
    solution as u64
}

fn count_solutions(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    slots: &[u8],
    block: Option<usize>,
    counts: &[usize],
) -> usize {
    if slots.is_empty() {
        return match (block, counts.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == counts[0] => 1,
            _ => 0,
        };
    }
    if block.is_some() && counts.is_empty() {
        return 0;
    }

    let key = (slots.len(), block.unwrap_or(0), counts.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }
    // let chars: String = slots.iter().map(|b| b.as_char()).collect();
    // println!(
    //     "block: {:?}, remaining: {:?}, slots: {chars}",
    //     block, counts
    // );

    let solutions_count = match (slots[0], block) {
        (b'.', Some(x)) if x != counts[0] => 0,
        (b'.', Some(_)) => count_solutions(cache, &slots[1..], None, &counts[1..]),
        (b'.', None) => count_solutions(cache, &slots[1..], None, counts),
        (b'#', Some(_)) => count_solutions(cache, &slots[1..], block.map(|x| x + 1), counts),
        (b'#', None) => count_solutions(cache, &slots[1..], Some(1), counts),
        (b'?', Some(x)) => {
            let mut ans = count_solutions(cache, &slots[1..], block.map(|x| x + 1), counts);
            if x == counts[0] {
                ans += count_solutions(cache, &slots[1..], None, &counts[1..]);
            }
            ans
        }
        (b'?', None) => {
            count_solutions(cache, &slots[1..], Some(1), counts)
                + count_solutions(cache, &slots[1..], None, counts)
        }
        _ => unreachable!(),
    };
    cache.insert(key, solutions_count);
    solutions_count
}
