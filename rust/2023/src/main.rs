#![feature(test)]

mod day09_lagrange;

fn main() {
    let input = aoc_utils::get_input(2023, 09);
    day09_lagrange::solve_a(&input);
    day09_lagrange::solve_b(&input);
}
