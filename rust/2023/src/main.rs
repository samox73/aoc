#![feature(test)]

mod day09;

fn main() {
    let input = aoc_utils::get_input(2023, 09);
    day09::solve_a(&input);
    day09::solve_b(&input);
}
