extern crate test;

use nom::{character::complete::anychar, multi::many1, IResult};
use rand::seq::SliceRandom;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 10);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 10);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    println!("{}", input);
    let solution = do_something(input);
    println!("part a: {}", solution);
}

#[derive(PartialEq, Eq)]
enum Connection {
    North,
    East,
    South,
    West,
}

fn to_vec(con: &Connection) -> (isize, isize) {
    match con {
        Connection::North => (-1, 0),
        Connection::East => (0, 1),
        Connection::South => (1, 0),
        Connection::West => (0, -1),
    }
}

struct Pipe {
    connections: Vec<Connection>,
}

impl Pipe {
    fn get_random_connection(&self) -> &Connection {
        self.connections.choose(&mut rand::thread_rng()).unwrap()
    }

    fn get_next_except(&self, con: &Connection) -> &Connection {
        self.connections
            .iter()
            .filter(|c| *c != con)
            .next()
            .unwrap()
    }
}

struct Grid {
    pipes: Vec<Vec<Pipe>>,
    dimensions: (isize, isize),
    start: (isize, isize),
}

impl Grid {
    fn get(&self, (x, y): (isize, isize)) -> &Pipe {
        self.pipes.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn get_start(&self) -> &Pipe {
        self.get((self.start.1, self.start.0))
    }
}

fn char_to_pipe(c: char) -> Option<Pipe> {
    match c {
        '|' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        '-' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        'J' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        '7' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        'L' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        'F' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
        }),
        '.' => None,
        'S' => Some(Pipe {
            connections: Vec::new(),
        }),
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let mut pipes: Vec<Vec<Pipe>> = Vec::new();
    let mut dimensions = (0isize, 0isize);
    let mut start = (0isize, 0isize);
    let (input, pipes) = many1(|line| -> IResult<&str, Vec<Vec<Pipe>>> {
        let (line, v) = many1(|c| -> IResult<&str, Vec<Pipe>> {
            let (c, con) = anychar(c)?;
            if let Some(pipe) = char_to_pipe(c) {
                return Ok((c, pipe));
            }
        })?;
    })?;
    Grid {
        pipes,
        dimensions,
        start,
    };
}

fn find_loop_length(grid: Grid) -> u32 {
    let count = 0;
    let start = grid.start;
    let current = grid.start.clone();

    let mut con = grid.get(current).get_random_connection();
    let dir = to_vec(con);
    let mut current = (current.0 + dir.0, current.1 + dir.1);

    while current != start {
        con = grid.get(current).get_next_except(con);
        let dir = to_vec(con);
        current = (current.0 + dir.0, current.1 + dir.1);
    }
    count
}

pub fn solve_b(input: &str) {
    let solution = do_something_differently(input);
    println!("part b: {}", solution);
}

fn do_something(input: &str) -> u64 {
    return 0;
}

fn do_something_differently(input: &str) -> u64 {
    return 0;
}
