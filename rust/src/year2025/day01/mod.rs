use nom::{
    branch::alt,
    character::complete::{char, i32},
    combinator::map,
    sequence::pair,
    IResult,
};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 01);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2025, 01);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_line(l: &str) -> IResult<&str, (i32, i32)> {
    pair(alt((map(char('L'), |_| -1), map(char('R'), |_| 1))), i32)(l)
}

fn solve_a(input: &str) -> u64 {
    let mut count = 0;
    let mut pos: i32 = 50;
    for line in input.lines() {
        let Ok((_, (sign, amount))) = parse_line(line) else {
            panic!("could not parse '{line}'")
        };
        pos = (pos + sign * amount).rem_euclid(100);
        if pos == 0 {
            count += 1
        }
    }
    count
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut count: u64 = 0;
    let mut pos: i32 = 50;
    for line in input.lines() {
        let Ok((_, (sign, amount))) = parse_line(line) else {
            panic!("could not parse '{line}'")
        };
        let delta = sign * amount;
        let pos_overflowed = pos + delta;
        let new_pos = (pos_overflowed).rem_euclid(100);
        println!("{pos} + {delta} => {new_pos} ({pos_overflowed})");
        let was_zero = pos == 0;
        pos = new_pos;
        let mut total = if pos == 0 && pos_overflowed >= 0 && pos_overflowed <= 100 {
            1
        } else if pos_overflowed < 0 {
            pos_overflowed.abs() / 100 + 1
        } else if pos_overflowed > 100 {
            pos_overflowed.abs() / 100
        } else {
            0
        };
        if total > 0 {
            if was_zero && (pos_overflowed < 0) {
                total -= 1;
            }
            println!("\trotated {total} times ONTO 0");
            count += total as u64;
        }
    }
    count
}
