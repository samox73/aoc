extern crate test;

use nom::{character::complete::anychar, multi::many1, IResult};
use rand::seq::SliceRandom;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 10);
    b.iter(|| solve(&input));
}
#[allow(dead_code)]
pub fn solve(input: &str) {
    let grid = parse_grid(input).unwrap().1;
    let (length, grid) = find_loop_length(grid);
    println!("part a: {}", length / 2);
    let count = count_inside(grid);
    println!("part b: {}", count);
}

fn count_inside(grid: Grid) -> usize {
    let mut count = 0;
    for y in 0..grid.dimensions.1 {
        let mut inside = false;
        for x in 0..grid.dimensions.0 {
            let pipe = grid.get((x, y));
            if pipe.part_of_loop {
                if "|JLS".contains(pipe.symbol) {
                    inside = !inside;
                }
            } else {
                if inside {
                    count += 1
                }
            }
        }
    }
    count
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
        Connection::North => (0, -1),
        Connection::East => (1, 0),
        Connection::South => (0, 1),
        Connection::West => (-1, 0),
    }
}

fn invert(con: &Connection) -> &Connection {
    match con {
        Connection::North => &Connection::South,
        Connection::East => &Connection::West,
        Connection::South => &Connection::North,
        Connection::West => &Connection::East,
    }
}

struct Pipe {
    connections: Vec<Connection>,
    part_of_loop: bool,
    symbol: char,
}

impl Pipe {
    fn get_random_connection(&self) -> &Connection {
        self.connections.choose(&mut rand::thread_rng()).unwrap()
    }

    fn get_next_except(&self, con: &Connection) -> &Connection {
        self.connections
            .iter()
            .filter(|c| *c != invert(con))
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
}

fn char_to_pipe(c: char) -> Option<Pipe> {
    match c {
        '|' => Some(Pipe {
            connections: vec![Connection::North, Connection::South],
            part_of_loop: false,
            symbol: c,
        }),
        '-' => Some(Pipe {
            connections: vec![Connection::East, Connection::West],
            part_of_loop: false,
            symbol: c,
        }),
        'J' => Some(Pipe {
            connections: vec![Connection::North, Connection::West],
            part_of_loop: false,
            symbol: c,
        }),
        '7' => Some(Pipe {
            connections: vec![Connection::South, Connection::West],
            part_of_loop: false,
            symbol: c,
        }),
        'L' => Some(Pipe {
            connections: vec![Connection::North, Connection::East],
            part_of_loop: false,
            symbol: c,
        }),
        'F' => Some(Pipe {
            connections: vec![Connection::East, Connection::South],
            part_of_loop: false,
            symbol: c,
        }),
        '.' => None,
        'S' => Some(Pipe {
            connections: Vec::new(),
            part_of_loop: false,
            symbol: c,
        }),
        _ => {
            println!("Invalid character '{}'", c);
            None
        }
    }
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    let mut pipes: Vec<Vec<Pipe>> = Vec::new();
    let mut current_pos = (0isize, 0isize);
    let mut start = (0isize, 0isize);
    let mut xmax = 0;
    let mut ymax = 0;
    for line in input.lines() {
        let (_, v) = many1(|c| -> IResult<&str, Pipe> {
            let (c, con) = anychar(c)?;
            let current = current_pos.clone();
            xmax = xmax.max(current.0);
            current_pos.0 += 1;
            if let Some(mut pipe) = char_to_pipe(con) {
                if con == 'S' {
                    pipe.connections.push(Connection::North);
                    pipe.connections.push(Connection::South);
                    start = current;
                }
                return Ok((c, pipe));
            } else {
                return Ok((
                    c,
                    Pipe {
                        connections: Vec::new(),
                        part_of_loop: false,
                        symbol: ' ',
                    },
                ));
            }
        })(line)?;
        ymax = ymax.max(current_pos.1);
        current_pos.0 = 0;
        current_pos.1 += 1;
        pipes.push(v);
    }
    return Ok((
        input,
        Grid {
            pipes,
            dimensions: (xmax, ymax),
            start,
        },
    ));
}

fn find_loop_length(grid: Grid) -> (u32, Grid) {
    let mut count = 0;
    let start = grid.start;
    let mut current = grid.start.clone();

    // grid.get_mut(current).part_of_loop = true;
    let mut con = grid.get(current).get_random_connection();
    // let dir = to_vec(con);
    // let mut current = (current.0 + dir.0, current.1 + dir.1);
    // count += 1;

    while current != start {
        // grid.get_mut(current).part_of_loop = true;
        if count == 0 {}
        con = grid.get(current).get_next_except(con);
        let dir = to_vec(con);
        current = (current.0 + dir.0, current.1 + dir.1);
        count += 1;
    }

    (count, grid)
}
