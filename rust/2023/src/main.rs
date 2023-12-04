#![feature(test)]

mod day04;

fn main() {
    let input = aoc_utils::get_input(2023, 04);
    day04::solve_a(&input);
    day04::solve_b(&input);
}
