use std::collections::HashMap;

use nom::{
    character::complete::{space1, u64},
    sequence::separated_pair,
    IResult,
};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 01);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 01);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_vecs(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    for line in input.lines() {
        if let Ok((_, (l, r))) = parse_line(line) {
            v1.push(l);
            v2.push(r);
        }
    }
    Ok((input, (v1, v2)))
}

fn parse_vec_map(input: &str) -> IResult<&str, (Vec<u64>, HashMap<u64, u64>)> {
    let mut v1 = Vec::new();
    let mut v2 = HashMap::new();
    for line in input.lines() {
        if let Ok((_, (l, r))) = parse_line(line) {
            v1.push(l);
            *v2.entry(r).or_insert(0) += 1;
        }
    }
    Ok((input, (v1, v2)))
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
    let mut parser = separated_pair(u64, space1, u64);
    return parser(input);
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let (_, (mut v1, mut v2)) = parse_vecs(input).unwrap();
    v1.sort();
    v2.sort();
    for i in 0..v1.len() {
        solution += v2[i].abs_diff(v1[i]);
    }
    solution
}

fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let (_, (v1, v2)) = parse_vec_map(input).unwrap();
    for v in v1 {
        solution += v * v2.get(&v).unwrap_or(&0);
    }
    solution
}
