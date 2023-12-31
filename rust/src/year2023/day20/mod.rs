use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use itertools::Itertools;
use num_integer::Integer;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 20);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 20);
    b.iter(|| solve_b(&input));
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    #[default]
    Low,
    High,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Pulse::High => write!(f, "high"),
            Pulse::Low => write!(f, "low"),
        };
    }
}

impl std::ops::Not for Pulse {
    type Output = Pulse;
    fn not(self) -> Self::Output {
        return match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        };
    }
}

trait Module {
    fn tick(&mut self, pulse: &Pulse, source: &String) -> (Pulse, &Vec<String>);
    fn get_state_string(&self) -> String;
    fn get_destinations(&self) -> &Vec<String>;
    fn add_input(&mut self, _: String) {}
    fn abort(&self, _: &Pulse) -> bool {
        false
    }
}

struct FlipFlop {
    state: Pulse,
    destinations: Vec<String>,
}

impl Module for FlipFlop {
    fn tick(&mut self, p: &Pulse, _: &String) -> (Pulse, &Vec<String>) {
        match (self.state, p) {
            (Pulse::Low, Pulse::Low) => {
                self.state = Pulse::High;
                return (self.state, &self.destinations);
            }
            (Pulse::High, Pulse::Low) => {
                self.state = Pulse::Low;
                return (self.state, &self.destinations);
            }
            (_, _) => return (p.clone(), &self.destinations),
        }
    }

    fn get_state_string(&self) -> String {
        self.state.to_string()
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn abort(&self, pulse: &Pulse) -> bool {
        *pulse == Pulse::High
    }
}

#[derive(Default)]
struct Conjunction {
    state: HashMap<String, Pulse>,
    destinations: Vec<String>,
}

impl Module for Conjunction {
    fn tick(&mut self, pulse: &Pulse, source: &String) -> (Pulse, &Vec<String>) {
        self.state.insert(source.clone(), *pulse);
        if self.state.values().all(|&p| p == Pulse::High) {
            return (Pulse::Low, &self.destinations);
        }
        return (Pulse::High, &self.destinations);
    }
    fn get_state_string(&self) -> String {
        self.state
            .iter()
            .map(|(key, val)| "(".to_string() + key + ": " + &val.to_string() + ")")
            .join(", ")
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn add_input(&mut self, input: String) {
        self.state.insert(input, Pulse::Low);
    }
}

#[derive(Default)]
struct Broadcaster {
    destinations: Vec<String>,
}

impl Module for Broadcaster {
    fn tick(&mut self, pulse: &Pulse, _: &String) -> (Pulse, &Vec<String>) {
        (pulse.clone(), &self.destinations)
    }
    fn get_state_string(&self) -> String {
        "".to_string()
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }
}

fn parse_module(input: &str) -> (String, Box<dyn Module>) {
    let (source, destinations) = input.split_once(" -> ").unwrap();
    let (char, name) = source.split_at(1);
    let destinations = destinations.split(", ").map(str::to_string).collect_vec();
    match char {
        "%" => {
            return (
                name.to_string(),
                Box::new(FlipFlop {
                    state: Pulse::default(),
                    destinations,
                }),
            )
        }
        "&" => {
            return (
                name.to_string(),
                Box::new(Conjunction {
                    state: HashMap::new(),
                    destinations,
                }),
            )
        }
        "b" => {
            return (
                char.to_string() + name,
                Box::new(Broadcaster { destinations }),
            )
        }
        _ => unreachable!(),
    }
}

fn get_cycles(input: &str) -> HashMap<String, u64> {
    let modules = init_modules(input);
    // find the subgraphs modules that lead to rx
    let last = modules
        .iter()
        .filter(|(_, module)| module.get_destinations().contains(&"rx".to_string()))
        .next()
        .unwrap();
    let cycles = modules
        .iter()
        .filter(|(_, module)| module.get_destinations().contains(last.0))
        .map(|(name, _)| (name.clone(), 0))
        .collect::<HashMap<String, u64>>();
    cycles
}

fn init_conjunction_modules(modules: &mut HashMap<String, Box<dyn Module>>) {
    let mut conjunction_modules_inputs: HashMap<String, Vec<String>> = HashMap::new();
    for (name, module) in modules.iter() {
        for destination in module.get_destinations() {
            if let Some(dest) = conjunction_modules_inputs.get_mut(destination) {
                dest.push(name.clone());
            } else {
                conjunction_modules_inputs.insert(destination.clone(), vec![name.clone()]);
            }
        }
    }
    for (name, module) in modules.iter_mut() {
        if let Some(inputs) = conjunction_modules_inputs.get(name) {
            for input in inputs {
                module.add_input(input.clone())
            }
        }
    }
}

fn init_modules(input: &str) -> HashMap<String, Box<dyn Module>> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    for line in input.lines() {
        let (name, module): (String, Box<dyn Module>) = parse_module(line);
        modules.insert(name, module);
    }
    init_conjunction_modules(&mut modules);
    modules
}
#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let mut modules = init_modules(input);
    let mut cycles = get_cycles(input);

    let mut high_count = 0;
    let mut low_count = 0;
    let mut button_count = 0;
    'outer: loop {
        button_count += 1;
        let mut stack: VecDeque<(String, Pulse, String)> =
            VecDeque::from([("broadcaster".to_string(), Pulse::Low, "button".to_string())]);
        while let Some((ref name, input, source)) = stack.pop_front() {
            if button_count <= 1000 {
                match input {
                    Pulse::High => high_count += 1,
                    Pulse::Low => low_count += 1,
                };
            }
            if !modules.contains_key(name) {
                continue;
            }
            let module = modules.get_mut(name).unwrap();
            if module.abort(&input) {
                continue;
            }
            let (output, destinations) = module.tick(&input, &source);
            for destination in destinations {
                if destination == "qb" && cycles.contains_key(name) && output == Pulse::High {
                    println!("{button_count}: {name}");
                    cycles.insert(name.clone(), button_count);
                    if cycles.values().all(|&v| v != 0) {
                        break 'outer;
                    }
                }
                stack.push_back((destination.clone(), output, name.clone()));
            }
        }
    }
    let solution = low_count * high_count;
    println!("part a: {}", solution);
    let lcm = cycles.values().fold(1, |acc, element| acc.lcm(element));
    println!("part b: {}", lcm);
    0
}

#[allow(dead_code)]
fn solve_b(_: &str) -> u64 {
    0
}
