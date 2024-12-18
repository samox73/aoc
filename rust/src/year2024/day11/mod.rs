use std::collections::LinkedList;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 11);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2024, 11);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn get_new_values(v: u64) -> Vec<u64> {
    Vec::new()
}

fn blink(stones: &mut LinkedList<u64>) {
    let mut c = stones.cursor_front_mut();
    loop {
        //if *node == 0 {
        //    *node = 1;
        //} else if (*node).to_string().len() % 2 == 0 {
        //    println!("even node");
        //    let len = (*node).to_string().len();
        //    c.insert_after((*node).to_string()[len / 2..].parse().unwrap());
        //    *node = (*node).to_string()[..len / 2].parse().unwrap();
        //} else {
        //    *node *= 2024;
        //}
        let node = c.current();
        if node.is_none() {
            break;
        }

        let n = get_new_values(*node.unwrap());
        if n.len() > 1 {
            if let Some(v) = node {
                *v = n[0];
            }
            c.insert_after(n[1]);
        }

        c.move_next();
    }
}

fn solve_a(input: &str) -> u64 {
    let solution = 0;
    //let input = "125 17";
    let input = "0";
    let mut stones: LinkedList<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", stones);
    for _ in 0..5 {
        blink(&mut stones);
        println!("{:?}", stones);
    }

    solution
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let solution = 0;
    solution
}
