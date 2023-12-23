use itertools::Itertools;
use pathfinding::{directed::dijkstra::dijkstra_reach, grid, matrix::Matrix};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 21);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 21);
    b.iter(|| solve_b(&input));
}

pub fn solve_a(input: &str) {
    let solution = count_reachable_positions(input, false, 64);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let x0 = 65;
    let y0 = count_reachable_positions(input, true, x0);
    println!("y0: {y0}");
    let x1 = 65 + 131;
    let y1 = count_reachable_positions(input, true, x1);
    println!("y1: {y1}");
    let x2 = 65 + 131 * 2;
    let y2 = count_reachable_positions(input, true, x2);
    println!("y2: {y2}");
    let (a, b, c) = interpolate_quadratic(
        x0 as f64, x1 as f64, x2 as f64, y0 as f64, y1 as f64, y2 as f64,
    );
    let x = (202300 * 131 + 65) as f64;
    let solution = (a * x.powi(2) + b * x + c) as u64;
    println!("part b: {}", solution);
}

type State = ((isize, isize), usize);

fn map_to_unit_cell(
    coords: (isize, isize),
    bounds: (usize, usize),
) -> ((usize, usize), (isize, isize)) {
    (
        (
            (coords.0.rem_euclid(bounds.0 as isize)) as usize,
            (coords.1.rem_euclid(bounds.1 as isize)) as usize,
        ),
        (coords.0 / bounds.0 as isize, coords.1 / bounds.1 as isize),
    )
}

fn interpolate_quadratic(x0: f64, x1: f64, x2: f64, y0: f64, y1: f64, y2: f64) -> (f64, f64, f64) {
    let a = ((y0 - y1) * (x0 - x2) - (y0 - y2) * (x0 - x1))
        / ((x0.powi(2) - x1.powi(2)) * (x0 - x2) - (x0.powi(2) - x2.powi(2)) * (x0 - x1));
    let b = ((y0 - y1) - a * (x0.powi(2) - x1.powi(2))) / (x0 - x1);
    let c = y0 - a * x0.powi(2) - b * x0;
    return (a, b, c);
}

fn get_neighbors(state: &State) -> Vec<(isize, isize)> {
    let mut v = Vec::new();
    for x in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        v.push((state.0 .0 + x.0, state.0 .1 + x.1));
    }
    v
}

fn count_reachable_positions(input: &str, extend: bool, step_count: usize) -> u64 {
    let rows = input.lines().map(|l| l.trim().chars());
    let grid = Matrix::from_rows(rows).unwrap();
    let start = grid
        .items()
        .filter(|(_, &v)| v == 'S')
        .map(|e| (e.0 .0 as isize, e.0 .1 as isize))
        .next()
        .unwrap();
    let bounds = (grid.columns, grid.rows);
    let start: State = (start, 0);
    let results = dijkstra_reach(&start, |state: &State, _| {
        let mut legal_moves: Vec<(State, u64)> = Vec::new();
        for neighbor in get_neighbors(state) {
            if (neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= bounds.0 as isize
                || neighbor.1 >= bounds.1 as isize)
                && !extend
            {
                continue;
            }
            let (unit_neighbor, _) = map_to_unit_cell(neighbor, bounds);
            let tile_value = *grid.get(unit_neighbor).unwrap();
            if tile_value != '#' && state.1 < step_count {
                legal_moves.push(((neighbor, state.1 + 1), 1));
            }
        }
        legal_moves
    });
    let results = results
        .filter(|c| c.node.1 == step_count)
        .sorted_by(|a, b| a.node.1.cmp(&b.node.1))
        .unique_by(|c| c.node.0);
    let count = results.count();
    return count as u64;
}

#[cfg(test)]
mod tests {
    use crate::day21::interpolate_quadratic;

    #[test]
    fn interpolation_works() {
        let res = interpolate_quadratic(0f64, 1f64, 2f64, 1f64, 2f64, 5f64);
        assert_eq!(res, (1f64, 0f64, 1f64));
    }
}
