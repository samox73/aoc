use nom::IResult;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        if let Ok((_, (l, w, h))) = parse_line(line) {
            solution += 2 * (l * w + w * h + h * l);
            let (v1, v2) = smallest_two(l, w, h);
            solution += v1 * v2;
        }
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        if let Ok((_, (l, w, h))) = parse_line(line) {
            let (v1, v2) = smallest_two(l, w, h);
            solution += 2 * (v1 + v2);
            solution += l * w * h;
        }
    }
    solution
}

fn smallest_two(l: u64, w: u64, h: u64) -> (u64, u64) {
    let mut v = vec![l, w, h];
    v.sort();
    (v[0], v[1])
}

fn parse_line(input: &str) -> IResult<&str, (u64, u64, u64)> {
    let (input, l) = nom::character::complete::u64(input)?;
    let (input, _) = nom::bytes::complete::take(1u32)(input)?;
    let (input, w) = nom::character::complete::u64(input)?;
    let (input, _) = nom::bytes::complete::take(1u32)(input)?;
    let (input, h) = nom::character::complete::u64(input)?;
    return Ok((input, (l, w, h)));
}
