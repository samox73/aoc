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

fn solve_a(input: &str) -> i64 {
    let mut solution = 0;
    for line in input.lines() {
        for char in line.chars() {
            match char {
                '(' => solution += 1,
                ')' => solution -= 1,
                _ => unreachable!(),
            }
        }
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let mut pos = 0;
    for line in input.lines() {
        for char in line.chars() {
            pos += 1;
            match char {
                '(' => solution += 1,
                ')' => solution -= 1,
                _ => unreachable!(),
            }
            if solution == -1 {
                return pos;
            }
        }
    }
    0
}
