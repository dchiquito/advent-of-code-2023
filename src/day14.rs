#![allow(unused)]
use crate::util::DaySolver;

type Rock = char;

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
    pub fn weight(&self) -> usize {
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, row)| (self.rocks.len() - y) * row.iter().filter(|c| c == &&'O').count())
            .sum()
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
        None
    }
}
