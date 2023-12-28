#![feature(test)]

mod day25;

fn main() {
    let input = aocutils::get_input(2023, 25);
    day25::solve_a(&input);
    day25::solve_b(&input);
}
