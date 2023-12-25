#![feature(test)]

mod day23;

fn main() {
    let input = aoc_utils::get_input(2023, 23);
    day23::solve_a(&input);
    day23::solve_b(&input);
}
