use num_traits::Num;

use crate::utils::{
    grid::{Grid, Vertexable},
    vec2::Vec2,
};
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 06);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 06);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

struct Vertex {
    value: char,
}

impl Vertexable for Vertex {
    fn get_value(&self) -> char {
        self.value
    }
    fn set_value(&mut self, c: char) {
        self.value = c;
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Walker {
    pos: Vec2<isize>,
    dir: Vec2<isize>,
}

impl Vec2<isize> {
    fn rotate_cw(&self) -> Vec2<isize> {
        (-self.y, self.x).into()
    }
    fn rotate_ccw(&self) -> Vec2<isize> {
        (self.y, -self.x).into()
    }
}

impl Walker {
    fn new(pos: (isize, isize)) -> Walker {
        Walker {
            pos: pos.into(),
            dir: (0, -1).into(),
        }
    }

    fn next_pos(&self) -> Vec2<isize> {
        self.pos.add(self.dir)
    }

    fn rotate_cw(&mut self) {
        self.dir = self.dir.rotate_cw();
    }
}

struct WalkerGrid {
    pub grid: Grid,
    pub visited: HashSet<Vec2<isize>>,
    pub obstruction_hits: HashMap<Vec2<isize>, HashSet<Vec2<isize>>>,
    pub obstacle_locations: HashSet<Vec2<isize>>,
}

impl WalkerGrid {
    fn new(width: isize, height: isize, walker_pos: Option<(isize, isize)>) -> WalkerGrid {
        let grid = Grid::new(width, height);
        let p = if let Some(w) = walker_pos { w } else { (0, 0) };
        let wg = WalkerGrid {
            grid,
            visited: HashSet::new(),
            obstruction_hits: HashMap::new(),
            obstacle_locations: HashSet::new(),
        };
        wg
    }

    fn walk_a(&mut self, mut walker: Walker) {
        self.visited.insert(walker.pos);
        let mut next_pos = walker.next_pos();
        while self.grid.is_inside(&next_pos.raw()) {
            let c = self.grid.get_value(&next_pos.raw()).unwrap();
            if c == '#' {
                walker.rotate_cw();
            } else {
                walker.pos = next_pos;
            }
            next_pos = walker.next_pos();
            self.visited.insert(walker.pos);
        }
    }

    fn walk_loops(
        &mut self,
        mut obstacles: HashMap<Vec2<isize>, HashSet<Vec2<isize>>>,
        mut walker: Walker,
        mut visited: HashSet<Vec2<isize>>,
        loop_sim: bool,
    ) -> bool {
        visited.insert(walker.pos);
        let mut next_pos = walker.next_pos();
        while self.grid.is_inside(&next_pos.raw()) {
            let c = self.grid.get_value(&next_pos.raw()).unwrap();
            if c == '#' || c == '@' {
                walker.rotate_cw();
                let directions = obstacles.entry(next_pos).or_insert(HashSet::new());
                if loop_sim && directions.contains(&walker.dir) {
                    return true;
                }
                directions.insert(walker.dir);
            } else {
                if !loop_sim && !visited.contains(&next_pos) {
                    self.grid.get_mut(&next_pos.raw()).unwrap().set_value('#');
                    if self.walk_loops(obstacles.clone(), walker.clone(), visited.clone(), true) {
                        self.obstacle_locations.insert(next_pos);
                    }
                    self.grid.get_mut(&next_pos.raw()).unwrap().set_value('.');
                }
                walker.pos = next_pos;
            }
            next_pos = walker.next_pos();
            visited.insert(walker.pos);
        }
        false
    }

    fn visualize(&self, walker: Walker, visited: HashSet<Vec2<isize>>) {
        for _ in 0..self.grid.width + 2 {
            print!("_");
        }
        println!();
        for y in 0..self.grid.height {
            print!("|");
            for x in 0..self.grid.width {
                let v: Vec2<isize> = (x, y).into();
                if walker.pos == v {
                    print!("W");
                } else if visited.contains(&v) {
                    print!("X");
                } else {
                    print!("{}", self.grid.get_value(&v.raw()).unwrap());
                }
            }
            println!("|");
        }
        for _ in 0..self.grid.width + 2 {
            print!("‾");
        }
        println!();
    }
}

fn parse_grid(input: &str) -> (WalkerGrid, Walker) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut wg = WalkerGrid::new(width as isize, height as isize, None);
    let mut w = Walker::new((0, 0));
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = (x as isize, y as isize);
            if c == '.' {
                wg.grid.add_vertex(pos, Box::new(Vertex { value: ' ' }));
            } else {
                wg.grid.add_vertex(pos, Box::new(Vertex { value: c }));
            }
            if c == '^' {
                w.pos = pos.into();
            }
        }
    }
    (wg, w)
}

fn solve_a(input: &str) -> u64 {
    let (mut wg, w) = parse_grid(input);
    wg.walk_a(w);
    wg.visited.len() as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let (mut wg, w) = parse_grid(input);
    wg.walk_loops(HashMap::new(), w, HashSet::new(), false);
    wg.obstacle_locations.len() as u64
}
