#![feature(test)]

mod year2015;
mod utils;

fn main() {
    let year = 2015;
    let day = 01;
    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");
    year2015::day01::solve(&input);
}
