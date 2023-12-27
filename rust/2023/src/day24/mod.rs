use std::collections::HashSet;

use aocutils::vec2::Vec2;
use itertools::Itertools;
use nalgebra::{Matrix3, Matrix6, Vector3, Vector6};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 24);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 24);
    b.iter(|| solve_b(&input));
}

struct Line {
    p1: Vec2<f64>,
    p2: Vec2<f64>,
    raw: String,
}

fn parse_line(l: &str) -> Line {
    let raw = l.to_string();
    let (p, v) = l.split_once(" @ ").unwrap();
    let p = p.splitn(3, ",").collect_vec();
    let px: f64 = p[0].trim().parse().unwrap();
    let py: f64 = p[1].trim().parse().unwrap();
    let pz: f64 = p[2].trim().parse().unwrap();
    let v = v.splitn(3, ",").collect_vec();
    let vx: f64 = v[0].trim().parse().unwrap();
    let vy: f64 = v[1].trim().parse().unwrap();
    let vz: f64 = v[2].trim().parse().unwrap();
    let a = vy / vx;
    let b = py - a * px;
    let p1 = Vec2::from((px, py));
    let p2 = Vec2::from((px + vx, py + vy));
    Line { p1, p2, raw }
}

struct Hailstone<T> {
    p: Vector3<T>,
    v: Vector3<T>,
}

fn parse_hailstone(l: &str) -> Hailstone<i64> {
    let (p, v) = l.split_once(" @ ").unwrap();
    let p = p.splitn(3, ",").collect_vec();
    let px: i64 = p[0].trim().parse().unwrap();
    let py: i64 = p[1].trim().parse().unwrap();
    let pz: i64 = p[2].trim().parse().unwrap();
    let v = v.splitn(3, ",").collect_vec();
    let vx: i64 = v[0].trim().parse().unwrap();
    let vy: i64 = v[1].trim().parse().unwrap();
    let vz: i64 = v[2].trim().parse().unwrap();
    let a = vy / vx;
    let b = py - a * px;
    let p = Vector3::new(px, py, pz);
    let v = Vector3::new(vx, vy, vz);
    Hailstone { p, v }
}

fn to_f64(h: &Hailstone<i64>) -> Hailstone<f64> {
    Hailstone {
        p: h.p.map(|f| f as f64),
        v: h.v.map(|f| f as f64),
    }
}

fn intersect(l1: &Line, l2: &Line) -> Option<(f64, f64)> {
    let x1 = l1.p1.x;
    let x2 = l1.p2.x;
    let y1 = l1.p1.y;
    let y2 = l1.p2.y;
    let x3 = l2.p1.x;
    let x4 = l2.p2.x;
    let y3 = l2.p1.y;
    let y4 = l2.p2.y;
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2))
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));
    Some((t, u))
}

fn is_inside(x: f64, y: f64, min: f64, max: f64) -> bool {
    x > min && x < max && y > min && y < max
}

fn get_intersection_point(line: &Line, t: f64, u: f64) -> Vec2<f64> {
    Vec2::from((
        line.p1.x + t * (line.p2.x - line.p1.x),
        line.p1.y + t * (line.p2.y - line.p1.y),
    ))
}

pub fn solve_a(input: &str) -> u64 {
    let lines: Vec<Line> = input.lines().map(parse_line).collect_vec();
    let mut solution = 0;
    for i in 0..lines.len() {
        for j in 0..i {
            let l1 = &lines[i];
            let l2 = &lines[j];
            if let Some((t, u)) = intersect(l1, l2) {
                let i = get_intersection_point(l1, t, u);
                if is_inside(i.x, i.y, 200000000000000f64, 400000000000000f64)
                    && t > 0f64
                    && u > 0f64
                {
                    solution += 1;
                }
            }
        }
    }

    println!("part a: {}", solution);
    solution
}

fn cross(v: &Vector3<f64>) -> Matrix3<f64> {
    Matrix3::new(0f64, -v[2], v[1], v[2], 0f64, -v[0], -v[1], v[0], 0f64)
}

pub fn solve_b(input: &str) -> u64 {
    let hailstones: Vec<Hailstone<f64>> = input
        .lines()
        .map(|l| to_f64(&parse_hailstone(l)))
        .collect_vec();
    let hailstonesi64: Vec<Hailstone<i64>> = input.lines().map(parse_hailstone).collect_vec();

    let c0v = cross(&hailstones[1].v);
    let c0p = cross(&hailstones[1].p);
    let c1v = cross(&hailstones[2].v);
    let c1p = cross(&hailstones[2].p);
    let c2v = cross(&hailstones[3].v);
    let c2p = cross(&hailstones[3].p);
    let mut m: Matrix6<f64> = Matrix6::default();
    for i in 0..3 {
        for j in 0..3 {
            m[i + j * 6] = c0v[i + j * 3] - c1v[i + j * 3];
            m[i + j * 6 + 3] = c0v[i + j * 3] - c2v[i + j * 3];
            m[i + j * 6 + 18] = -c0p[i + j * 3] + c1p[i + j * 3];
            m[i + j * 6 + 21] = -c0p[i + j * 3] + c2p[i + j * 3];
        }
    }
    println!("{m}");
    println!("{}", -c0p + c1p);
    let inv = m.try_inverse().unwrap();
    println!("{inv}");

    let mut rhs: Vector6<f64> = Vector6::default();
    let e02 = -hailstones[0].p.cross(&hailstones[0].v) + hailstones[1].p.cross(&hailstones[1].v);
    let e35 = -hailstones[0].p.cross(&hailstones[0].v) + hailstones[2].p.cross(&hailstones[2].v);
    for i in 0..3 {
        rhs[i] = e02[i];
        rhs[i + 3] = e35[i];
    }
    let result = inv * rhs;
    let result: Vector6<i64> = result.map(|f| f.round() as i64);
    println!("{result}");
    let mut solution = 0;
    for i in 0..3 {
        solution += result[i];
    }

    run_simulation(result, hailstonesi64);

    println!("part b: {}", solution);
    solution as u64
}

fn get_position_after(h: &Hailstone<i64>, t: i64) -> Vector3<i64> {
    h.p + t * h.v
}

fn get_rock_position_after(rock: &Vector6<i64>, t: i64) -> Vector3<i64> {
    Vector3::from(rock.fixed_rows::<3>(0)) + t * Vector3::from(rock.fixed_rows::<3>(3))
}

fn run_simulation(rock: Vector6<i64>, hailstones: Vec<Hailstone<i64>>) {
    let mut remaining = (0..hailstones.len() as i64).collect::<HashSet<i64>>();
    let mut t = 0i64;
    while remaining.len() != 0 {
        let r = get_rock_position_after(&rock, t as i64);
        println!("rock pos: {r}");
        println!("remaining: {}", remaining.len());
        for (i, h) in hailstones.iter().enumerate() {
            let h = get_position_after(h, t as i64);
            if h.eq(&r) {
                println!("rock hits hailstone #{i} at t={t}!");
                remaining.remove(&(i as i64));
            }
        }
        t += 1;
    }
}
