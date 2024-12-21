use std::{collections::HashMap, u64};

use itertools::Itertools;
use nalgebra::ComplexField;

use crate::utils::vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 12);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 12);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

#[derive(Clone, Debug)]
struct Vertex {
    value: char,
    group_index: u64,
}

fn get_element(grid: &Vec<Vec<Vertex>>, x: isize, y: isize) -> Option<&Vertex> {
    let xu = x as usize;
    let yu = y as usize;
    if yu < grid.len() && xu < grid[yu].len() {
        Some(&grid[yu][xu])
    } else {
        None
    }
}

fn get_element_mut(grid: &mut Vec<Vec<Vertex>>, x: isize, y: isize) -> Option<&mut Vertex> {
    let xu = x as usize;
    let yu = y as usize;
    if yu < grid.len() && xu < grid[yu].len() {
        Some(&mut grid[yu][xu])
    } else {
        None
    }
}

fn parse_grid(input: &str) -> (Vec<Vec<Vertex>>, HashMap<u64, Vec<Vec2<isize>>>) {
    let map = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut idx = 0;
    let mut y = 0;
    let mut vertices = Vec::new();
    let mut regions = HashMap::new();
    for r in map {
        let mut row = Vec::new();
        let mut x = 0;
        for v in r {
            row.push(Vertex {
                value: v,
                group_index: idx,
            });
            regions.entry(idx).or_insert(Vec::new()).push((x, y).into());
            idx += 1;
            x += 1;
        }
        y += 1;
        vertices.push(row);
    }
    (vertices, regions)
}

fn analyze_groups(vertices: &mut Vec<Vec<Vertex>>, regions: &mut HashMap<u64, Vec<Vec2<isize>>>) {
    let ymax = vertices.len();
    let xmax = vertices[0].len();
    for y in 0..ymax as isize {
        for x in 0..xmax as isize {
            let vertex = get_element(&vertices, x, y).unwrap().clone();
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                let mut neighbor_index = u64::MAX;
                if let Some(neighbor) = get_element_mut(vertices, x + dx, y + dy) {
                    if vertex.value == neighbor.value && vertex.group_index != neighbor.group_index
                    {
                        neighbor_index = neighbor.group_index;
                        let mut vs = regions.get_mut(&neighbor.group_index).unwrap().clone();
                        regions
                            .entry(vertex.group_index)
                            .and_modify(|v| v.append(&mut vs));
                        neighbor.group_index = vertex.group_index;
                    }
                }
                if let Some(positions) = regions.get(&neighbor_index) {
                    for p in positions {
                        let v = get_element_mut(vertices, p.x, p.y).unwrap();
                        v.group_index = vertex.group_index;
                    }
                    regions.remove(&neighbor_index);
                }
            }
        }
    }
}

fn solve_a(input: &str) -> u64 {
    let mut solution = 0;
    let (mut vertices, mut regions) = parse_grid(input);
    analyze_groups(&mut vertices, &mut regions);
    for v in regions.values() {
        let mut edges = 0;
        let area = v.len();
        for vx in v {
            let vertex = get_element(&vertices, vx.x, vx.y).unwrap().clone();
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
                if let Some(neighbor) = get_element_mut(&mut vertices, vx.x + dx, vx.y + dy) {
                    if vertex.value != neighbor.value {
                        edges += 1;
                    }
                } else {
                    edges += 1;
                }
            }
        }
        solution += edges * area;
    }

    solution as u64
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let (mut vertices, mut regions) = parse_grid(input);
    analyze_groups(&mut vertices, &mut regions);
    for v in regions.values() {
        let mut corners = 0;
        let area = v.len() as u64;
        for vx in v {
            let c = check_corner(&vertices, vx.x, vx.y);
            corners += c;
        }
        solution += corners * area;
    }

    solution as u64
}

fn check_corner(vertices: &Vec<Vec<Vertex>>, x: isize, y: isize) -> u64 {
    let mut count = 0;
    let mut x1 = Vec2::from((-1, 0));
    let mut x2 = Vec2::from((0, -1));
    let idx = get_element(vertices, x, y).unwrap().group_index;
    for _ in 0..4 {
        x1 = Vec2::from((-x1.y, x1.x));
        x2 = Vec2::from((-x2.y, x2.x));
        let v1 = get_element(vertices, x + x1.x, y + x1.y);
        let v2 = get_element(vertices, x + x2.x, y + x2.y);
        let v3 = get_element(vertices, x + x1.x + x2.x, y + x1.y + x2.y);

        // OO
        // OX <- vertex (x,y)
        if (v1.is_none() || v1.unwrap().group_index != idx)
            && (v2.is_none() || v2.unwrap().group_index != idx)
            && (v3.is_none() || v3.unwrap().group_index != idx)
        {
            count += 1;
        }
        if v1.is_some() && v2.is_some() && v3.is_some() {
            let i1 = v1.unwrap().group_index;
            let i2 = v2.unwrap().group_index;
            let i3 = v3.unwrap().group_index;
            // OX
            // XX <- vertex (x,y)
            if i1 == idx && i2 == idx && i3 != idx {
                count += 1;
            }
            // XO
            // OX <- vertex (x,y)
            if i1 != idx && i2 != idx && i3 == idx {
                count += 1;
            }
        }
    }
    count
}
