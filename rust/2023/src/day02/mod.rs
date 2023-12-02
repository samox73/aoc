use regex::{Captures, Regex};

pub fn solve_a() {
    let input = aoc_utils::get_input(2023, 02);
    let ids = get_ids_of_valid_games(&input);
    let sum: u32 = ids.iter().sum();
    println!("part a: {}", sum);
}

pub fn solve_b() {
    let input = aoc_utils::get_input(2023, 02);
    let ids = get_sum_of_power_of_games(&input);
    let sum: u32 = ids.iter().sum();
    println!("part b: {}", sum);
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

fn parse_game(line: &str) -> Game {
    let parts: Vec<&str> = line.split(":").collect();
    let id = parts[0]
        .strip_prefix("Game ")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let rounds: Vec<Round> = parts[1].trim().split(";").map(|r| parse_round(r)).collect();
    return Game { id, rounds };
}

fn get_color(r: &str, color: &str) -> u32 {
    let re = format!(r".*?(?<{}>\d+) {}.*", color, color);
    let re = Regex::new(&re).unwrap();
    if let Some(c) = re.captures(r) {
        let count = c[color].parse::<u32>().unwrap();
        c[color].parse::<u32>().unwrap()
    } else {
        0
    }
}

fn parse_round(r: &str) -> Round {
    Round {
        red_count: get_color(r, "red"),
        green_count: get_color(r, "green"),
        blue_count: get_color(r, "blue"),
    }
}

fn is_possible(game: &Game) -> bool {
    for round in &game.rounds {
        if round.red_count > 12 || round.green_count > 13 || round.blue_count > 14 {
            return false;
        }
    }
    true
}

fn get_ids_of_valid_games(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| -> u32 {
            let game = parse_game(line);
            if is_possible(&game) {
                return game.id;
            } else {
                return 0;
            };
        })
        .collect()
}

fn get_sum_of_power_of_games(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| -> u32 {
            let game = parse_game(line);
            let round = get_minimum_set(game);
            round.red_count * round.green_count * round.blue_count
        })
        .collect()
}

fn get_minimum_set(game: Game) -> Round {
    let mut round = Round {
        red_count: 0,
        green_count: 0,
        blue_count: 0,
    };
    if let Some(red) = game.rounds.iter().max_by_key(|r| r.red_count) {
        round.red_count = red.red_count;
    };
    if let Some(green) = game.rounds.iter().max_by_key(|r| r.green_count) {
        round.green_count = green.green_count;
    };
    if let Some(blue) = game.rounds.iter().max_by_key(|r| r.blue_count) {
        round.blue_count = blue.blue_count;
    };
    round
}
