use crate::utils::vec2::Vec2;
use std::collections::BTreeSet;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 11);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 11);
    b.iter(|| solve_b(&input));
}
#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let galaxy = parse_galaxy(input);
    let galaxy = expand_empty_columns_rows(galaxy, 2);
    let solution = count_galaxy_paths(&galaxy);
    solution as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let galaxy = parse_galaxy(input);
    let galaxy = expand_empty_columns_rows(galaxy, 1000000);
    let solution = count_galaxy_paths(&galaxy);
    solution as u64
}

struct Galaxy {
    raw_galaxy: Vec<Vec<char>>,
    nodes: Vec<Vec2<isize>>,
}

impl std::fmt::Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.raw_galaxy {
            for char in row {
                write!(f, "{}", char)?;
            }
            writeln!(f, "")?;
        }
        return Ok(());
    }
}

fn parse_galaxy(input: &str) -> Galaxy {
    let raw_galaxy: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let galaxy = Galaxy {
        raw_galaxy,
        nodes: Vec::new(),
    };
    let galaxy = parse_nodes(galaxy);
    galaxy
}

fn parse_nodes(mut galaxy: Galaxy) -> Galaxy {
    for (y, row) in galaxy.raw_galaxy.iter().enumerate() {
        for (x, &node) in row.iter().enumerate() {
            if node == '#' {
                galaxy.nodes.push(Vec2 {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }
    galaxy
}

fn count_galaxy_paths(galaxy: &Galaxy) -> isize {
    let mut distances = 0isize;
    for i in 0..galaxy.nodes.len() {
        for j in 0..i {
            let node_a = galaxy.nodes.get(i).unwrap();
            let node_b = galaxy.nodes.get(j).unwrap();
            distances += (node_a.x - node_b.x).abs();
            distances += (node_a.y - node_b.y).abs()
        }
    }
    return distances;
}

fn expand_empty_columns_rows(mut galaxy: Galaxy, scale: isize) -> Galaxy {
    let xs: Vec<isize> = (0..galaxy.raw_galaxy.len() as isize).collect();
    let ys: Vec<isize> = (0..galaxy.raw_galaxy[0].len() as isize).collect();
    let mut empty_columns: BTreeSet<isize> = BTreeSet::from_iter(xs);
    let mut empty_rows: BTreeSet<isize> = BTreeSet::from_iter(ys);
    for node in galaxy.nodes.iter() {
        empty_columns.remove(&node.x);
        empty_rows.remove(&node.y);
    }
    println!("empty rows: {:?}", empty_rows);
    println!("empty cols: {:?}", empty_columns);

    for row in empty_rows.into_iter().rev() {
        galaxy
            .nodes
            .iter_mut()
            .filter(|n| n.y > row)
            .for_each(|n| n.y += scale - 1);
    }
    for col in empty_columns.into_iter().rev() {
        galaxy
            .nodes
            .iter_mut()
            .filter(|n| n.x > col)
            .for_each(|n| n.x += scale - 1);
    }
    galaxy
}
