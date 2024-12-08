use std::{isize, ops::RangeInclusive};

use crate::utils::grid::{Grid, Vertexable};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 04);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 04);
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

fn get_grid(input: &str) -> Grid {
    let lines: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut grid: Grid = Grid::new(lines[0].len() as isize, lines.len() as isize);
    for (y, line) in lines.iter().enumerate() {
        for (x, &token) in line.iter().enumerate() {
            grid.add_vertex((x as isize, y as isize), Box::new(Vertex { value: token }));
        }
    }
    grid
}

fn range_inclusive(start: isize, end: isize) -> RangeInclusive<isize> {
    let r: RangeInclusive<isize>;
    if start < end {
        r = start..=end;
    } else {
        r = end..=start;
    }
    r
}

impl Grid {
    fn get_string(&self, start: (isize, isize), end: (isize, isize)) -> Option<String> {
        if !self.is_inside(&start) || !self.is_inside(&end) {
            return None;
        }
        if start.0 != end.0
            && start.1 != end.1
            && end.0.abs_diff(start.0) != end.1.abs_diff(start.1)
        {
            return None;
        }
        let mut s = String::new();

        // column
        if start.0 == end.0 {
            for y in range_inclusive(start.1, end.1) {
                let c = self.get(&(start.0, y)).unwrap().get_value();
                s.push(c);
            }
            return Some(s);
        }

        // row
        if start.1 == end.1 {
            for x in range_inclusive(start.0, end.0) {
                let c = self.get(&(x, start.1)).unwrap().get_value();
                s.push(c);
            }
            return Some(s);
        }

        if end.0 - start.0 == end.1 - start.1 {
            // major diagonal
            for z in 0isize..=end.0.abs_diff(start.0) as isize {
                let c: char;
                if end.0 < start.0 {
                    match self.get_value(&(start.0 - z, start.1 - z)) {
                        Some(val) => c = val,
                        None => return None,
                    }
                } else {
                    match self.get_value(&(start.0 + z, start.1 + z)) {
                        Some(val) => c = val,
                        None => return None,
                    }
                }
                s.push(c);
            }
        } else {
            // minor diagonal
            for z in 0isize..=end.0.abs_diff(start.0) as isize {
                let c: char;
                if end.0 < start.0 {
                    match self.get_value(&(start.0 - z, start.1 + z)) {
                        Some(val) => c = val,
                        None => return None,
                    }
                } else {
                    match self.get_value(&(start.0 + z, start.1 - z)) {
                        Some(val) => c = val,
                        None => return None,
                    }
                }
                s.push(c);
            }
        }
        Some(s)
    }
}

fn count_occurrences(g: Grid) -> u64 {
    let mut solution = 0;

    // line
    for y in 0..g.height {
        for x in 0..g.width - 3 {
            let s = g.get_string((x, y), (x + 3, y)).unwrap();
            if s == "XMAS" || s == "SAMX" {
                solution += 1;
            }
        }
    }

    // column
    for y in 0..g.height - 3 {
        for x in 0..g.width {
            let s = g.get_string((x, y), (x, y + 3)).unwrap();
            if s == "XMAS" || s == "SAMX" {
                solution += 1;
            }
        }
    }

    // minor diagonal
    for y in 0..g.height - 3 {
        for x in 0..g.width - 3 {
            let s = g.get_string((x, y), (x + 3, y + 3)).unwrap();
            if s == "XMAS" || s == "SAMX" {
                solution += 1;
            }
        }
    }

    // major diagonal
    for y in 0..g.height - 3 {
        for x in 3..g.width {
            let s = g.get_string((x, y), (x - 3, y + 3)).unwrap();
            if s == "XMAS" || s == "SAMX" {
                solution += 1;
            }
        }
    }

    solution
}

fn solve_a(input: &str) -> u64 {
    let grid = get_grid(input);
    count_occurrences(grid)
}

fn count_mas_crosses(g: Grid) -> u64 {
    let mut count = 0;
    for y in 0..g.height {
        for x in 0..g.width {
            if g.get_value(&(x, y)).unwrap() == 'A' {
                let major = match g.get_string((x - 1, y - 1), (x + 1, y + 1)) {
                    Some(s) => s == "MAS" || s == "SAM",
                    None => false,
                };
                let minor = match g.get_string((x - 1, y + 1), (x + 1, y - 1)) {
                    Some(s) => s == "MAS" || s == "SAM",
                    None => false,
                };
                if major && minor {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_b(input: &str) -> u64 {
    let grid = get_grid(input);
    count_mas_crosses(grid)
}
