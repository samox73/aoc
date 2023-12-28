use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

use itertools::Itertools;
use num::traits::Pow;
use petgraph::{data::Build, graph::UnGraph};
use rustworkx_core::{connectivity::stoer_wagner_min_cut, Result};

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 25);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = aocutils::get_input(2023, 25);
    b.iter(|| solve_b(&input));
}

fn build_graph(input: &str) -> UnGraph<&str, i32> {
    let mut g = UnGraph::<&str, i32>::new_undirected();
    let nodes: HashSet<&str> = input
        .lines()
        .flat_map(|l| l.split(": ").into_iter().map(|a| a.split(" ")).flatten())
        .collect();
    let nodes = nodes
        .iter()
        .map(|&n| (n, g.add_node(n)))
        .collect::<HashMap<_, _>>();

    for l in input.lines() {
        let (src, dsts) = l.split_once(": ").unwrap();
        let dsts = dsts.split(" ").collect_vec();
        for dst in dsts {
            g.add_edge(nodes[src], nodes[dst], 1);
        }
    }
    g
}

pub fn solve_a(input: &str) -> u64 {
    println!("{}", input);
    let g = build_graph(input);
    let min_cut: Result<_> = stoer_wagner_min_cut(&g, |_| Ok(1));
    let solution = if let Ok(Some((_, cut))) = &min_cut {
        let product = (g.node_count() - cut.len()) * cut.len();
        product as u64
    } else {
        0
    };

    println!("part a: {}", solution);
    solution
}

pub fn solve_b(input: &str) -> u64 {
    let solution = 0;
    println!("part b: {}", solution);
    solution
}
