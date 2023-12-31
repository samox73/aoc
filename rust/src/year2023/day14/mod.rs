use std::collections::HashMap;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 14);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 14);
    b.iter(|| solve_b(&input));
}

fn get_transposed(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut transposed: Vec<Vec<char>> = Vec::new();
    for i in 0..matrix[0].len() {
        let row = (0..matrix.len())
            .map(|j| matrix[j][i])
            .collect::<Vec<char>>();
        transposed.push(row);
    }
    transposed
}

fn shift_north(matrix: &mut Vec<Vec<char>>) {
    matrix.iter_mut().for_each(|row| shift_row_north(row));
}

fn shift_row_north(row: &mut Vec<char>) {
    let length = row.len();
    let mut idx_inside_group = 0;
    let mut group_location = 0;
    for i in 0..length {
        if row[i] == 'O' {
            row[i] = '.';
            row[group_location + idx_inside_group] = 'O';
            idx_inside_group += 1;
        }
        if row[i] == '#' {
            idx_inside_group = 1;
            group_location = i;
        }
    }
}

fn get_north_load(matrix: &Vec<Vec<char>>) -> u64 {
    matrix.iter().map(|row| get_row_north_load(row)).sum()
}

fn get_row_north_load(row: &Vec<char>) -> u64 {
    let length = row.len();
    let mut load = 0;
    for i in 0..length {
        if row[i] == 'O' {
            load += length - i;
        }
    }
    load as u64
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn rotate_ccw(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    get_transposed(matrix).into_iter().rev().collect()
}

fn rotate_cw(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    get_transposed(matrix.into_iter().rev().collect())
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let matrix = get_matrix(input);
    let mut rotated = rotate_ccw(matrix);
    shift_north(&mut rotated);
    let total_load: u64 = get_north_load(&rotated);
    total_load
}

fn spin_once(mut matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // shift north
    shift_north(&mut matrix);

    // shift west
    matrix = rotate_cw(matrix);
    _ = shift_north(&mut matrix);

    // shift south
    matrix = rotate_cw(matrix);
    _ = shift_north(&mut matrix);

    // shift east
    matrix = rotate_cw(matrix);
    _ = shift_north(&mut matrix);

    matrix = rotate_cw(matrix);
    matrix
}

fn find_cycle(mut matrix: Vec<Vec<char>>) -> (Vec<Vec<char>>, (u64, u64)) {
    let mut cache: HashMap<Vec<Vec<char>>, u64> = HashMap::new();
    for cycle in 0..1000000000 {
        if cache.contains_key(&matrix) {
            let start = *cache.get(&matrix).unwrap();
            return (matrix, (start, cycle));
        }
        cache.insert(matrix.clone(), cycle);
        matrix = spin_once(matrix);
    }
    unreachable!();
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let matrix = get_matrix(input);
    let matrix = rotate_ccw(matrix);
    let (mut matrix, (start, end)) = find_cycle(matrix);
    let remaining_cycles = (1000_000_000 - start) % (end - start);
    for _ in 0..remaining_cycles {
        matrix = spin_once(matrix);
    }
    let total_load = get_north_load(&matrix);
    total_load
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::year2023::day14::{
        find_cycle, get_matrix, get_north_load, rotate_ccw, shift_north, spin_once,
    };

    use super::rotate_cw;

    fn get_string_representation(matrix: Vec<Vec<char>>) -> String {
        let matrix = rotate_cw(matrix);
        matrix
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n")
    }

    #[test]
    fn calculate_total_load_correctly() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let matrix = get_matrix(input);
        let mut rotated = rotate_ccw(matrix);
        shift_north(&mut rotated);
        let total_load: u64 = get_north_load(&rotated);
        let transposed = rotate_cw(rotated);
        let shifted = transposed
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n");
        println!("{shifted}");
        assert_eq!(total_load, 136);
    }

    #[test]
    fn rotation_works() {
        let input = "123
456
789";
        let matrix = get_matrix(input);
        let rotated = rotate_ccw(matrix.clone())
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n");
        let expected = "369
258
147";
        assert_eq!(rotated, expected);

        let rotated = rotate_cw(matrix.clone())
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .join("\n");
        let expected = "741
852
963";
        assert_eq!(rotated, expected);
    }

    #[test]
    fn spinning_works() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let mut matrix = get_matrix(input);

        matrix = rotate_ccw(matrix);

        matrix = spin_once(matrix);
        let s = get_string_representation(matrix.clone());
        println!("{s}\n");
        assert_eq!(
            s,
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );

        matrix = spin_once(matrix);
        let s = get_string_representation(matrix.clone());
        println!("{s}\n");
        assert_eq!(
            s,
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
        );

        matrix = spin_once(matrix);
        let s = get_string_representation(matrix.clone());
        println!("{s}\n");
        assert_eq!(
            s,
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }

    #[test]
    fn spin_a_billion_times_works() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let matrix = get_matrix(input);
        let matrix = rotate_ccw(matrix);
        let (mut matrix, (start, end)) = find_cycle(matrix);
        println!("cycle found: [{start}, {end}]");
        let remaining_cycles = (1_000_000_000 - start) % (end - start);
        println!("remaining:   {remaining_cycles}");
        for _ in 0..remaining_cycles {
            matrix = spin_once(matrix);
        }
        let s = get_string_representation(matrix.clone());
        println!("final:\n{s}");
        let total_load = get_north_load(&matrix);
        assert_eq!(total_load, 64);
    }
}
