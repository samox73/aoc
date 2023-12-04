#![feature(test)]

mod day04_optimized;

fn main() {
    let input = aoc_utils::get_input(2023, 04);
    day04_optimized::solve_a(&input);
    day04_optimized::solve_b(&input);
}
