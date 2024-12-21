use regex::Regex;

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 13);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 13);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

#[derive(Clone, Debug)]
struct Matrix {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

#[derive(Clone, Debug)]
struct System {
    matrix: Matrix,
    solution: Vec2<i64>,
}

impl Matrix {
    fn det(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }

    fn cramers(&self, p: Vec2<i64>) -> Option<Vec2<i64>> {
        let mut m1 = self.clone();
        m1.a = p.x;
        m1.c = p.y;
        let d = self.det();
        if d == 0 {
            return None;
        }
        let c1 = m1.det() as f64 / d as f64;
        let mut m2 = self.clone();
        m2.b = p.x;
        m2.d = p.y;
        let c2 = m2.det() as f64 / d as f64;
        if c1.floor() != c1 || c2.floor() != c2 {
            return None;
        }
        Some((c1 as i64, c2 as i64).into())
    }
}

fn parse_vec2(a: &str, b: &str) -> Vec2<i64> {
    let a1 = a.parse::<i64>().unwrap();
    let a2 = b.parse::<i64>().unwrap();
    (a1, a2).into()
}

fn parse(input: &str, c: i64) -> Vec<System> {
    let mut systems = Vec::new();
    let re_a = Regex::new(r"Button A: X\+(?<a1>\d*), Y\+(?<a2>\d*)").unwrap();
    let re_b = Regex::new(r"Button B: X\+(?<b1>\d*), Y\+(?<b2>\d*)").unwrap();
    let re_s = Regex::new(r"Prize: X=(?<p1>\d*), Y=(?<p2>\d*)").unwrap();
    for sys in input.split("\n\n") {
        let mut a: Vec2<i64> = (0, 0).into();
        let mut b: Vec2<i64> = (0, 0).into();
        let mut s: Vec2<i64> = (0, 0).into();
        if let Some(cap) = re_a.captures_iter(sys).next() {
            a = parse_vec2(&cap["a1"], &cap["a2"]);
        };
        if let Some(cap) = re_b.captures_iter(sys).next() {
            b = parse_vec2(&cap["b1"], &cap["b2"]);
        };
        if let Some(cap) = re_s.captures_iter(sys).next() {
            s = parse_vec2(&cap["p1"], &cap["p2"]);
            s.x += c;
            s.y += c;
        };
        systems.push(System {
            matrix: Matrix {
                a: a.x,
                b: b.x,
                c: a.y,
                d: b.y,
            },
            solution: s,
        });
    }
    systems
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let systems = parse(input, 0);
    for s in systems {
        if let Some(sol) = s.matrix.cramers(s.solution) {
            if sol.x <= 100 && sol.y <= 100 && sol.x >= 0 && sol.y >= 0 {
                solution += 3 * sol.x;
                solution += sol.y;
            }
        }
    }
    solution as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let systems = parse(input, 10000000000000);
    for s in systems {
        if let Some(sol) = s.matrix.cramers(s.solution) {
            if sol.x >= 0 && sol.y >= 0 {
                solution += 3 * sol.x;
                solution += sol.y;
            }
        }
    }
    solution as u64
}
