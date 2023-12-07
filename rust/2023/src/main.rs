#![feature(test)]

mod day07;

fn main() {
    let input = aoc_utils::get_input(2023, 07);
    day07::solve_a(&input);
    day07::solve_b(&input);
}
