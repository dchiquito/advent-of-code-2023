#![allow(unused)]
use std::collections::{HashMap, HashSet};

use crate::util::DaySolver;

#[derive(Clone, Eq, PartialEq)]
pub enum Tile {
    Rock,
    Plot,
}
#[derive(Clone)]
pub struct Field {
    tiles: HashMap<(i32, i32), Tile>,
    width: i32,
    height: i32,
    start: (i32, i32),
}
impl Field {
    pub fn is_plot(&self, x: i32, y: i32) -> bool {
        let x_mod = ((x % self.width) + self.width) % self.width;
        let y_mod = ((y % self.height) + self.height) % self.height;
        self.tiles.get(&(x_mod, y_mod)) == Some(&Tile::Plot)
    }
    pub fn print(&self, even: bool) {
        for y in -self.height..self.height * 2 {
            for x in -self.width..self.width * 2 {
                match self.tiles.get(&(x, y)) {
                    Some(Tile::Rock) => print!("#"),
                    Some(Tile::Plot) => {
                        if even == ((x + y) % 2 == 0) {
                            print!("O");
                        } else {
                            print!(".");
                        }
                    }
                    _ => print!(" "),
                }
            }
            println!();
        }
    }
    pub fn count(&self, steps: i32) -> u64 {
        let mut count = 0;
        let mut to_visit = HashSet::from([self.start]);
        let mut just_visited = HashSet::new();
        for _ in 0..=steps {
            let mut next_visits = HashSet::new();
            for (x, y) in to_visit.iter() {
                let (x, y) = (*x, *y);
                if self.is_plot(x, y) {
                    if (((x + y) % 2) + 2) % 2 == steps % 2 {
                        count += 1;
                    }
                    if !just_visited.contains(&(x + 1, y)) {
                        next_visits.insert((x + 1, y));
                    }
                    if !just_visited.contains(&(x - 1, y)) {
                        next_visits.insert((x - 1, y));
                    }
                    if !just_visited.contains(&(x, y + 1)) {
                        next_visits.insert((x, y + 1));
                    }
                    if !just_visited.contains(&(x, y - 1)) {
                        next_visits.insert((x, y - 1));
                    }
                }
            }
            just_visited = to_visit;
            to_visit = next_visits;
        }
        // field.print(even);
        count
    }
}

pub struct Day21();

impl Day21 {
    pub fn parse(input: &[String]) -> Field {
        let (sy, line) = input
            .iter()
            .enumerate()
            .find(|(_, line)| line.contains('S'))
            .unwrap();
        let sx = line.chars().enumerate().find(|(_, c)| c == &'S').unwrap().0;
        let start = (sx as i32, sy as i32);
        let tiles = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (x as i32, y as i32),
                        match c {
                            'S' | '.' => Tile::Plot,
                            '#' => Tile::Rock,
                            _ => unreachable!(),
                        },
                    )
                })
            })
            .collect();
        Field {
            tiles,
            width: input[0].len() as i32,
            height: input.len() as i32,
            start,
        }
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day21 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut field = Self::parse(&input);
        // These will eventually pass with the test input
        // assert_eq!(field.count(field.start, 6), 16);
        // assert_eq!(field.count(field.start, 10), 50);
        // assert_eq!(field.count(field.start, 50), 1594);
        // assert_eq!(field.count(field.start, 100), 6536);
        // assert_eq!(field.count(field.start, 500), 167004);
        // assert_eq!(field.count(field.start, 1000), 668697);
        // assert_eq!(field.count(field.start, 5000), 16733044);
        Some(field.count(64))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let field = Self::parse(&input);
        let w = field.width;
        let w2 = w / 2;
        let all_steps = 26501365;
        let a = field.count(w2);
        let b = field.count(w + w2);
        let c = field.count(w + w + w2);
        let s = b - a;
        let mut r = c - b;
        let d = r - s;
        let mut res = b;
        for i in 0..((all_steps - w2) / w) - 1 {
            res += r;
            r += d;
        }
        Some(res)
    }
}
