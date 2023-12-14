#![feature(test)]

mod day11;
mod utils;

fn main() {
    let input = aoc_utils::get_input(2023, 11);
    day11::solve_a(&input);
    day11::solve_b(&input);
}
