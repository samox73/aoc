use std::{char, collections::HashMap};

use itertools::Itertools;
use nom::IResult;

extern crate test;

#[bench]
pub fn bench_a(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 19);
    b.iter(|| solve_a(&input));
}

#[bench]
pub fn bench_b(b: &mut test::Bencher) {
    let input = crate::utils::input::get(2023, 19);
    b.iter(|| solve_b(&input));
}

#[allow(dead_code)]
fn solve_b(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);

    // build a tree of ranges for every outcome
    let mut stack = vec![((1, 4000), (1, 4000), (1, 4000), (1, 4000), "in", 0)];
    let mut accepted_ranges = vec![];
    while let Some(current) = stack.pop() {
        let (x, m, a, s, workflow_key, rule_index) = current;
        if workflow_key == "A" {
            accepted_ranges.push((x, m, a, s));
            continue;
        } else if workflow_key == "R" {
            continue;
        }

        let rules = workflows.get(workflow_key).unwrap();
        let rule = &rules[rule_index];
        match rule {
            Rule::Accept => {
                accepted_ranges.push((x, m, a, s));
                continue;
            }
            Rule::Reject => {
                continue;
            }
            Rule::Goto(new_wf_key) => {
                stack.push((x, m, a, s, new_wf_key, 0));
                continue;
            }
            Rule::GT(prop, val, to) => match prop.as_str() {
                "x" => {
                    stack.push(((val + 1, x.1), m, a, s, to.as_str(), 0));
                    stack.push(((x.0, *val), m, a, s, workflow_key, rule_index + 1));
                }
                "m" => {
                    stack.push((x, (val + 1, m.1), a, s, to.as_str(), 0));
                    stack.push((x, (m.0, *val), a, s, workflow_key, rule_index + 1));
                }
                "a" => {
                    stack.push((x, m, (val + 1, a.1), s, to.as_str(), 0));
                    stack.push((x, m, (a.0, *val), s, workflow_key, rule_index + 1));
                }
                "s" => {
                    stack.push((x, m, a, (val + 1, s.1), to.as_str(), 0));
                    stack.push((x, m, a, (s.0, *val), workflow_key, rule_index + 1));
                }
                _ => {
                    panic!("unknown prop {}", prop)
                }
            },
            Rule::LT(prop, val, to) => match prop.as_str() {
                "x" => {
                    stack.push(((x.0, val - 1), m, a, s, to.as_str(), 0));
                    stack.push(((*val, x.1), m, a, s, workflow_key, rule_index + 1));
                }
                "m" => {
                    stack.push((x, (m.0, val - 1), a, s, to.as_str(), 0));
                    stack.push((x, (*val, m.1), a, s, workflow_key, rule_index + 1));
                }
                "a" => {
                    stack.push((x, m, (a.0, val - 1), s, to.as_str(), 0));
                    stack.push((x, m, (*val, a.1), s, workflow_key, rule_index + 1));
                }
                "s" => {
                    stack.push((x, m, a, (s.0, val - 1), to.as_str(), 0));
                    stack.push((x, m, a, (*val, s.1), workflow_key, rule_index + 1));
                }
                _ => {
                    panic!("unknown prop {}", prop)
                }
            },
        }
    }

    let sum = accepted_ranges
        .iter()
        .map(|(x, m, a, s)| (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1))
        .sum();
    println!("part a: {}", sum);
    sum
}

#[allow(dead_code)]
pub fn solve(input: &str) {
    println!("part a: {}", solve_a(input));
    println!("part b: {}", solve_b(input));
}

fn solve_a(input: &str) -> u64 {
    let (workflows, items) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let parts: Vec<Part> = items
        .lines()
        .map(|l| parse_part(l).unwrap().1)
        .collect_vec();
    let mut sum = 0;
    for part in parts {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            let rule = process_part(&part, workflow);
            match rule {
                Rule::Accept => {
                    sum += part.values().sum::<u64>();
                    break;
                }
                Rule::Reject => break,
                Rule::Goto(ref destination) => workflow = workflows.get(destination).unwrap(),
                _ => unreachable!(),
            }
        }
    }
    println!("part a: {}", sum);
    sum
}

fn process_part(part: &Part, rules: &Vec<Rule>) -> Rule {
    for rule in rules {
        match rule {
            Rule::Accept | Rule::Reject | Rule::Goto(_) => return rule.clone(),
            Rule::LT(char, value, destination) => {
                if part.get(&char.chars().next().unwrap()).unwrap() < value {
                    match destination.as_str() {
                        "A" => return Rule::Accept,
                        "R" => return Rule::Reject,
                        _ => return Rule::Goto(destination.clone()),
                    };
                }
            }
            Rule::GT(char, value, destination) => {
                if part.get(&char.chars().next().unwrap()).unwrap() > value {
                    match destination.as_str() {
                        "A" => return Rule::Accept,
                        "R" => return Rule::Reject,
                        _ => return Rule::Goto(destination.clone()),
                    }
                }
            }
        }
    }
    unreachable!()
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let mut item: Part = HashMap::new();
    let characteristics = input.trim_matches('{').trim_matches('}').split(",");
    for c in characteristics {
        let (c, key) = nom::character::complete::anychar(c)?;
        let (c, _) = nom::character::complete::char('=')(c)?;
        let (_, value) = nom::character::complete::u64(c)?;
        item.insert(key, value);
    }
    Ok((input, item))
}

fn parse_workflows(input: &str) -> Workflows {
    let mut workflows = HashMap::new();
    let re = regex::Regex::new(r"^(.+)\{(.+)\}").unwrap();
    for line in input.lines() {
        let mut rules: Vec<Rule> = Vec::new();
        let captures = re.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str().to_string();
        for rule in captures.get(2).unwrap().as_str().split(",") {
            rules.push(parse_rule(rule));
        }
        workflows.insert(name, rules);
    }
    workflows
}

fn parse_rule(input: &str) -> Rule {
    return match input {
        "A" => Rule::Accept,
        "R" => Rule::Reject,
        _ => parse_rule_non_trivial(input),
    };
}

fn parse_rule_non_trivial(input: &str) -> Rule {
    return if input.contains(":") {
        let (condition, destination) = input.split_once(":").unwrap();
        if condition.contains("<") {
            let (category, value) = condition.split_once("<").unwrap();
            return Rule::LT(
                category.to_string(),
                value.parse().unwrap(),
                destination.to_string(),
            );
        } else if condition.contains(">") {
            let (category, value) = condition.split_once(">").unwrap();
            return Rule::GT(
                category.to_string(),
                value.parse().unwrap(),
                destination.to_string(),
            );
        }
        unreachable!();
    } else {
        Rule::Goto(input.to_string())
    };
}

#[derive(PartialEq, Eq, Clone)]
enum Rule {
    GT(String, u64, String),
    LT(String, u64, String),
    Accept,
    Reject,
    Goto(String),
}

type Workflows = HashMap<String, Vec<Rule>>;
type Part = HashMap<char, u64>;

#[cfg(test)]
mod tests {
    use super::solve_a;

    #[test]
    fn process_part_works() {
        let input = "in{a>1:A,m>1:R,A}

{x=1,m=2,a=3,s=4}
";
        let solution = solve_a(input);
        assert_eq!(solution, 10);
    }
}
