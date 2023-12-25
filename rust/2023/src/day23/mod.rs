extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 23);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 23);
    b.iter(|| solve_b(&input));
}

pub fn solve_a(input: &str) -> u64{
    println!("{}", input);
    let solution = 0;
    println!("part a: {}", solution);
    solution
}

pub fn solve_b(input: &str) -> u64 {
    let solution = 0;
    println!("part b: {}", solution);
    solution
}