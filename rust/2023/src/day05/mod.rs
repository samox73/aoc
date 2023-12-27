mod tests;
// mod reddit;

extern crate test;

use std::ops::Range;

use btree_range_map::RangeMap;
use nom::{
    bytes::complete::take_till,
    character::{
        complete::{multispace0, multispace1},
        is_digit,
    },
    multi::many1,
    IResult,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 05);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 05);
    b.iter(|| solve_b(&input));
}
pub fn solve_a(input: &str) {
    let almanac = parse(input).unwrap().1;
    let solution = find_lowest_location(almanac);
    println!("part a: {}", solution);
}

pub fn solve_b(input: &str) {
    let almanac = parse(input).unwrap().1;
    let solution = find_lowest_location_with_ranged_seeds(almanac);
    println!("part b: {}", solution);
}

struct SourceDest {
    source_start: u64,
    destination_start: u64,
}

impl PartialEq for SourceDest {
    fn eq(&self, other: &Self) -> bool {
        (self.source_start, self.destination_start) == (other.source_start, other.destination_start)
    }
}

impl Clone for SourceDest {
    fn clone(&self) -> Self {
        SourceDest {
            source_start: self.source_start,
            destination_start: self.destination_start,
        }
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<RangeMap<u64, SourceDest>>,
}

fn parse(input: &str) -> IResult<&str, Almanac> {
    let (input, seeds) = parse_seeds(input)?;
    let (input, _) = multispace1(input)?;
    let range_maps: Vec<RangeMap<u64, SourceDest>> = input
        .split("\n\n")
        .map(|lines| parse_range_map(lines).unwrap().1)
        .collect();
    return Ok((
        input,
        Almanac {
            seeds: seeds,
            maps: range_maps,
        },
    ));
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
    let (input, seeds) = many1(|s| -> IResult<&str, u64> {
        let (s, _) = multispace0(s)?;
        let (s, seed) = nom::character::complete::u64(s)?;
        return Ok((s, seed));
    })(input)?;
    return Ok((input, seeds));
}

fn parse_range_map(input: &str) -> IResult<&str, RangeMap<u64, SourceDest>> {
    let mut range_map = RangeMap::new();
    let (input, _) = take_till(|c| is_digit(c as u8))(input)?;
    for line in input.lines() {
        let (line, destination_start) = nom::character::complete::u64(line)?;
        let (line, _) = nom::character::complete::multispace0(line)?;
        let (line, source_start) = nom::character::complete::u64(line)?;
        let (line, _) = nom::character::complete::multispace0(line)?;
        let (_, length) = nom::character::complete::u64(line)?;
        let range = source_start..(source_start + length);
        let source_dest = SourceDest {
            source_start,
            destination_start,
        };
        range_map.insert(range, source_dest);
    }

    return Ok((input, range_map));
}

fn get_lowest_maps_range_length(almanac: &Almanac) -> u64 {
    let mut l = u64::MAX;
    for map in &almanac.maps {
        for (k, _) in map.iter() {
            let range_length = k.len() as u64;
            if range_length < l {
                l = range_length;
            }
        }
    }
    l
}

fn find_lowest_location(almanac: Almanac) -> u64 {
    let mut lowest_loc = u64::MAX;
    for seed in &almanac.seeds {
        let location = get_location_from_seed(*seed, &almanac.maps);
        if location < lowest_loc {
            lowest_loc = location;
        }
    }
    return lowest_loc;
}

fn get_location_from_seed(seed: u64, maps: &Vec<RangeMap<u64, SourceDest>>) -> u64 {
    let mut traverser = seed;
    for map in maps {
        if let Some(source_dest) = map.get(traverser) {
            traverser = source_dest.destination_start + traverser - source_dest.source_start;
        }
    }
    return traverser;
}

fn get_min_location_from_seed_range(range: Range<u64>, almanac: &Almanac, step_size: usize) -> u64 {
    range
        .step_by(step_size)
        .collect::<Vec<u64>>()
        .into_par_iter()
        .map(|seed| get_location_from_seed(seed, &almanac.maps))
        .min()
        .unwrap_or(u64::MAX)
}

fn find_lowest_seed_range(almanac: &Almanac, step_size: usize) -> Range<u64> {
    let mut lowest_loc = u64::MAX;
    let mut lowest_range: Range<u64> = 0..0;
    for chunk in almanac.seeds.chunks(2) {
        let seed_range = chunk[0]..chunk[0] + chunk[1];
        let location = get_min_location_from_seed_range(seed_range.clone(), &almanac, step_size);
        if location < lowest_loc {
            lowest_loc = location;
            lowest_range = seed_range;
        }
    }
    return lowest_range;
}

fn find_lowest_seed_range_from(
    seed_ranges: &Vec<Range<u64>>,
    almanac: &Almanac,
    step_size: usize,
) -> Range<u64> {
    let mut lowest_loc = u64::MAX;
    let mut lowest_range: Range<u64> = 0..0;
    for seed_range in seed_ranges {
        let location = get_min_location_from_seed_range(seed_range.clone(), &almanac, step_size);
        if location < lowest_loc {
            lowest_loc = location;
            lowest_range = seed_range.clone();
        }
    }
    return lowest_range;
}

fn find_lowest_location_with_ranged_seeds(almanac: Almanac) -> u64 {
    let min_range_length = get_lowest_maps_range_length(&almanac) as usize;
    let mut step_size = min_range_length;
    let mut current_seed_ranges = get_seed_ranges(&almanac);
    step_size /= 100;
    while step_size >= 1 {
        let range = find_lowest_seed_range_from(&current_seed_ranges, &almanac, step_size);
        step_size /= 100;
        if step_size == 0 {
            step_size = 1;
            break;
        }
        current_seed_ranges = range
            .step_by(step_size)
            .map(|u| u..(u + step_size as u64))
            .collect();
    }
    let range = find_lowest_seed_range_from(&current_seed_ranges, &almanac, step_size);
    return get_min_location_from_seed_range(range, &almanac, 1);
}

fn get_seed_ranges(almanac: &Almanac) -> Vec<Range<u64>> {
    almanac.seeds.chunks(2).map(|c| c[0]..c[0] + c[1]).collect()
}
