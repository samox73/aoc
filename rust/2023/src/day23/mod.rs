use std::fmt::Display;

use aocutils::{
    grid::{Grid, Vertexable},
    vec2::Vec2,
};
use pathfinding::directed::dfs::dfs_reach;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 23);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 23);
    b.iter(|| solve_b(&input));
}

struct Vertex {
    value: char,
}

impl Vertexable for Vertex {
    fn get_value(&self) -> char {
        self.value
    }
}

fn get_grid(input: &str) -> Grid {
    let lines: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut grid: Grid = Grid::new(lines[0].len() as isize, lines.len() as isize);
    for (y, line) in lines.iter().enumerate() {
        for (x, &token) in line.iter().enumerate() {
            if token == '.' || token == '>' || token == 'v' || token == '<' || token == '^' {
                grid.add_vertex((x as isize, y as isize), Box::new(Vertex { value: token }));
            }
        }
    }
    grid
}

fn get_start(input: &str) -> Vec2<isize> {
    let x = input
        .lines()
        .next()
        .unwrap()
        .split_once(".")
        .unwrap()
        .0
        .len();
    Vec2::from((x as isize, 0))
}

pub fn dfs<N, FN, IN>(start: N, successors: FN) -> Vec<Vec2<isize>>
where
    FN: FnMut(N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let v = Vec::new();
    v
}

pub fn solve_a(input: &str) -> u64 {
    let grid = get_grid(input);
    let start = get_start(input);
    let s = dfs(start, |state| {
        let mut next: Vec<Vec2<isize>> = Vec::new();
        let steps: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for step in steps {
            let proposed = state + Vec2::from(step);
            if !grid.is_inside(&proposed.raw()) {
                continue;
            }
            if let Some(v) = grid.get(&proposed.raw()) {
                match (v.get_value(), step) {
                    ('.', _) => next.push(proposed),
                    ('>', (1, 0)) => next.push(proposed),
                    ('v', (0, 1)) => next.push(proposed),
                    ('<', (-1, 0)) => next.push(proposed),
                    ('^', (0, -1)) => next.push(proposed),
                    _ => continue,
                }
            }
        }
        next
    });
    println!("done");
    let longest = s
        .into_iter()
        .filter(|s| s.coords.y == grid.height - 1)
        .next()
        // .max_by(|a, b| a.visited.len().cmp(&b.visited.len()))
        .unwrap();
    println!("{longest}");

    // for y in 0..grid.height {
    //     for x in 0..grid.width {
    //         if let Some(v) = grid.get(&(x, y)) {
    //             if longest.visited.contains(&Vec2::from((x, y))) {
    //                 print!("O");
    //             } else {
    //                 print!("{}", v.get_value());
    //             }
    //         } else {
    //             print!("#",);
    //         }
    //     }
    //     println!();
    // }

    let solution = 0;
    println!("part a: {}", solution);
    solution
}

pub fn solve_b(input: &str) -> u64 {
    let input = input
        .replace("v", ".")
        .replace("^", ".")
        .replace("<", ".")
        .replace(">", ".");
    let solution = solve_a(input.as_str());
    println!("part b: {}", solution);
    solution
}
