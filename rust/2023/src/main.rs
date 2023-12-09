#![feature(test)]

mod day09_legendre;

fn main() {
    let input = aoc_utils::get_input(2023, 09);
    day09_legendre::solve_a(&input);
    day09_legendre::solve_b(&input);
}
