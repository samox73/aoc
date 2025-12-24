use nom::{bytes::complete::tag, character::complete::u64, sequence::separated_pair, IResult};

extern crate test;

const UNIQUE_PRIME_FACTORS: [[usize; 2]; 13] = [
    [1, 0],  // 1
    [2, 0],  // 2
    [3, 0],  // 3
    [2, 0],  // 4
    [5, 0],  // 5
    [2, 3],  // 6
    [7, 0],  // 7
    [2, 0],  // 8
    [3, 0],  // 9
    [2, 5],  // 10
    [11, 0], // 11
    [2, 3],  // 12
    [13, 0], // 13
];

const POW10: [u64; 11] = [
    1,           // 0
    10,          // 1
    100,         // 2
    1000,        // 3
    10000,       // 4
    100000,      // 5
    1000000,     // 6
    10000000,    // 7
    100000000,   // 8
    1000000000,  // 9
    10000000000, // 10
];

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 2);
    let mut solution = 0;
    let Ok((_, ranges)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    b.iter(|| {
        for (start, end) in &ranges {
            solution += sum_invalid_ids_for_range(*start, *end, true);
        }
    });
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 2);
    let mut solution = 0;
    let Ok((_, ranges)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    b.iter(|| {
        for (start, end) in &ranges {
            solution += sum_invalid_ids_for_range(*start, *end, false);
        }
    });
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    nom::multi::separated_list0(tag(","), separated_pair(u64, tag("-"), u64))(input)
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let Ok((_, ranges)) = parse(input) else {
        panic!("could not parse '{input}'")
    };
    for (start, end) in ranges {
        solution += sum_invalid_ids_for_range(start, end, true);
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let Ok((_, ranges)) = parse(input) else {
        panic!("could not parse '{input}'")
    };
    for (start, end) in ranges {
        solution += sum_invalid_ids_for_range(start, end, false);
    }
    solution
}

fn sum_invalid_ids_for_range(start: u64, end: u64, part_a: bool) -> u64 {
    let mut count = 0;
    // significantly faster than string allocation
    let len_start = start.ilog10() as usize + 1;
    let len_end = end.ilog10() as usize + 1;
    for len in len_start..=len_end {
        if len == 1 {
            continue;
        }
        let primes = UNIQUE_PRIME_FACTORS[len - 1];
        for prime in primes {
            if prime == 0 {
                continue;
            }
            if part_a && prime != 2 {
                continue;
            }
            let digits = len / prime;
            count += sum_invalid_ids(start, end, len, digits);
        }

        // correct for overcounting
        if primes[0] != 0 && primes[1] != 0 && !part_a {
            count -= sum_invalid_ids(start, end, len, 1);
        }
    }
    count
}

fn sum_invalid_ids(start: u64, end: u64, len: usize, digits: usize) -> u64 {
    let f = ((POW10[len] - 1) / (POW10[digits] - 1)) as u64;
    let a = start.max(POW10[len - 1]).div_ceil(f);
    let b = end.min(POW10[len]) / f;
    if b < a {
        return 0;
    }
    // sum_{i=a}^{b} i = (b^2 + b - a^2 + a) / 2
    f * (b.pow(2) - a.pow(2) + a + b) / 2
}
