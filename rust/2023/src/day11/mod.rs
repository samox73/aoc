use std::collections::{BTreeSet, HashSet};

use crate::utils::coordinates::Coordinate;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 11);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 11);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let galaxy = parse_galaxy(input);
    let galaxy = expand_empty_columns_rows(galaxy, 2);
    let solution = count_galaxy_paths(&galaxy);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let galaxy = parse_galaxy(input);
    let galaxy = expand_empty_columns_rows(galaxy, 1000000);
    let solution = count_galaxy_paths(&galaxy);
    println!("part b: {}", solution);
}

struct Galaxy {
    raw_galaxy: Vec<Vec<char>>,
    nodes: Vec<Coordinate>,
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
                galaxy.nodes.push(Coordinate {
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

    // let mut new_raw_galaxy: Vec<Vec<char>> = Vec::new();
    // for row in &galaxy.raw_galaxy {
    //     if row.iter().all_equal() {
    //         new_raw_galaxy.push(row.clone());
    //     }
    //     new_raw_galaxy.push(row.clone());
    // }
    // // iterate inverse so that we don't have to account for changing char positions
    // for i in (0..galaxy.raw_galaxy[0].len()).rev() {
    //     let column: Vec<char> = galaxy
    //         .raw_galaxy
    //         .iter()
    //         .map(|l| *l.get(i).unwrap())
    //         .collect();
    //     if column.iter().all_equal() {
    //         for j in 0..new_raw_galaxy.len() {
    //             new_raw_galaxy.get_mut(j).unwrap().insert(i, '.');
    //         }
    //     }
    // }
    // galaxy.raw_galaxy = new_raw_galaxy;
    // galaxy
}
