#![feature(test)]

mod day19;

fn main() {
    let input = aoc_utils::get_input(2023, 19);
    day19::solve_a(&input);
    day19::solve_b(&input);
}
