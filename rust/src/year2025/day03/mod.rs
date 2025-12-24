extern crate test;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, u64},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 03);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 03);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    map(digit1, |s: &str| {
        s.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()
    })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    nom::multi::separated_list0(line_ending, parse_line)(input)
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let Ok((_, parsed)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    for line in parsed {
        solution += find_largest_joltage(line, 2);
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let Ok((_, parsed)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    for line in parsed {
        solution += find_largest_joltage(line, 12);
    }
    solution
}

fn find_joltage_bank(line: &[u64]) -> (usize, &u64) {
    line.iter()
        .enumerate()
        .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1)))
        .unwrap()
}

fn find_largest_joltage(line: Vec<u64>, m: usize) -> u64 {
    let l = line.len();
    let mut next_idx = 0;
    let mut exponent = m as u32 - 1;
    let mut value = 0;
    for i in 1..=m {
        let range = next_idx..l - m + i;
        let subslice = &line[next_idx..l - m + i];
        let (idx, v) = find_joltage_bank(subslice);
        next_idx += idx + 1;
        value += 10u64.pow(exponent) * v;
        if exponent != 0 {
            exponent -= 1;
        }
    }
    value
}
