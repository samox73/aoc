use nom::{bytes::complete::tag, character::complete::u64, multi::separated_list0, IResult};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 07);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 07);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn parse_result(line: &str) -> IResult<&str, (u64, Vec<u64>)> {
    let (line, result) = u64(line)?;
    let (line, _) = tag(": ")(line)?;
    let (line, operands) = separated_list0(tag(" "), u64)(line)?;
    Ok((line, (result, operands)))
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn branch(
    target: u64,
    mut current: u64,
    mut operands: Vec<u64>,
    op: Operator,
    with_concat: bool,
) -> bool {
    let next = operands.pop().unwrap();
    if current > target {
        return false;
    }
    match op {
        Operator::Add => {
            let r = current.overflowing_add(next);
            if r.1 {
                return false;
            }
            current = r.0
        }
        Operator::Mul => {
            let r = current.overflowing_mul(next);
            if r.1 {
                return false;
            }
            current = r.0
        }
        Operator::Concat => {
            if !with_concat {
                return false;
            }
            let s = current.to_string() + next.to_string().as_str();
            if let Ok(i) = s.parse::<u64>() {
                current = i;
            } else {
                return false;
            }
        }
    }
    if operands.is_empty() {
        if target == current {
            return true;
        }
        return false;
    }
    let b1 = branch(
        target,
        current,
        operands.clone(),
        Operator::Add,
        with_concat,
    );
    let b2 = branch(
        target,
        current,
        operands.clone(),
        Operator::Mul,
        with_concat,
    );
    let b3 = branch(
        target,
        current,
        operands.clone(),
        Operator::Concat,
        with_concat,
    );
    b1 || b2 || b3
}

fn solve_equation(line: &str, with_concat: bool) -> u64 {
    if let Ok((_, (result, mut operands))) = parse_result(line) {
        operands.reverse();
        let current = operands.pop().unwrap();
        let op1 = operands.clone();
        let mut b1 = branch(result, current, op1, Operator::Add, with_concat);
        let op2 = operands.clone();
        b1 = b1 || branch(result, current, op2, Operator::Mul, with_concat);
        if with_concat {
            let op3 = operands.clone();
            b1 = b1 || branch(result, current, op3, Operator::Concat, with_concat);
        }
        if b1 {
            return result;
        }
    }
    0
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        solution += solve_equation(line, false);
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    for line in input.lines() {
        solution += solve_equation(line, true);
    }
    solution
}
