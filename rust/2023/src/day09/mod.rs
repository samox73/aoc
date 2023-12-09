use itertools::Itertools;
use nom::{character::complete::multispace0, multi::many1, IResult};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 09);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 09);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let series = parse_series(input).unwrap().1;
    let solution = sum_extrapolate_next(series);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let series = parse_series(input).unwrap().1;
    let solution = sum_extrapolate_prev(series);
    println!("part b: {}", solution);
}

fn parse_series(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let mut series: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let (_, vec) = many1(|s| -> IResult<&str, i64> {
            let (s, d) = nom::character::complete::i64(s)?;
            let (s, _) = multispace0(s)?;
            Ok((s, d))
        })(line)?;
        series.push(vec);
    }
    Ok((input, series))
}

fn sum_extrapolate_next(series: Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for s in series {
        let diffs = build_diffs(&s);
        let next = get_next_value(&diffs);
        sum += next;
    }
    return sum;
}

fn sum_extrapolate_prev(series: Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for s in series {
        let diffs = build_diffs(&s);
        let next = get_previous_value(&diffs);
        sum += next;
    }
    return sum;
}

fn get_next_value(diffs: &Vec<Vec<i64>>) -> i64 {
    let mut val = 0;
    for i in (0..diffs.len() - 1).rev() {
        let series = &diffs[i];
        let next = series[series.len() - 1];
        val += next;
    }
    return val;
}

fn get_previous_value(diffs: &Vec<Vec<i64>>) -> i64 {
    let mut val = 0;
    for i in (0..diffs.len() - 1).rev() {
        let series = &diffs[i];
        let next = series[0];
        val = next - val;
    }
    return val;
}

fn build_diffs(series: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut current = series.clone();
    let mut diffs: Vec<Vec<i64>> = Vec::new();
    diffs.push(current.clone());
    while current.iter().any(|v| *v != 0) {
        let mut next_diff: Vec<i64> = Vec::new();
        for i in 1..current.len() {
            next_diff.push(current[i] - current[i - 1]);
        }
        current = next_diff.clone();
        diffs.push(next_diff);
    }
    diffs
}

fn do_something_differently(input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::day09::get_previous_value;

    use super::{build_diffs, get_next_value};

    #[test]
    fn extrapolation_is_correct() {
        let series = vec![10, 13, 16, 21, 30, 45];
        let diffs = build_diffs(&series);
        let next = get_next_value(&diffs);
        assert_eq!(next, 68);
        let prev = get_previous_value(&diffs);
        assert_eq!(prev, 5);
    }
}
