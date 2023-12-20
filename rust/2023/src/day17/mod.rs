use aoc_utils::coordinate::Coordinate;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 17);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 17);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let grid = &mut parse_grid(input);
    find_cheapest_route(grid, grid.start, None, None, 1, 3, 0);
    print_path(grid);
    println!("part a: {}", grid.data.last().unwrap().last().unwrap().cost);
}

pub fn solve_b(input: &str) {
    let solution = do_something_differently(input);
    println!("part b: {}", solution);
}

fn print_path(grid: &mut Grid) {
    let mut pos = grid.end;
    println!("{}", pos);
    while pos != grid.start {
        pos = grid.get_mut(&pos).previous.unwrap();
        println!("{}", pos);
    }
}

struct Tile {
    weight: usize,
    visited: bool,
    cost: usize,
    previous: Option<Coordinate<isize>>,
}

struct Grid {
    data: Vec<Vec<Tile>>,
    start: Coordinate<isize>,
    end: Coordinate<isize>,
}

impl Grid {
    fn step_valid(&self, target: Coordinate<isize>) -> bool {
        target.x >= 0
            && target.y >= 0
            && (target.x as usize) < self.data[0].len()
            && (target.y as usize) < self.data.len()
    }

    fn get_mut(&mut self, coordinate: &Coordinate<isize>) -> &mut Tile {
        self.data
            .get_mut(coordinate.y as usize)
            .unwrap()
            .get_mut(coordinate.x as usize)
            .unwrap()
    }

    fn get_weight(&mut self, coordinate: &Coordinate<isize>) -> usize {
        self.data
            .get(coordinate.y as usize)
            .unwrap()
            .get(coordinate.x as usize)
            .unwrap()
            .weight
    }
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn find_cheapest_route(
    grid: &mut Grid,
    position: Coordinate<isize>,
    previous_position: Option<Coordinate<isize>>,
    previous_dir: Option<Coordinate<isize>>,
    min_step: isize,
    max_step: isize,
    mut cost: usize,
) {
    if position == grid.end {
        println!();
    }
    let tile = grid.get_mut(&position);
    if tile.cost < cost {
        return;
    }
    cost += tile.weight;
    if tile.cost > cost {
        tile.cost = cost;
        tile.previous = previous_position;
    }
    println!(
        "tile: {}, weight: {}, cost: {}",
        position, tile.weight, tile.cost
    );
    if position == grid.end {
        return;
    }

    for dir in DIRECTIONS {
        let dir = Coordinate::from(dir);
        if let Some(prev_dir) = previous_dir {
            if prev_dir == dir || prev_dir == dir * -1 {
                continue;
            }
        }
        for step_size in min_step..=max_step {
            let step = dir * step_size;
            let target = position + step;
            if grid.step_valid(target) {
                let mut intermediate_costs = 0;
                for i in 1..step_size {
                    intermediate_costs += grid.get_weight(&(position + dir * i));
                }
                find_cheapest_route(
                    grid,
                    target,
                    Some(position),
                    Some(dir),
                    min_step,
                    max_step,
                    cost + intermediate_costs,
                );
            }
        }
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut data: Vec<Vec<Tile>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<Tile> = Vec::new();
        for char in line.trim().chars() {
            row.push(Tile {
                weight: char.to_digit(10).unwrap() as usize,
                visited: false,
                cost: usize::MAX,
                previous: None,
            })
        }
        data.push(row);
    }
    let x = (data[0].len() - 1) as isize;
    let y = (data.len() - 1) as isize;
    Grid {
        data,
        start: Coordinate { x: 0, y: 0 },
        end: Coordinate { x, y },
    }
}

fn do_something_differently(input: &str) -> usize {
    return 0;
}
