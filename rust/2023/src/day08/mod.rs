use nom::{
    bytes::complete::{take_till, take_until},
    character::{
        complete::{alpha1, anychar, multispace1},
        is_alphabetic,
    },
    multi::many1,
    IResult,
};
use num::Integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 08);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aoc_utils::get_input(2023, 08);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let Ok((input, instructions)) = build_instructions(input) else {
        todo!()
    };
    let Ok((_, lookup)) = build_lookup_vector(input) else {
        todo!()
    };
    let solution = count_steps_a(instructions, lookup);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let Ok((input, instructions)) = build_instructions(input) else {
        todo!()
    };
    let Ok((_, lookup)) = build_lookup_vector(input) else {
        todo!()
    };
    let solution = count_steps_b(instructions, lookup);
    println!("part b: {}", solution);
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

fn count_steps_a(instructions: Vec<Instruction>, lookup: Vec<(usize, (usize, usize))>) -> u64 {
    let mut count = 0;
    let mut node = 0usize;
    let max = get_int_from_node("ZZZ");
    while node != max {
        for instruction in &instructions {
            let tuple = lookup.get(node).unwrap();
            node = if *instruction == Instruction::Left {
                tuple.1 .0
            } else {
                tuple.1 .1
            };
            count += 1;
            if node == max {
                return count;
            }
        }
    }
    return count;
}

fn get_starting_nodes(lookup: &Vec<(usize, (usize, usize))>) -> Vec<&(usize, (usize, usize))> {
    lookup
        .par_iter()
        .filter(|c| c.1 .0 != 0 && c.1 .1 != 0)
        .filter(|c| c.0 % 26 == 0)
        .collect::<Vec<&(usize, (usize, usize))>>()
}

fn get_abc_value(mut i: usize) -> String {
    let mut result = String::new();
    while i > 0 {
        let remainder = (i % 26) as u8;
        let digit = (b'A' + remainder) as char;
        result.insert(0, digit);
        i /= 26;
    }
    for _ in 0..3 - result.len() {
        result.insert(0, 'A');
    }
    result
}

fn go_step(
    instruction: &Instruction,
    lookup: &Vec<(usize, (usize, usize))>,
    node: &(usize, (usize, usize)),
) -> (usize, (usize, usize)) {
    let node = if *instruction == Instruction::Left {
        &lookup[node.1 .0]
    } else {
        &lookup[node.1 .1]
    };
    *node
}

fn find_route(
    instructions: &Vec<Instruction>,
    lookup: &Vec<(usize, (usize, usize))>,
    mut node: (usize, (usize, usize)),
) -> (usize, usize) {
    let mut loc = 0;
    while node.0 % 26 != 25 {
        node = go_step(&instructions[loc % instructions.len()], lookup, &node);
        loc += 1;
    }
    let anchor = node;
    let mut period = 0;
    node = go_step(&instructions[period % instructions.len()], lookup, &node);
    period += 1;
    while node != anchor {
        node = go_step(
            &instructions[(loc + period) % instructions.len()],
            lookup,
            &node,
        );
        period += 1;
        if node.0 % 26 == 25 && node != anchor {
            println!("found {} while at {}", get_abc_value(node.0), loc + period);
        }
    }
    return (loc, period);
}

fn count_steps_b(instructions: Vec<Instruction>, lookup: Vec<(usize, (usize, usize))>) -> usize {
    let mut locations: Vec<usize> = Vec::new();
    let mut periods: Vec<usize> = Vec::new();
    for (i, node) in get_starting_nodes(&lookup).iter().enumerate() {
        let (location, period) = find_route(&instructions, &lookup, **node);
        locations.push(location);
        periods.push(period);
        println!("node #{i}: location: {location}, period: {period}");
    }
    // apparently after testing this, the period of each cycle is equal to its offset from 0,
    // so we can just use the lcm of all the periods... smh
    let mut count = 1usize;
    for period in periods {
        count = count.lcm(&period);
    }
    return count;
}

fn build_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, line) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (_, instructions) = many1(|d| -> IResult<&str, Instruction> {
        let (d, char) = anychar(d)?;
        if char == 'L' {
            return Ok((d, Instruction::Left));
        } else {
            return Ok((d, Instruction::Right));
        }
    })(line)?;
    return Ok((input, instructions));
}

fn get_int_from_node(node: &str) -> usize {
    let digits = node
        .chars()
        .map(|c| (c.to_digit(36).unwrap() - 10) as usize);
    let mut value = 0;
    for (i, d) in digits.enumerate() {
        value += d * 26usize.pow((node.len() - i - 1) as u32);
    }
    return value;
}

fn build_lookup_vector(input: &str) -> IResult<&str, Vec<(usize, (usize, usize))>> {
    let max_size = get_int_from_node("ZZZ") + 1;
    let mut network: Vec<(usize, (usize, usize))> = Vec::with_capacity(max_size);
    unsafe {
        network.set_len(max_size);
    }
    let (_, _) = many1(|d| -> IResult<&str, (usize, usize)> {
        let (d, node) = take_until(" ")(d)?;
        let (d, _) = take_till(|c| is_alphabetic(c as u8))(d)?;
        let (d, left) = take_until(",")(d)?;
        let (d, _) = take_till(|c| is_alphabetic(c as u8))(d)?;
        let (d, right) = take_until(")")(d)?;
        let (d, _) = take_till(|c| is_alphabetic(c as u8))(d)?;
        let node = get_int_from_node(node);
        let left = get_int_from_node(left);
        let right = get_int_from_node(right);
        network[node] = (node, (left, right));
        return Ok((d, (0usize, 0usize)));
    })(input)?;
    return Ok((input, network));
}

#[cfg(test)]
mod tests {
    use super::get_int_from_node;

    fn print_value(n: &str) {
        let int = get_int_from_node(n) % 26;
        println!("{n} -> {int}");
    }

    #[test]
    fn test_node_to_int_conversion() {
        print_value("AAA");
        print_value("BBA");
        print_value("BCA");
    }
}
