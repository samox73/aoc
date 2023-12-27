#![feature(test)]

mod day24;

fn main() {
    let input = aocutils::get_input(2023, 24);
    let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    day24::solve_a(&input);
    day24::solve_b(&input);
}
