#![allow(unused)]
use std::{collections::HashMap, hash::Hash};

use crate::util::DaySolver;

type Rock = char;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    pub fn roll_north(&mut self) {
        for y in 0..self.rocks.len() {
            for x in 0..self.rocks[0].len() {
                if self.rocks[y][x] == 'O' {
                    self.rocks[y][x] = '.';
                    let mut yy = y;
                    while yy > 0 && self.rocks[yy - 1][x] == '.' {
                        yy -= 1;
                    }
                    self.rocks[yy][x] = 'O';
                }
            }
        }
    }
    pub fn roll_south(&mut self) {
        for y in (0..self.rocks.len()).rev() {
            for x in 0..self.rocks[0].len() {
                if self.rocks[y][x] == 'O' {
                    self.rocks[y][x] = '.';
                    let mut yy = y;
                    while yy < self.rocks.len() - 1 && self.rocks[yy + 1][x] == '.' {
                        yy += 1;
                    }
                    self.rocks[yy][x] = 'O';
                }
            }
        }
    }
    pub fn roll_east(&mut self) {
        for x in (0..self.rocks[0].len()).rev() {
            for y in 0..self.rocks.len() {
                if self.rocks[y][x] == 'O' {
                    self.rocks[y][x] = '.';
                    let mut xx = x;
                    while xx < self.rocks[0].len() - 1 && self.rocks[y][xx + 1] == '.' {
                        xx += 1;
                    }
                    self.rocks[y][xx] = 'O';
                }
            }
        }
    }
    pub fn roll_west(&mut self) {
        for x in 0..self.rocks[0].len() {
            for y in 0..self.rocks.len() {
                if self.rocks[y][x] == 'O' {
                    self.rocks[y][x] = '.';
                    let mut xx = x;
                    while xx > 0 && self.rocks[y][xx - 1] == '.' {
                        xx -= 1;
                    }
                    self.rocks[y][xx] = 'O';
                }
            }
        }
    }
    pub fn spin(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }
    pub fn weight(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, row)| (self.rocks.len() - y) * row.iter().filter(|c| c == &&'O').count())
            .sum()
    }
    fn print(&self) {
        self.rocks.iter().for_each(|row| {
            row.iter().for_each(|c| print!("{}", c));
            println!();
        });
    }
}

pub struct Day14();

impl Day14 {
    fn parse(input: &[String]) -> Platform {
        Platform {
            rocks: input.iter().map(|line| line.chars().collect()).collect(),
        }
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day14 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut platform = Self::parse(&input);
        platform.roll_north();
        Some(platform.weight())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut platform = Self::parse(&input);
        let mut record = HashMap::<Platform, u64>::new();
        let mut start: u64 = 0;
        let mut cycle: u64 = 0;
        for i in 1.. {
            platform.spin();
            if record.contains_key(&platform) {
                start = i;
                break;
            }
            record.insert(platform.clone(), i);
        }
        let mut origin = platform.clone();
        for i in 1.. {
            platform.spin();
            if origin == platform {
                cycle = i;
                break;
            }
        }
        let x = (1000000000_u64 - start) % cycle;
        for _ in 0..x {
            origin.spin();
        }
        Some(origin.weight())
    }
}
