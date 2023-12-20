#![feature(test)]

mod day18;

fn main() {
    let input = aoc_utils::get_input(2023, 18);
    day18::solve_a(&input);
    day18::solve_b(&input);
}
