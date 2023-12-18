#![feature(test)]

mod day15;

fn main() {
    let input = aoc_utils::get_input(2023, 15);
    day15::solve_a(&input);
    day15::solve_b(&input);
}
