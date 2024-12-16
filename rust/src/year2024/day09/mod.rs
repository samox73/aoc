use itertools::Itertools;
use num::iter::Range;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 09);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 09);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn get_last_file_index(disk: &Vec<i64>) -> usize {
    for i in (0..disk.len()).rev() {
        if *disk.get(i).unwrap() != -1 {
            return i;
        }
    }
    return 0;
}

fn solve_a(input: &str) -> u64 {
    let input = "2333133121414131402";
    let disks_map = input.trim().chars().collect_vec();
    let mut disk = Vec::new();
    let mut file = true;
    let mut id = 0;
    for b in disks_map {
        let count = b.to_string().parse::<u32>().unwrap();
        if file {
            for _ in 0..count {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..count {
                disk.push(-1);
            }
        }
        file = !file;
    }

    let l = (&disk).len();
    for i in 0..l {
        if disk[i] == -1 {
            let j = get_last_file_index(&disk);
            if j > i {
                disk.swap(i, j);
            }
        }
    }
    let s: String = disk.iter().map(|i| i.to_string()).collect();
    let solution = disk
        .iter()
        .filter(|&&c| c >= 0)
        .enumerate()
        .map(|(i, &c)| i as u64 * c as u64)
        .sum::<u64>()
        .into();
    solution
}

#[derive(Debug, Clone)]
struct Block {
    pos: u64,
    len: u64,
    is_free: bool,
    id: u64,
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let mut solution = 0;
    let disks_map = input.trim().chars().collect_vec();
    let mut free = Vec::new();
    let mut files = Vec::new();
    let mut is_free = false;
    let mut id = 0;
    let mut pos = 0;
    for b in disks_map {
        let len = b.to_string().parse::<u64>().unwrap();
        let block = Block {
            pos,
            len,
            is_free,
            id,
        };
        if !is_free {
            files.push(block);
            id += 1;
        } else {
            free.push(block);
        }
        is_free = !is_free;
        pos += len;
    }
    println!("{}", to_string(&files));

    for i in (0..files.len()).rev() {
        let f = files.get_mut(i).unwrap();
        match free.iter_mut().filter(|b| b.len >= f.len).next() {
            Some(b) => {
                if f.pos > b.pos {
                    f.pos = b.pos;
                    b.pos += f.len;
                    b.len -= f.len;
                }
            }
            None => (),
        }
    }

    for f in files.iter() {
        for i in f.pos..f.pos + f.len {
            solution += i * f.id;
        }
    }

    solution
}

fn to_string(files: &Vec<Block>) -> String {
    let last = files
        .iter()
        .sorted_by(|a, b| a.pos.cmp(&b.pos))
        .rev()
        .next()
        .unwrap();
    let length = last.pos + last.len;
    let mut s = ".".repeat(length as usize);
    for b in files {
        for i in b.pos..b.pos + b.len {
            s.replace_range(i as usize..=i as usize, &b.id.to_string());
        }
    }
    s
}
