extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 01);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    return solve_for(input, "00000");
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    return solve_for(input, "000000");
}

fn solve_for(input: &str, prefix: &str) -> u64 {
    let mut solution = 0;
    loop {
        let value = format!("{}{}", input.trim(), solution);
        let digest = md5::compute(&value);
        let hex = format!("{:x}", digest);
        if hex.starts_with(prefix) {
            println!("found the solution: {}", value);
            break;
        }
        solution += 1;
    }
    solution
}
