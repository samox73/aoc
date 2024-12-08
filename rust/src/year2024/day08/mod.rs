use std::collections::{HashMap, HashSet};

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 08);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 08);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

type Antennas = HashMap<char, Vec<Vec2<i64>>>;

fn parse_antennas(input: &str) -> (Antennas, i64, i64) {
    let w = input.lines().next().unwrap().len();
    let h = input.lines().count();
    let mut a = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as i64, y as i64);
            if c != '.' {
                a.entry(c).or_insert(Vec::new()).push(pos.into());
            }
        }
    }
    (a, w as i64, h as i64)
}

fn find_antinodes(a: Antennas, w: i64, h: i64, harmonics: bool) -> HashSet<Vec2<i64>> {
    let mut s = HashSet::new();
    for (_, antennas) in a.into_iter() {
        for a1 in &antennas {
            for a2 in &antennas {
                if a1 == a2 {
                    continue;
                }
                let diff = *a2 - *a1;
                if harmonics {
                    let x_count = ((w as f64) / diff.x as f64) as u64;
                    let y_count = ((h as f64) / diff.y as f64) as u64;
                    for i in 0..x_count.max(y_count) {
                        s.insert(*a1 - diff * i as i64);
                        s.insert(*a1 + diff * i as i64);
                    }
                } else {
                    s.insert(*a1 - diff);
                    s.insert(*a2 + diff);
                }
            }
        }
    }
    s
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let (a, w, h) = parse_antennas(input);
    let anti_nodes = find_antinodes(a, w, h, false);
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    for an in anti_nodes {
        if an.x >= 0 && an.y >= 0 && an.x < w && an.y < h {
            solution += 1;
            lines[an.y as usize].replace_range(an.x as usize..=an.x as usize, "#");
        }
    }
    let input = lines.join("\n");
    println!("{}", input);
    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let (a, w, h) = parse_antennas(input);
    let anti_nodes = find_antinodes(a, w, h, true);
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    for an in anti_nodes {
        if an.x >= 0 && an.y >= 0 && an.x < w && an.y < h {
            solution += 1;
            lines[an.y as usize].replace_range(an.x as usize..=an.x as usize, "#");
        }
    }
    let input = lines.join("\n");
    println!("{}", input);
    solution
}
