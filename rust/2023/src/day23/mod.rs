use std::fmt::Display;

use aocutils::{
    grid::{Grid, Vertexable},
    vec2::Vec2,
};
use itertools::Itertools;
use pathfinding::directed::{dfs::dfs_reach, dijkstra::dijkstra_all};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct State {
    coords: Vec2<isize>,
    visited: Vec<Vec2<isize>>,
}

impl State {
    fn from(coords: Vec2<isize>, visited: Vec<Vec2<isize>>) -> State {
        State { coords, visited }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[coords: {}, visited: {:?}]", self.coords, self.visited)
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

fn get_start(input: &str) -> State {
    let x = input
        .lines()
        .next()
        .unwrap()
        .split_once(".")
        .unwrap()
        .0
        .len();
    let current: Vec2<isize> = Vec2::from((x as isize, 0));
    State {
        coords: current,
        visited: Vec::new(),
    }
}

pub fn solve_a(input: &str) -> u64 {
    let grid = get_grid(input);
    let start = get_start(input);
    let s = dfs_reach(start, |state| {
        let mut next: Vec<State> = Vec::new();
        for step in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let proposed = state.coords + Vec2::from(step);
            let mut visited = state.visited.clone();
            visited.push(state.coords);
            if !grid.is_inside(&proposed.raw()) || visited.contains(&proposed) {
                continue;
            }
            if let Some(v) = grid.get(&proposed.raw()) {
                match (v.get_value(), step) {
                    ('.', _) => next.push(State::from(proposed, visited)),
                    ('>', (1, 0)) => next.push(State::from(proposed, visited)),
                    ('v', (0, 1)) => next.push(State::from(proposed, visited)),
                    ('<', (-1, 0)) => next.push(State::from(proposed, visited)),
                    ('^', (0, -1)) => next.push(State::from(proposed, visited)),
                    _ => continue,
                }
            }
        }
        next
    })
    .collect_vec();
    let longest: State = s
        .into_iter()
        .filter(|s| s.coords.y == grid.height - 1)
        .max_by(|a, b| a.visited.len().cmp(&b.visited.len()))
        .unwrap();
    println!("dfs done");
    println!("{longest}");

    let solution = longest.visited.len() as u64;
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
