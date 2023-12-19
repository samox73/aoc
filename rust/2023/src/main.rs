#![feature(test)]

mod day16;

fn main() {
    let input = aoc_utils::get_input(2023, 16);
    //     let input  = ".|...\\....
    // |.-.\\.....
    // .....|-...
    // ........|.
    // ..........
    // .........\\
    // ..../.\\\\..
    // .-.-/..|..
    // .|....-|.\\
    // ..//.|....";
    day16::solve_a(&input);
    day16::solve_b(&input);
}
