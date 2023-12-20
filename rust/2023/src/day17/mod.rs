use aoc_utils::coordinate::Coordinate;
use pathfinding::{directed::dijkstra::dijkstra, matrix::Matrix};

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
    println!("part a: {}", find_shortest_path(input, 1, 3));
}

pub fn solve_b(input: &str) {
    println!("part b: {}", find_shortest_path(input, 4, 10));
}

struct Board {
    grid: Matrix<u64>,
    max_step_size: usize,
    min_step_size: usize,
    end: (usize, usize),
}

impl Board {
    fn extend(
        &self,
        mut state: Vec<(((usize, usize), (isize, isize), usize), u64)>,
        pos: (usize, usize),
        direction: (isize, isize),
        length: usize,
    ) -> Vec<(((usize, usize), (isize, isize), usize), u64)> {
        state.extend(
            &self
                .grid
                .move_in_direction(pos, direction)
                .map(|t| ((t, direction, length), self.grid[t])),
        );
        state
    }

    fn successors(
        &self,
    ) -> impl Fn(
        &((usize, usize), (isize, isize), usize),
    ) -> Vec<(((usize, usize), (isize, isize), usize), u64)>
           + '_ {
        |&(pos, dir, length)| {
            let mut next = Vec::with_capacity(3);
            // if max step size is not reached we can continue in the current direction
            if length < self.max_step_size {
                next = self.extend(next, pos, dir, length + 1);
            }
            // turn left or right
            if length >= self.min_step_size {
                next = self.extend(next, pos, (-dir.1, -dir.0), 1);
                next = self.extend(next, pos, (dir.1, dir.0), 1);
            // at the start
            } else if length == 0 {
                next = self.extend(next, pos, (1, 0), 1);
                next = self.extend(next, pos, (0, 1), 1);
            }
            next
        }
    }

    fn success(&self) -> impl Fn(&((usize, usize), (isize, isize), usize)) -> bool + '_ {
        |&(pos, _, length)| pos == self.end && length >= self.min_step_size
    }
}

fn find_shortest_path(input: &str, min_step_size: usize, max_step_size: usize) -> u64 {
    let rows = input
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap() as u64));
    let grid = Matrix::from_rows(rows).unwrap();
    let end = (grid.rows - 1, grid.columns - 1);
    let board = Board {
        grid,
        min_step_size,
        max_step_size,
        end,
    };

    let result = dijkstra(&((0, 0), (0, 0), 0), board.successors(), board.success()).unwrap();
    // for c in result.0 {
    //     println!("[{}, {}] [{}, {}], {}", c.0 .0, c.0 .1, c.1 .0, c.1 .1, c.2);
    // }
    result.1
}
