use std::panic::RefUnwindSafe;

use regex::Regex;

pub fn solve_a() {
    let input = aoc_utils::get_input(2023, 02);
    println!("{}", input);
    let ids = get_ids_of_valid_games(&input);
}

pub fn solve_b() {
    let input = aoc_utils::get_input(2023, 02);
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
    println!("{}", id);
    let rounds: Vec<Round> = parts[1].trim().split(";").map(|r| parse_round(r)).collect();
    for round in &rounds {
        println!(
            "round: {},{},{}",
            round.red_count, round.green_count, round.blue_count
        );
    }

    return Game { id, rounds };
}

fn parse_round(r: &str) -> Round {
    println!("{}", r);
    let re_blue = Regex::new(r".*(\d+) blue.*").unwrap();
    let re_green = Regex::new(r".*(\d+) green.*").unwrap();
    let re_red = Regex::new(r".*(\d+) red.*").unwrap();
    let reds = re_blue
        .find(r)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let greens = re_green
        .find(r)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());
    let blues = re_red
        .find(r)
        .map_or(0, |m| m.as_str().parse::<u32>().unwrap());

    Round {
        red_count: reds,
        green_count: greens,
        blue_count: blues,
    }
}

fn is_possible(game: &Game) -> bool {
    return true;
}

fn get_ids_of_valid_games(input: &str) {
    // println!("input: {}", input.lines().count());
    let ids: Vec<u32> = input
        .lines()
        .map(|line| -> u32 {
            let game = parse_game(line);
            if is_possible(&game) {
                return game.id;
            } else {
                return 0;
            };
        })
        .collect();
}
