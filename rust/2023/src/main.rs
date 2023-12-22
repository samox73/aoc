#![feature(test)]

mod day20;

fn main() {
    let input = aoc_utils::get_input(2023, 20);
    //     let input = "broadcaster -> a
    // %a -> inv, con
    // &inv -> b
    // %b -> con
    // &con -> output";
    day20::solve_a(&input);
    day20::solve_b(&input);
}
