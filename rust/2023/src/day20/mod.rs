extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 20);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 20);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    println!("{}", input);
    let solution = do_something(input);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let solution = do_something_differently(input);
    println!("part b: {}", solution);
}

#[derive(Default)]
enum Pulse {
    #[default]
    Low,
    High,
}

trait Module {
    // fn tick(&self);
}

struct FlipFlop {
    state: Pulse,
}

struct Conjunction {
    state: HashMap<String, Pulse>
}

fn parse_module(input: &str) -> Module {

}

fn do_something(input: &str) -> u64 {
    for line in input.lines() {

    }
    return 0;
}

fn do_something_differently(input: &str) -> u64 {
    return 0;
}
