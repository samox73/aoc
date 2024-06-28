use std::collections::HashMap;

use crate::utils::vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let mut locations = HashMap::new();
    let mut current_loc = vec2::Vec2::from((0, 0));
    for c in input.chars() {
        let direction = char_to_direction(c);
        current_loc += direction;
        if locations.contains_key(&current_loc) {
            *locations.get_mut(&current_loc).unwrap() += 1;
        } else {
            locations.insert(current_loc, 1);
        }
    }
    locations.len() as u64
}

fn solve_b(input: &str) -> u64 {
    let mut locations = HashMap::new();
    let mut santa_loc = vec2::Vec2::from((0, 0));
    let mut robot_loc = vec2::Vec2::from((0, 0));
    let chars: Vec<char> = input.chars().collect();
    for c in chars.chunks(2) {
        match c {
            [dir_santa, dir_robot] => {
                santa_loc += char_to_direction(*dir_santa);
                robot_loc += char_to_direction(*dir_robot);
                if locations.contains_key(&santa_loc) {
                    *locations.get_mut(&santa_loc).unwrap() += 1;
                } else {
                    locations.insert(santa_loc, 1);
                }
                if locations.contains_key(&robot_loc) {
                    *locations.get_mut(&robot_loc).unwrap() += 1;
                } else {
                    locations.insert(robot_loc, 1);
                }
            }
            _ => unreachable!(),
        }
    }
    locations.len() as u64
}

fn char_to_direction(c: char) -> vec2::Vec2<i32> {
    return match c {
        '<' => vec2::Vec2::from((-1, 0)),
        '>' => vec2::Vec2::from((1, 0)),
        'v' => vec2::Vec2::from((0, -1)),
        '^' => vec2::Vec2::from((0, 1)),
        _ => unreachable!("Encountered unknown char"),
    };
}
