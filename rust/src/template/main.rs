#![feature(test)]

mod yearYYYY;
mod utils;

fn main() {
    let year = YYYY;
    let day = DD;
    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");
    yearYYYY::dayDD::solve(&input);
}
