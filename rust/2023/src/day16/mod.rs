use std::fmt::Display;

use aocutils::Vec2::Vec2;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 16);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 16);
    b.iter(|| solve_b(&input));
}

pub fn solve_a(input: &str) {
    let beam = Beam {
        pos: Vec2 { x: 0, y: 0 },
        dir: Vec2 { x: 1, y: 0 },
    };
    let floor = &mut parse_floor(input);
    follow_beam(floor, beam);
    let solution: usize = floor
        .grid
        .iter()
        .map(|row| row.iter().filter(|t| t.lit).count())
        .sum();
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let original_floor = &mut parse_floor(input);

    let mut solution = 0;
    for x in 0..original_floor.grid[0].len() {
        let floor = &mut original_floor.clone();
        let beam = Beam {
            pos: Vec2 {
                x: x as isize,
                y: 0,
            },
            dir: Vec2 { x: 0, y: 1 },
        };
        follow_beam(floor, beam);
        let count: usize = floor.count_lit();
        solution = solution.max(count);

        let floor = &mut original_floor.clone();
        let beam = Beam {
            pos: Vec2 {
                x: x as isize,
                y: original_floor.grid.len() as isize - 1,
            },
            dir: Vec2 { x: 0, y: -1 },
        };
        follow_beam(floor, beam);
        let count: usize = floor.count_lit();
        solution = solution.max(count);
    }
    for y in 0..original_floor.grid.len() {
        let floor = &mut original_floor.clone();
        let beam = Beam {
            pos: Vec2 {
                x: 0,
                y: y as isize,
            },
            dir: Vec2 { x: 1, y: 0 },
        };
        follow_beam(floor, beam);
        let count: usize = floor.count_lit();
        solution = solution.max(count);

        let floor = &mut original_floor.clone();
        let beam = Beam {
            pos: Vec2 {
                x: original_floor.grid[0].len() as isize - 1,
                y: y as isize,
            },
            dir: Vec2 { x: -1, y: 0 },
        };
        follow_beam(floor, beam);
        let count: usize = floor.count_lit();
        solution = solution.max(count);
    }
    println!("part b: {}", solution);
}

fn parse_floor(input: &str) -> Floor {
    let mut grid: Vec<Vec<Tile>> = Vec::new();
    for line in input.lines() {
        let mut v: Vec<Tile> = Vec::new();
        for char in line.chars() {
            v.push(Tile {
                variant: char,
                lit: false,
                beams: Vec::new(),
            });
        }
        grid.push(v);
    }
    Floor { grid }
}

#[derive(Clone)]
struct Tile {
    variant: char,
    lit: bool,
    beams: Vec<Vec2<isize>>,
}

#[derive(Clone)]
struct Floor {
    grid: Vec<Vec<Tile>>,
}

impl Floor {
    fn get_mut(&mut self, Vec2: &Vec2<isize>) -> &mut Tile {
        self.grid
            .get_mut(Vec2.y as usize)
            .unwrap()
            .get_mut(Vec2.x as usize)
            .unwrap()
    }

    fn is_outside(&self, beam: &Beam) -> bool {
        // check < 0 before casting to usize
        beam.pos.y < 0
            || beam.pos.x < 0
            || (beam.pos.y as usize) >= self.grid.len()
            || (beam.pos.x as usize) >= self.grid[0].len()
    }

    fn count_lit(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|t| t.lit).count())
            .sum()
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for tile in row {
                match (tile.variant, tile.lit) {
                    (_, true) => write!(f, "#")?,
                    (_, false) => write!(f, ".")?,
                    // ('.', true) => write!(f, "#")?,
                    // ('.', false) => write!(f, ".")?,
                    // (_, _) => write!(f, "{}", tile.variant)?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Clone)]
struct Beam {
    pos: Vec2<isize>,
    dir: Vec2<isize>,
}

fn beam_continues(tile: &Tile, beam: &Beam) -> bool {
    match (tile.variant, (beam.dir.x, beam.dir.y)) {
        ('.', _) => true,
        ('-', (1, 0)) | ('-', (-1, 0)) => true,
        ('-', (0, 1)) | ('-', (0, -1)) => false,
        ('|', (1, 0)) | ('|', (-1, 0)) => false,
        ('|', (0, 1)) | ('|', (0, -1)) => true,
        (_, _) => false,
    }
}

fn follow_beam(floor: &mut Floor, mut beam: Beam) {
    if floor.is_outside(&beam) {
        return;
    }

    // terminate if beam has already passed through in this direction
    let mut tile = floor.get_mut(&beam.pos);
    if tile.beams.iter().any(|&b| b == beam.dir) {
        return;
    }

    // let term = console::Term::buffered_stdout();
    // if let Err(err) = term.read_char() {
    //     println!("encountered an error while reading char: {}", err);
    // };

    while beam_continues(&tile, &beam) {
        // cache that a beam has already passed this tile in this direction
        tile.beams.push(beam.dir.clone());
        tile.lit = true;
        beam.pos += beam.dir;
        if floor.is_outside(&beam) {
            return;
        }
        tile = floor.get_mut(&beam.pos);
    }

    tile.lit = true;
    match tile.variant {
        '/' => {
            beam.dir = Vec2::from((-beam.dir.y, -beam.dir.x));
            beam.pos += beam.dir;
            follow_beam(floor, beam);
        }
        '\\' => {
            beam.dir = Vec2::from((beam.dir.y, beam.dir.x));
            beam.pos += beam.dir;
            follow_beam(floor, beam);
        }
        '-' => {
            let mut east = beam.clone();
            let mut west = beam.clone();
            east.dir = Vec2::from((1, 0));
            follow_beam(floor, east);
            west.dir = Vec2::from((-1, 0));
            follow_beam(floor, west);
        }
        '|' => {
            let mut north = beam.clone();
            let mut south = beam.clone();
            north.dir = Vec2::from((0, -1));
            follow_beam(floor, north);
            south.dir = Vec2::from((0, 1));
            follow_beam(floor, south);
        }
        _ => unreachable!(),
    }
}
