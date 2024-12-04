extern crate test;

use regex::Regex;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 03);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 03);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let re = Regex::new(r"mul\((?<left>\d*),(?<right>\d*)\)").unwrap();
    for cap in re.captures_iter(input) {
        let left = &cap["left"];
        let right = &cap["right"];
        solution += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    for part in input.split("do()") {
        let muls = part.split("don't()").next().unwrap();
        let re = Regex::new(r"mul\((?<left>\d*),(?<right>\d*)\)").unwrap();
        for cap in re.captures_iter(muls) {
            let left = &cap["left"];
            let right = &cap["right"];
            solution += left.parse::<u64>().unwrap() * right.parse::<u64>().unwrap();
        }
    }
    solution
}
