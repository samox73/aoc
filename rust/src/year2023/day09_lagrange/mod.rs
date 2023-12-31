use nom::{character::complete::multispace0, multi::many1, IResult};
use num_integer::binomial;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 09);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 09);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let series = parse_series(input).unwrap().1;
    let solution = sum_extrapolate_next(series);
    solution as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let series = parse_series(input).unwrap().1;
    let solution = sum_extrapolate_prev(series);
    solution as u64
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
        let mut next = 0;
        let n = s.len();
        for (i, y) in s.iter().enumerate() {
            // see derivation.rnote for a derivation on why we can write the legendre polynomials like this
            next += y * binomial(n, i) as i64 * (-1i64).pow((n - 1 - i) as u32);
        }
        sum += next;
    }
    return sum;
}

fn sum_extrapolate_prev(series: Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;
    for s in series {
        let mut next = 0;
        let n = s.len();
        for (i, y) in s.iter().enumerate() {
            next += y * binomial(n, i + 1) as i64 * (-1i64).pow(i as u32);
        }
        sum += next;
    }
    return sum;
}
