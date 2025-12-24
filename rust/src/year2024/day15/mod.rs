use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 15);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 15);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

struct Grid {
    tiles: Vec<Vec<char>>,
    moves: Vec<char>,
    robot: Vec2<isize>,
}

impl Grid {
    fn print(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.robot.x as usize == x && self.robot.y as usize == y {
                    print!("@");
                } else {
                    print!("{tile}");
                }
            }
            println!()
        }
    }

    fn run_moves(&mut self) {
        for m in &self.moves {
            // println!("running move {m}");
            let vel = match m {
                '>' => (1, 0).into(),
                'v' => (0, 1).into(),
                '<' => (-1, 0).into(),
                '^' => (0, -1).into(),
                _ => unreachable!(),
            };
            let next = self.robot + vel;
            match self.tiles[next.y as usize][next.x as usize] {
                '#' => continue,
                '.' => {
                    self.robot += vel;
                    continue;
                }
                _ => (),
            }
            let boxes = self.collect_box_tree(self.robot, vel);
            if boxes.len() == 0 {
                continue;
            }
            // println!("collected boxes: {:?}", boxes);
            let (first, last) = (boxes[0], boxes[boxes.len() - 1]);
            self.tiles[first.y as usize][first.x as usize] = '.';
            self.tiles[last.y as usize][last.x as usize] = 'O';
            self.robot += vel;
        }
    }

    fn collect_box_tree(
        &self,
        mut pos: Vec2<isize>,
        vel: Vec2<isize>,
    ) -> Option<HashMap<Vec2<isize>, char>> {
        let mut boxes = HashMap::new();
        let mut stack = Vec::new();
        stack.push(pos);
        while !stack.is_empty() {
            let p = stack.pop().unwrap() + vel;
            let val = self.tiles[pos.y as usize][pos.x as usize];
            if val != 'O' && val != '[' && val != ']' {
                break;
            }
            boxes.insert(pos, val);
        }
        if self.tiles[pos.y as usize][pos.x as usize] == '.' {
            return Some(boxes);
        } else {
            return None;
        }
    }

    fn get_gps_coords(&self) -> u64 {
        let mut s = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == 'O' {
                    s += (y * 100 + x) as u64;
                }
            }
        }
        s
    }
}

fn parse_grid(input: &str, wide: bool) -> Grid {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut tiles = Vec::new();
    let mut robot = (0, 0).into();
    for (y, line) in grid_input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '@' => {
                    if wide {
                        robot = (2 * x as isize, y as isize).into();
                        row.push('.');
                        row.push('.');
                    } else {
                        robot = (x as isize, y as isize).into();
                        row.push('.');
                    }
                }
                'O' => {
                    if wide {
                        row.push('[');
                        row.push(']');
                    } else {
                        row.push('O');
                    }
                }
                _ => {
                    if wide {
                        row.push(tile);
                        row.push(tile);
                    } else {
                        row.push(tile);
                    }
                }
            }
        }
        tiles.push(row);
    }
    let moves = moves_input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect_vec();
    Grid {
        tiles,
        moves,
        robot,
    }
}

fn solve_a(input: &str) -> u64 {
    println!("{}", input);
    let mut grid = parse_grid(input, false);
    grid.run_moves();
    grid.print();
    grid.get_gps_coords()
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut grid = parse_grid(input, true);
    // grid.run_moves();
    grid.print();
    // grid.get_gps_coords()
    return 0;
}
