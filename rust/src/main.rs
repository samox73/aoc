#![feature(test)]

mod year2023;
mod utils;

fn main() {
    let year = 2023;
    let day = 25;
    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");
    year2023::day25::solve(&input);
}
