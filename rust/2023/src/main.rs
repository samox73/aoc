#![feature(test)]

mod day13;

fn main() {
    let input = aoc_utils::get_input(2023, 13);
    day13::solve_a(&input);
    day13::solve_b(&input);
}
