use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 10);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 10);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn find_trailheads(m: &Vec<Vec<u32>>, x: i64, y: i64, xmax: i64, ymax: i64) -> HashSet<Vec2<i64>> {
    if x < 0 || y < 0 || x >= xmax || y >= ymax {
        return HashSet::new();
    }
    let mut set = HashSet::new();
    let yu = y as usize;
    let xu = x as usize;
    let value = m[yu][xu];
    if value == 9 {
        set.insert(Vec2::from((x, y)));
        return set;
    }

    if yu > 0 && m[yu - 1][xu] == value + 1 {
        set.extend(find_trailheads(m, x, y - 1, xmax, ymax));
    }
    if xu > 0 && m[yu][xu - 1] == value + 1 {
        set.extend(find_trailheads(m, x - 1, y, xmax, ymax));
    }
    if y < ymax - 1 && m[yu + 1][xu] == value + 1 {
        set.extend(find_trailheads(m, x, y + 1, xmax, ymax));
    }
    if x < ymax - 1 && m[yu][xu + 1] == value + 1 {
        set.extend(find_trailheads(m, x + 1, y, xmax, ymax));
    }

    set
}

fn find_distinct_trails(m: &Vec<Vec<u32>>, x: i64, y: i64, xmax: i64, ymax: i64) -> u64 {
    if x < 0 || y < 0 || x >= xmax || y >= ymax {
        return 0;
    }
    let yu = y as usize;
    let xu = x as usize;
    let value = m[yu][xu];
    if value == 9 {
        return 1;
    }

    let mut count = 0;
    if yu > 0 && m[yu - 1][xu] == value + 1 {
        count += find_distinct_trails(m, x, y - 1, xmax, ymax);
    }
    if xu > 0 && m[yu][xu - 1] == value + 1 {
        count += find_distinct_trails(m, x - 1, y, xmax, ymax);
    }
    if y < ymax - 1 && m[yu + 1][xu] == value + 1 {
        count += find_distinct_trails(m, x, y + 1, xmax, ymax);
    }
    if x < ymax - 1 && m[yu][xu + 1] == value + 1 {
        count += find_distinct_trails(m, x + 1, y, xmax, ymax);
    }

    count
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let xmax = map[0].len() as i64;
    let ymax = map.len() as i64;
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 0 {
                solution += find_trailheads(&map, x as i64, y as i64, xmax, ymax).len() as u64;
            }
        }
    }
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let xmax = map[0].len() as i64;
    let ymax = map.len() as i64;
    for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 0 {
                solution += find_distinct_trails(&map, x as i64, y as i64, xmax, ymax);
            }
        }
    }
    solution
}
