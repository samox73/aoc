use regex::Regex;

pub fn solve_a() {
    let input = aoc_utils::get_input(2023, 03);
    let parts = parse_parts(&input);
    let symbols = parse_symbols(&input);
    let sum: u64 = parts
        .iter()
        .filter(|&part| is_part_number(part, &symbols))
        .map(|part| part.id)
        .sum();
    println!("{}", sum);
}

pub fn solve_b() {
    let input = aoc_utils::get_input(2023, 03);
    let parts = parse_parts(&input);
    let mut potential_gears = parse_symbols(&input);
    potential_gears.retain(|s| s.value == '*');
    let sum: u64 = potential_gears
        .iter_mut()
        .map(|potential_gear| {
            let r: u64 = get_gear_ratio(potential_gear, &parts);
            if r > 0 {
                let lines: Vec<&str> = input.lines().collect();
                let start = potential_gear.y - 2;
                let start = start.clamp(0, lines.len());
                let end = potential_gear.y + 1;
                let end = end.clamp(0, lines.len());
                for line in lines[start..end].iter() {
                    println!("{}", line);
                }
            }
            r
        })
        .sum();
    println!("{}", sum);
}

struct Part {
    id: u64,
    xs: Vec<usize>,
    ys: Vec<usize>,
}

struct Symbol {
    value: char,
    x: usize,
    y: usize,
    neighboring_parts_count: usize,
}

fn parse_parts(input: &str) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    let line_length = input.lines().next().unwrap().len() + 1;
    println!("line length: {}", line_length);
    let re = Regex::new(r"\d+").unwrap();
    for cap in re.captures_iter(input) {
        let loc = cap.get(0).unwrap().start();
        let id = cap.get(0).unwrap().as_str();
        let len = cap.get(0).unwrap().as_str().len();
        let id = id.parse::<u64>().unwrap();
        let mut xs: Vec<usize> = Vec::new();
        let mut ys: Vec<usize> = Vec::new();
        for i in 0..len {
            xs.push(loc % line_length + 1 + i);
            ys.push(loc / line_length + 1);
        }
        parts.push(Part { id, xs, ys });
    }
    parts
}

fn parse_symbols(input: &str) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();
    let line_length = input.lines().next().unwrap().len() + 1;
    println!("line length: {}", line_length);
    let re = Regex::new(r"[^\d\.\n]").unwrap();
    for cap in re.captures_iter(input) {
        let loc = cap.get(0).unwrap().start();
        let value = cap.get(0).unwrap().as_str().chars().nth(0).unwrap();
        let x = loc % line_length + 1;
        let y = loc / line_length + 1;
        symbols.push(Symbol {
            value,
            x,
            y,
            neighboring_parts_count: 0,
        });
    }
    symbols
}

fn is_part_number(part: &Part, symbols: &Vec<Symbol>) -> bool {
    for symbol in symbols {
        if is_adjacent(&part, symbol) {
            println!(
                "{} ({},{}) is part number!",
                part.id,
                part.xs.first().unwrap(),
                part.ys.first().unwrap()
            );
            return true;
        }
    }
    return false;
}

fn is_adjacent(part: &Part, symbol: &Symbol) -> bool {
    symbol.x >= *part.xs.first().unwrap() - 1
        && symbol.x <= *part.xs.last().unwrap() + 1
        && symbol.y >= *part.ys.first().unwrap() - 1
        && symbol.y <= *part.ys.last().unwrap() + 1
}

fn get_gear_ratio(symbol: &mut Symbol, parts: &Vec<Part>) -> u64 {
    let mut mul: u64 = 1;
    for part in parts {
        if is_adjacent(&part, &symbol) {
            if symbol.neighboring_parts_count > 1 {
                return 0;
            }
            symbol.neighboring_parts_count += 1;
            mul *= part.id;
        }
    }
    if symbol.neighboring_parts_count != 2 {
        return 0;
    }
    println!("\nx: {}\ny: {}\nr: {}", symbol.x, symbol.y, mul);
    return mul;
}
