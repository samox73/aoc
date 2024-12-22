use std::{
    collections::HashMap,
    io::{self, Stdin},
};

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::i64, IResult};
use num_traits::Euclid;
use regex::Regex;

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 14);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 14);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2<i64>,
    vel: Vec2<i64>,
}

fn parse_robot(line: &str) -> IResult<&str, Robot> {
    let (line, _) = tag("p=")(line)?;
    let (line, px) = i64(line)?;
    let (line, _) = tag(",")(line)?;
    let (line, py) = i64(line)?;
    let (line, _) = tag(" v=")(line)?;
    let (line, vx) = i64(line)?;
    let (line, _) = tag(",")(line)?;
    let (line, vy) = i64(line)?;
    Ok((
        line,
        Robot {
            pos: (px, py).into(),
            vel: (vx, vy).into(),
        },
    ))
}

fn parse_robots(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        if let Ok((_, robot)) = parse_robot(line) {
            robots.push(robot);
        } else {
            println!("could not parse line");
        }
    }
    robots
}

fn print_bots(botmap: &HashMap<Vec2<i64>, usize>, xmax: i64, ymax: i64) {
    for y in 0..ymax {
        for x in 0..xmax {
            if let Some(count) = botmap.get(&(x, y).into()) {
                print!("{}", count.to_string());
            } else {
                print!("0");
            }
        }
        println!();
    }
}

fn safety_score(robots: &Vec<Robot>, xmax: i64, ymax: i64) -> u64 {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for b in robots.iter().map(|b| b.pos) {
        let left = b.x < xmax / 2;
        let top = b.y < ymax / 2;
        let right = b.x >= (xmax + 1) / 2;
        let bot = b.y >= (ymax + 1) / 2;
        match (left, top, right, bot) {
            (true, true, false, false) => q1 += 1,
            (false, true, true, false) => q2 += 1,
            (true, false, false, true) => q3 += 1,
            (false, false, true, true) => q4 += 1,
            _ => (),
        }
    }
    q1 * q2 * q3 * q4
}

fn solve_a(input: &str) -> u64 {
    println!("{}", input);
    let mut robots = parse_robots(input);
    let xmax = 101;
    let ymax = 103;
    let mut botmap: HashMap<Vec2<i64>, usize> = HashMap::new();
    for robot in robots.iter_mut() {
        robot.pos += robot.vel * 100;
        robot.pos.x = robot.pos.x.rem_euclid(xmax);
        robot.pos.y = robot.pos.y.rem_euclid(ymax);
        *botmap.entry(robot.pos).or_insert(0) += 1;
    }
    print_bots(&botmap, xmax, ymax);
    safety_score(&robots, xmax, ymax)
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    // the tree should be when most robots gather in one place, which should minimize the
    // safety score. another alternative would be to calculate the variance
    let mut robots = parse_robots(input);
    let xmax = 101;
    let ymax = 103;
    let mut scores = Vec::new();
    for _ in 0..10_000 {
        let s = safety_score(&robots, xmax, ymax);
        scores.push(s);
        let mut botmap: HashMap<Vec2<i64>, usize> = HashMap::new();
        for robot in robots.iter_mut() {
            robot.pos += robot.vel;
            robot.pos.x = robot.pos.x.rem_euclid(xmax);
            robot.pos.y = robot.pos.y.rem_euclid(ymax);
            *botmap.entry(robot.pos).or_insert(0) += 1;
        }
    }
    scores.iter().position_min().unwrap() as u64
}
