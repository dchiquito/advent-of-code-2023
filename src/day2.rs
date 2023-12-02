#![allow(unused)]
use crate::util::DaySolver;

type Solution = u32;

struct Reveal {
    r: u32,
    g: u32,
    b: u32,
}

impl From<&str> for Reveal {
    fn from(value: &str) -> Self {
        let mut reveal = Reveal { r: 0, g: 0, b: 0 };
        for pull in value.split(',') {
            let (digits, color) = pull.trim().split_once(' ').unwrap();
            match color {
                "red" => reveal.r = digits.parse().unwrap(),
                "green" => reveal.g = digits.parse().unwrap(),
                "blue" => reveal.b = digits.parse().unwrap(),
                _ => panic!("{color} is not a valid color"),
            }
        }
        reveal
    }
}

struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl From<&String> for Game {
    fn from(value: &String) -> Self {
        let (game_id_str, reveals_str) = value.split_once(':').unwrap();
        let id: u32 = game_id_str[5..].parse().unwrap();
        // println!("{}", reveals_str);
        let reveals = reveals_str.split(';').map(Reveal::from).collect();
        Game { id, reveals }
    }
}

pub struct Day2();

impl Day2 {
    fn parse(input: Vec<String>) -> Vec<Game> {
        input.iter().map(Game::from).collect()
    }
}

impl DaySolver<Solution> for Day2 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let games = Self::parse(input);
        Some(
            games
                .iter()
                .filter(|g| !g.reveals.iter().any(|r| r.r > 12 || r.g > 13 || r.b > 14))
                .map(|g| g.id)
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let games = Self::parse(input);
        Some(
            games
                .iter()
                .map(|g| {
                    let mut max = Reveal { r: 0, g: 0, b: 0 };
                    for r in &g.reveals {
                        max.r = max.r.max(r.r);
                        max.g = max.g.max(r.g);
                        max.b = max.b.max(r.b);
                    }
                    max.r * max.g * max.b
                })
                .sum(),
        )
    }
}
