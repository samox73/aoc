#![feature(test)]
#![feature(linked_list_cursors)]

mod utils;
mod year2024;

fn main() {
    let year = 2024;
    let day = 11;
    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");
    year2024::day11::solve(&input);
}
