#![feature(test)]

mod year2024;
mod utils;

fn main() {
    let year = 2024;
    let day = 08;
    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");
    year2024::day08::solve(&input);
}
