#![feature(test)]

mod day05;

fn main() {
    let input = aoc_utils::get_input(2023, 05);
    day05::solve_a(&input);
    day05::solve_b(&input);
}
