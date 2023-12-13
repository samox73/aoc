#![feature(test)]

mod day10;

fn main() {
    let input = aoc_utils::get_input(2023, 10);
    day10::solve_a(&input);
    day10::solve_b(&input);
}
