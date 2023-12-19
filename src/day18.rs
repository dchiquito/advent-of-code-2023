#![allow(unused)]
use std::collections::HashSet;

use crate::util::DaySolver;

#[derive(Debug, Eq, PartialEq)]
pub enum Dir {
    U,
    D,
    L,
    R,
}
impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Step {
    dir: Dir,
    distance: u64,
}

pub struct Day18();

impl Day18 {
    pub fn parse1(input: &[String]) -> Vec<Step> {
        input
            .iter()
            .map(|line| {
                let dir = Dir::from(&line[..1]);
                let distance = &line[2..4].trim_end().parse().unwrap();
                let color = 0;
                Step {
                    dir,
                    distance: *distance,
                }
            })
            .collect()
    }
    pub fn parse2(input: &[String]) -> Vec<Step> {
        input
            .iter()
            .map(|line| {
                let dir = match &line[line.len() - 2..line.len() - 1].parse().unwrap() {
                    0 => Dir::R,
                    1 => Dir::D,
                    2 => Dir::L,
                    3 => Dir::U,
                    _ => unreachable!(),
                };
                let distance =
                    u64::from_str_radix(&line[line.len() - 7..line.len() - 2], 16).unwrap();
                Step { dir, distance }
            })
            .collect()
    }
    pub fn area(steps: &[Step]) -> i64 {
        let mut x: i64 = 0;
        let mut area: i64 = 1;
        steps.iter().for_each(|step| {
            match step.dir {
                Dir::U => {
                    area += step.distance as i64 * (x + 1);
                }
                Dir::D => {
                    area -= step.distance as i64 * x;
                }
                Dir::L => {
                    x += step.distance as i64;
                    area += step.distance as i64;
                }
                Dir::R => {
                    x -= step.distance as i64;
                }
            };
        });
        area
    }
}

type Solution = i64;
impl DaySolver<Solution> for Day18 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut steps = Self::parse1(&input);
        Some(Self::area(&steps))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut steps = Self::parse2(&input);
        Some(Self::area(&steps))
    }
}
