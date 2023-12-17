extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 13);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 13);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let solution: u64 = input
        .split("\n\n")
        .map(|group| {
            println!("checking group:\n{group}");
            let map = group
                .trim()
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            get_reflection_plane(map, 0)
        })
        .sum();
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let solution: u64 = input
        .split("\n\n")
        .map(|group| {
            println!("checking group:\n{group}");
            let map = group
                .trim()
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            get_reflection_plane(map, 1)
        })
        .sum();
    println!("part b: {}", solution);
}

fn get_reflection_plane(input: Vec<Vec<char>>, expected_diff: u64) -> u64 {
    let width = input[0].len();
    let height = input.len();

    for x in 1..width {
        let mut diff: u64 = 0;
        println!("checking x = {x}");
        for line in &input {
            for j in 0..x.min(width - x) {
                println!("\t{} == {}", line[x - j - 1], line[x + j]);
                if line[x - j - 1] != line[x + j] {
                    diff += 1;
                }
            }
        }
        if expected_diff == diff {
            println!("found vertical reflection at x = {x}");
            return x as u64;
        }
    }

    for x in 1..height {
        let mut diff: u64 = 0;
        println!("checking y = {x}");
        for j in 0..x.min(height - x) {
            for i in 0..width {
                println!("\t{} == {}", input[x - j - 1][i], input[x + j][i]);
                if input[x - j - 1][i] != input[x + j][i] {
                    diff += 1;
                }
            }
        }
        if expected_diff == diff {
            println!("found horizontal reflection at x = {x}");
            return (100 * x) as u64;
        }
    }

    return 0;
}

fn do_something_differently(input: &str) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::get_reflection_plane;

    #[test]
    fn test_find_vertical_reflection_plane_works() {
        let group = "##..##
.#..#.
#....#";
        let group = group
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let c = get_reflection_plane(group, 0);
        assert!(c == 3);
    }

    #[test]
    fn test_find_reflection_plane_works() {
        let group = "##..##
..#..#
..#..#
##..##
.#...#";
        let group = group
            .trim()
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let c = get_reflection_plane(group, 0);
        assert!(c == 2);
    }
}
