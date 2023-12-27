#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};

use aocutils::{vec2::Vec2, vec3::Vec3};
use itertools::Itertools;
use nom::{character::complete, IResult};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 22);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 22);
    b.iter(|| solve_b(&input));
}

type HeightMap = HashMap<Vec2<u64>, (u64, usize)>;

fn parse_brick(input: &str) -> IResult<&str, Vec<Vec3<u64>>> {
    let (input, x0) = complete::u64(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, y0) = complete::u64(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, z0) = complete::u64(input)?;
    let (input, _) = complete::char('~')(input)?;
    let (input, x1) = complete::u64(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, y1) = complete::u64(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, z1) = complete::u64(input)?;
    let mut coords = Vec::new();
    for x in x0..=x1 {
        for y in y0..=y1 {
            for z in z0..=z1 {
                coords.push(Vec3::from((x, y, z)));
            }
        }
    }
    Ok((input, coords))
}

fn get_lowest_z(v: &Vec<Vec3<u64>>) -> u64 {
    v.iter().map(|a| a.raw().2).min().unwrap()
}

fn get_highest_z(v: &Vec<Vec3<u64>>) -> u64 {
    v.iter().map(|a| a.raw().2).max().unwrap()
}

fn build_height_map(bricks: &BTreeMap<usize, Vec<Vec3<u64>>>) -> HeightMap {
    let mut map: HeightMap = HashMap::new();
    for (x, y, id) in bricks
        .iter()
        .flat_map(|(id, coords)| coords.iter().map(|v3| (v3.x, v3.y, *id)))
    {
        map.insert(Vec2::from((x, y)), (0, id));
    }
    map
}

fn drop_brick(
    id: usize,
    brick: &mut Vec<Vec3<u64>>,
    height_map: &mut HashMap<Vec2<u64>, (u64, usize)>,
) -> u64 {
    let mut distance = u64::MAX;
    for (x, y, z) in brick.iter().map(|v3| v3.raw()) {
        let (height, _) = height_map[&Vec2::from((x, y))];
        distance = distance.min(z - height - 1);
    }
    for vec in brick.iter_mut() {
        vec.z -= distance;
        height_map.insert(Vec2::from((vec.x, vec.y)), (vec.z, id));
    }
    distance
}

fn print_y_map(coord_to_id: &BTreeMap<Vec3<u64>, usize>) {
    let xmax = coord_to_id.keys().map(|v| v.x).max().unwrap();
    let ymax = coord_to_id.keys().map(|v| v.y).max().unwrap();
    let zmax = coord_to_id.keys().map(|v| v.z).max().unwrap();
    for z in (0..=zmax).rev() {
        for x in 0..=xmax {
            let mut ids = Vec::new();
            for y in 0..=ymax {
                let v = &Vec3::from((x, y, z));
                if let Some(id) = coord_to_id.get(v) {
                    if !ids.contains(id) {
                        ids.push(*id);
                    }
                }
            }
            if z == 0 {
                print!("-");
            } else {
                if ids.len() == 0 {
                    print!(" ");
                } else {
                    if ids.len() > 1 {
                        print!("?");
                    } else {
                        print!("{}", ids[0]);
                    }
                }
            }
        }
        println!();
    }
}

fn print_x_map(coord_to_id: &BTreeMap<Vec3<u64>, usize>) {
    let xmax = coord_to_id.keys().map(|v| v.x).max().unwrap();
    let ymax = coord_to_id.keys().map(|v| v.y).max().unwrap();
    let zmax = coord_to_id.keys().map(|v| v.z).max().unwrap();
    for z in (0..=zmax).rev() {
        for y in 0..=ymax {
            let mut ids = Vec::new();
            for x in 0..=xmax {
                let v = &Vec3::from((x, y, z));
                if let Some(id) = coord_to_id.get(v) {
                    if !ids.contains(id) {
                        ids.push(*id);
                    }
                }
            }
            if z == 0 {
                print!("-");
            } else {
                if ids.len() == 0 {
                    print!(" ");
                } else {
                    if ids.len() > 1 {
                        print!("?");
                    } else {
                        print!("{}", ids[0]);
                    }
                }
            }
        }
        println!();
    }
}

fn drop_bricks(id_to_brick: &mut BTreeMap<usize, Vec<Vec3<u64>>>, ignore: Option<usize>) -> u64 {
    // map for storing the current (highest z, idx) per (x,y)
    let height_map: &mut HeightMap = &mut build_height_map(&id_to_brick);
    let mut count = 0;
    for (&id, brick) in id_to_brick
        .iter_mut()
        .sorted_by(|(_, a), (_, b)| get_lowest_z(a).cmp(&get_lowest_z(b)))
    {
        if let Some(ignore_id) = ignore {
            if ignore_id == id {
                continue;
            }
        }
        if drop_brick(id, brick, height_map) > 0 {
            count += 1;
        }
    }
    count
}

fn get_coord_to_id_map(
    id_to_brick: &BTreeMap<usize, Vec<Vec3<u64>>>,
) -> BTreeMap<Vec3<u64>, usize> {
    let mut coord_to_id: BTreeMap<Vec3<u64>, usize> = BTreeMap::new();
    for (id, brick) in id_to_brick {
        for c in brick {
            coord_to_id.insert(c.clone(), *id);
        }
    }
    coord_to_id
}

fn build_support_ids(
    id_to_brick: &BTreeMap<usize, Vec<Vec3<u64>>>,
    coord_to_id: &BTreeMap<Vec3<u64>, usize>,
) -> HashMap<Vec<Vec3<u64>>, HashSet<usize>> {
    let mut brick_supports: HashMap<Vec<Vec3<u64>>, HashSet<usize>> = HashMap::new();
    for (&id, brick) in id_to_brick
        .iter()
        .sorted_by(|(_, a), (_, b)| get_lowest_z(a).cmp(&get_lowest_z(b)))
        .rev()
    {
        let mut support_ids: HashSet<usize> = HashSet::new();
        for v in brick.iter() {
            let below = &Vec3::from((v.x, v.y, v.z - 1));
            if let Some(&support_id) = coord_to_id.get(below) {
                support_ids.insert(support_id);
            }
        }
        support_ids.remove(&id);
        // println!("{id}: {:?}", support_ids);
        brick_supports.insert(brick.clone(), support_ids);
    }
    brick_supports
}

fn build_supported_ids(
    id_to_brick: &BTreeMap<usize, Vec<Vec3<u64>>>,
    coord_to_id: &BTreeMap<Vec3<u64>, usize>,
) -> HashMap<usize, HashSet<usize>> {
    let mut id_to_supported_ids: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (&id, brick) in id_to_brick
        .iter()
        .sorted_by(|(_, a), (_, b)| get_lowest_z(a).cmp(&get_lowest_z(b)))
        .rev()
    {
        let mut ids_below: HashSet<usize> = HashSet::new();
        let mut ids_above: HashSet<usize> = HashSet::new();
        for v in brick.iter() {
            let below = &Vec3::from((v.x, v.y, v.z - 1));
            if let Some(&id_below) = coord_to_id.get(below) {
                if id_below != id {
                    ids_below.insert(id_below);
                    ids_above = id_to_supported_ids
                        .get(&id)
                        .iter()
                        .cloned()
                        .flatten()
                        .cloned()
                        .collect();
                }
            }
        }
        for id_below in ids_below {
            if !id_to_supported_ids.contains_key(&id_below) {
                id_to_supported_ids.insert(id_below, HashSet::new());
            }
            id_to_supported_ids.get_mut(&id_below).unwrap().insert(id);
            for id_above in ids_above.iter() {
                id_to_supported_ids
                    .get_mut(&id_below)
                    .unwrap()
                    .insert(*id_above);
            }
        }
    }
    id_to_supported_ids
}

pub fn solve_a(input: &str) {
    // idx -> Nodes map
    let mut id_to_brick: BTreeMap<usize, Vec<Vec3<u64>>> = BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        let brick = parse_brick(line).unwrap().1;
        id_to_brick.insert(i, brick);
    }
    drop_bricks(&mut id_to_brick, None);

    let coord_to_id = get_coord_to_id_map(&id_to_brick);
    // get a map of [brick] -> ids of bricks directly under [brick]
    let brick_to_support_ids = build_support_ids(&id_to_brick, &coord_to_id);
    let non_disintegratable_count = brick_to_support_ids
        .values()
        .filter(|v| v.len() == 1)
        .flatten()
        .unique()
        .count();
    println!("part a: {}", id_to_brick.len() - non_disintegratable_count);
}

fn sort_by_z(input: &str) -> String {
    input
        .lines()
        .sorted_by(|a, b| {
            let a = parse_brick(a).unwrap();
            let b = parse_brick(b).unwrap();
            let a = a.1.iter().map(|v| v.z).min().unwrap();
            let b = b.1.iter().map(|v| v.z).min().unwrap();
            a.cmp(&b)
        })
        .join("\n")
}

pub fn solve_b(input: &str) {
    let input = sort_by_z(input);
    println!("{input}");
    // idx -> Nodes map
    let mut id_to_brick: BTreeMap<usize, Vec<Vec3<u64>>> = BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        let brick = parse_brick(line).unwrap().1;
        id_to_brick.insert(i, brick);
    }
    drop_bricks(&mut id_to_brick, None);

    let coord_to_id = get_coord_to_id_map(&id_to_brick);
    let brick_to_support_ids = build_support_ids(&id_to_brick, &coord_to_id);
    let non_disintegratables: HashSet<usize> = brick_to_support_ids
        .values()
        .filter(|v| v.len() == 1)
        .flatten()
        .cloned()
        .unique()
        .sorted()
        .collect();
    // get a map of [brick] -> ids of bricks directly/indirectly above [brick]
    // let id_to_supported_ids = build_supported_ids(&id_to_brick, &coord_to_id);
    // let count: usize = non_disintegratables
    //     .iter()
    //     .sorted()
    //     .map(|id| {
    //         let c = id_to_supported_ids.get(id).unwrap_or(&HashSet::new()).len();
    //         // println!("{id}: {c}");
    //         c
    //     })
    //     .sum();

    let mut count = 0;
    for id in non_disintegratables {
        let itb = &mut id_to_brick.clone();
        let c = drop_bricks(itb, Some(id));
        println!("{id} -> {c}");
        count += c;
    }
    println!("part b: {}", count);
}
