#![feature(test)]

mod day08;

fn main() {
    let input = aoc_utils::get_input(2023, 08);
    // let input = "LR

    // AAA = (AAB, XXX)
    // AAB = (XXX, AAZ)
    // AAZ = (AAB, XXX)
    // BBA = (BBB, XXX)
    // BBB = (BBC, BBC)
    // BBC = (BBZ, BBZ)
    // BBZ = (BBB, BBB)
    // XXX = (XXX, XXX)";
    day08::solve_a(&input);
    day08::solve_b(&input);
}
