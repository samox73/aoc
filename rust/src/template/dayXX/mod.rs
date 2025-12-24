extern crate test;

use nom::{bytes::complete::tag, character::complete::u64, sequence::separated_pair, IResult};

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(YYYY, DD);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(YYYY, DD);
    b.iter(|| solve_b(&input));
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
    let solution = 0;
    println!("{}", input);
    let Ok((_, parsed)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let solution = 0;
    let Ok((_, parsed)) = parse(&input) else {
        panic!("could not parse '{input}'")
    };
    solution
}
