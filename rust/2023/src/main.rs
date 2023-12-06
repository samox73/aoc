#![feature(test)]

mod day06;

fn main() {
    let input = aoc_utils::get_input(2023, 06);
    day06::solve_a(&input);
    day06::solve_b(&input);
}
