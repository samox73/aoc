use nom::{
    bytes::complete::take_till,
    character::{complete::multispace0, is_digit},
    multi::many1,
    IResult,
};

use itertools::izip;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 06);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 06);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let solution = do_something(input);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let solution = do_something_differently(input);
    println!("part b: {}", solution);
}

fn get_count(time: u64, distance: u64) -> u64 {
    let t_mid: f64 = (time as f64 - 1.0) / 2.0;
    let r = t_mid.powi(2) + t_mid + 0.25 - distance as f64;
    let x1 = t_mid + 0.5 - r.sqrt();
    let x2 = t_mid + 0.5 + r.sqrt();
    (x2.ceil() - x1.floor() - 1.0) as u64
}

fn do_something(input: &str) -> u64 {
    let (input, times) = parse_line(input).unwrap();
    let (_, distances) = parse_line(input).unwrap();
    let mut mult = 1u64;
    for (time, distance) in izip!(times, distances) {
        mult *= get_count(time, distance);
    }
    return mult;
}

fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
    return parse_vector(input);
}

fn parse_vector(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, seeds) = many1(|s| -> IResult<&str, u64> {
        let (s, _) = multispace0(s)?;
        let (s, number) = nom::character::complete::u64(s)?;
        return Ok((s, number));
    })(input)?;
    return Ok((input, seeds));
}

fn do_something_differently(input: &str) -> u64 {
    let (input, time) = parse_line_ignore_spaces(input).unwrap();
    let (_, distance) = parse_line_ignore_spaces(input).unwrap();
    return get_count(time, distance);
}

fn parse_line_ignore_spaces(input: &str) -> IResult<&str, u64> {
    let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
    let (input, number) = take_till(|c| c == '\n')(input)?;
    let b = number.replace(" ", "");
    let number = b.parse::<u64>().unwrap();
    return Ok((input, number));
}
