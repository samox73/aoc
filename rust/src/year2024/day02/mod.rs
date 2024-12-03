use itertools::Itertools;
use nom::character::complete::{i16, space1};
use nom::multi::separated_list1;
use nom::IResult;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 02);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 02);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_line(line: &str) -> IResult<&str, Vec<i16>> {
    let mut parser = separated_list1(space1, i16);
    parser(line)
}

fn is_safe(numbers: Vec<i16>) -> bool {
    let diffs = numbers
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect_vec();
    let in_positive_range = |&x| x >= 1 && x <= 3;
    let in_negative_range = |&x| x >= -3 && x <= -1;
    let pos = diffs.iter().filter(|&x| in_positive_range(x));
    let count_positives = pos.count();
    let count_negatives = diffs.iter().filter(|&x| in_negative_range(x)).count();
    if count_positives == diffs.len() || count_negatives == diffs.len() {
        return true;
    } else {
        return false;
    }
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        match parse_line(line) {
            Ok((_, numbers)) => {
                if is_safe(numbers) {
                    solution += 1;
                }
            }
            Err(_) => unreachable!(),
        }
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        match parse_line(line) {
            Ok((_, numbers)) => {
                let len = numbers.len();
                for i in 0..len {
                    let mut n: Vec<i16> = Vec::with_capacity(len - 1);
                    n.extend_from_slice(&numbers[..i]);
                    n.extend_from_slice(&numbers[i + 1..]);
                    if is_safe(n) {
                        solution += 1;
                        break;
                    }
                }
            }
            Err(_) => unreachable!(),
        }
    }
    solution
}
