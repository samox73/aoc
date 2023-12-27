use aocutils::Vec2::Vec2;
use nom::IResult;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 18);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 18);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let (vertices, perimeter) = parse_vertices(input, false);
    let count = perimeter / 2 + shoelace(vertices) / 2 + 1;
    println!("part a: {}", count);
}

pub fn solve_b(input: &str) {
    let (vertices, perimeter) = parse_vertices(input, true);
    let count = perimeter / 2 + shoelace(vertices) / 2 + 1;
    println!("part b: {}", count);
}

fn parse_direction(d: &str) -> (isize, isize) {
    match d {
        "R" | "0" => (1, 0),
        "D" | "1" => (0, 1),
        "L" | "2" => (-1, 0),
        "U" | "3" => (0, -1),
        _ => unreachable!(),
    }
}

fn parse_line(line: &str, part_b: bool) -> IResult<&str, ((isize, isize), u64)> {
    let (line, dir) = nom::character::complete::alpha1(line)?;
    let dir = parse_direction(dir);
    let (line, _) = nom::character::complete::space1(line)?;
    let (line, length) = nom::character::complete::u64(line)?;
    let (line, _) = nom::character::complete::space1(line)?;
    if !part_b {
        return Ok((line, (dir, length)));
    }
    let color = line.trim_matches('(').trim_matches(')');
    let length = u64::from_str_radix(&color[1..6], 16).unwrap();
    let dir = parse_direction(&color[6..]);
    return Ok((line, (dir, length)));
}

fn determinant(v1: Vec2<isize>, v2: Vec2<isize>) -> isize {
    v1.x * v2.y - v1.y * v2.x
}

fn shoelace(data: Vec<Vec2<isize>>) -> usize {
    let mut area = 0;
    for i in 0..data.len() {
        let v1 = data.get(i).unwrap();
        let v2 = data.get((i + 1) % data.len()).unwrap();
        area += determinant(*v1, *v2);
    }
    area.abs() as usize
}

fn parse_vertices(input: &str, part_b: bool) -> (Vec<Vec2<isize>>, usize) {
    let mut data: Vec<Vec2<isize>> = Vec::new();
    let mut position: Vec2<isize> = Vec2::from((0, 0));
    let mut perimeter: usize = 0;
    for line in input.lines().into_iter() {
        let (direction, length) = parse_line(line.trim(), part_b).unwrap().1;
        for _ in 0..length {
            perimeter += 1;
            position += Vec2::from(direction);
            data.push(position.clone());
        }
    }
    (data, perimeter)
}

#[cfg(test)]
mod tests {
    use aocutils::Vec2::Vec2;

    use super::shoelace;

    #[test]
    fn shoelace_works() {
        // example from https://en.wikipedia.org/wiki/Shoelace_formula#Example
        let vs = vec![
            Vec2::from((1, 6)),
            Vec2::from((3, 1)),
            Vec2::from((7, 2)),
            Vec2::from((4, 4)),
            Vec2::from((8, 5)),
        ];
        let res = shoelace(vs) as f64 / 2.;
        assert_eq!(res, 16.5);
    }
}
