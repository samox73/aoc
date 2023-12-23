#![feature(test)]

mod day21;

fn main() {
    let input = aoc_utils::get_input(2023, 21);
    day21::solve_a(&input);
    day21::solve_b(&input);
}
