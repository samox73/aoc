#![feature(test)]

mod dayXX;

fn main() {
    let input = aocutils::get_input(2023, XX);
    dayXX::solve_a(&input);
    dayXX::solve_b(&input);
}
