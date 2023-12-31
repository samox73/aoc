use std::collections::{HashMap, VecDeque};

use nom::character::complete::{self, alpha1};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 15);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 15);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let solution: u64 = input.trim().split(",").map(|step| hash(step)).sum();
    println!("part a: {}", solution);
    solution
}

fn hash(word: &str) -> u64 {
    let mut value: u64 = 0;
    for char in word.chars().into_iter() {
        value += char as u64;
        value *= 17;
        value %= 256;
    }
    value
}

fn remove_label(list: &mut VecDeque<(&str, u64)>, label: &str) {
    for (i, (l, _)) in list.into_iter().enumerate() {
        if label == *l {
            list.remove(i);
            return;
        }
    }
}

fn process<'a>(
    map: &mut HashMap<u64, VecDeque<(&'a str, u64)>>,
    (label, instruction): (&'a str, &str),
) {
    let hash = hash(label);
    if instruction == "-" {
        if map.contains_key(&hash) {
            remove_label(map.get_mut(&hash).unwrap(), label);
        }
    } else {
        let focal_length = complete::u64::<&str, ()>(&instruction[1..]).unwrap().1;
        if !map.contains_key(&hash) {
            map.insert(hash, VecDeque::new());
        }
        let list = map.get_mut(&hash).unwrap();
        if let Some(l) = list.iter_mut().find(|(l, _)| *l == label) {
            l.1 = focal_length;
        } else {
            list.push_back((label, focal_length));
        }
    }
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut map: HashMap<u64, VecDeque<(&str, u64)>> = HashMap::new();
    input
        .trim()
        .split(",")
        .map(|step| alpha1::<&str, ()>(step).unwrap())
        .for_each(|(instruction, label)| process(&mut map, (label, instruction)));

    for (hash, list) in map.iter() {
        println!("box {hash}: {:?}", list);
    }
    let solution = map
        .iter()
        .map(|(hash, list)| {
            let inner = list
                .iter()
                .enumerate()
                .map(|(i, (_, f))| (i + 1) as u64 * f)
                .sum::<u64>();
            (hash + 1) * inner
        })
        .sum();
    println!("part b: {}", solution);
    solution
}

#[cfg(test)]
mod tests {
    use crate::year2023::day15::{hash, solve_b};

    use super::solve_a;

    #[test]
    fn hash_works() {
        let hash = hash("HASH");
        assert_eq!(hash, 52);
    }

    #[test]
    fn solution_works() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let solution = solve_a(input);
        assert_eq!(solution, 1320);
    }

    #[test]
    fn boxes_work() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let solution = solve_b(input);
        assert_eq!(solution, 145);
    }
}
