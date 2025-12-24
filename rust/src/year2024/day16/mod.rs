extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 16);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 16);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let solution = 0;
    println!("{}", input);
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let solution = 0;
    solution
}
