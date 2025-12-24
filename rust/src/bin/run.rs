#![feature(test)]

use aoc::utils;
use aoc::year2015;
use aoc::year2024;
use aoc::year2025;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <YEAR> <DAY>", args[0]);
        eprintln!("Example: {} 2024 1", args[0]);
        process::exit(1);
    }

    let year: i16 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Error: Year must be a valid number");
        process::exit(1);
    });

    let day: i8 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Error: Day must be a valid number");
        process::exit(1);
    });

    if !(1..=25).contains(&day) {
        eprintln!("Error: Day must be between 1 and 25");
        process::exit(1);
    }

    let input = utils::input::get(year, day);
    println!(" ----------------------------");
    println!(" |   AdventOfCode {}/{:0>2}   |", year, day);
    println!(" ----------------------------");

    match (year, day) {
        // 2015
        (2015, day) => run_year_2015(day, &input),
        // 2023
        (2023, day) => run_year_2023(day, &input),
        // 2024
        (2024, day) => run_year_2024(day, &input),
        // 2025
        (2025, day) => run_year_2025(day, &input),
        _ => {
            eprintln!("Error: Year {} is not implemented", year);
            process::exit(1);
        }
    }
}

fn run_year_2015(day: i8, input: &str) {
    match day {
        5 => year2015::day05::solve(input),
        _ => {
            eprintln!("Error: Year 2015 Day {} is not implemented", day);
            process::exit(1);
        }
    }
}

fn run_year_2023(day: i8, input: &str) {
    match day {
        _ => {
            eprintln!("Error: Year 2023 Day {} is not implemented", day);
            process::exit(1);
        }
    }
}

fn run_year_2024(day: i8, input: &str) {
    match day {
        1 => year2024::day01::solve(input),
        2 => year2024::day02::solve(input),
        3 => year2024::day03::solve(input),
        4 => year2024::day04::solve(input),
        5 => year2024::day05::solve(input),
        6 => year2024::day06::solve(input),
        7 => year2024::day07::solve(input),
        8 => year2024::day08::solve(input),
        9 => year2024::day09::solve(input),
        10 => year2024::day10::solve(input),
        11 => year2024::day11::solve(input),
        12 => year2024::day12::solve(input),
        13 => year2024::day13::solve(input),
        14 => year2024::day14::solve(input),
        // 15 => year2024::day15::solve(input), // Has compilation errors
        16 => year2024::day16::solve(input),
        _ => {
            eprintln!("Error: Year 2024 Day {} is not implemented", day);
            process::exit(1);
        }
    }
}

fn run_year_2025(day: i8, input: &str) {
    match day {
        1 => year2025::day01::solve(input),
        2 => year2025::day02::solve(input),
        3 => year2025::day03::solve(input),
        _ => {
            eprintln!("Error: Year 2025 Day {} is not implemented", day);
            process::exit(1);
        }
    }
}
