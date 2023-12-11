#![allow(unused)]
use crate::util::DaySolver;

type Galaxy = (u64, u64);

#[derive(Debug)]
pub struct Observation {
    length: usize,
    galaxies: Vec<Galaxy>,
}

impl Observation {
    pub fn expand(&mut self, factor: u64) {
        // TODO this can be removed for the first call
        self.galaxies.sort_by_key(|(x, y)| *y);
        let mut last_y = 0;
        let mut y_expand = 0;
        for i in 0..self.galaxies.len() {
            let y = self.galaxies[i].1;
            if y > last_y + 1 {
                y_expand += (y - last_y - 1) * factor;
            }
            last_y = y;
            self.galaxies[i].1 += y_expand;
        }
        self.galaxies.sort_by_key(|(x, y)| *x);
        let mut last_x = 0;
        let mut x_expand = 0;
        for i in 0..self.galaxies.len() {
            let x = self.galaxies[i].0;
            if x > last_x + 1 {
                x_expand += (x - last_x - 1) * factor;
            }
            last_x = x;
            self.galaxies[i].0 += x_expand;
        }
    }
}

pub struct Day11();

impl Day11 {
    fn parse(input: &[String]) -> Observation {
        let galaxies: Vec<Galaxy> = input
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| (x as u64, y as u64))
            })
            .collect();
        Observation {
            length: input.len(),
            galaxies,
        }
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day11 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut obs = Self::parse(&input);
        obs.expand(1);
        Some(
            obs.galaxies
                .iter()
                .enumerate()
                .map(|(i, (x1, y1))| {
                    obs.galaxies[i + 1..]
                        .iter()
                        .map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
                        .sum::<u64>()
                })
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut obs = Self::parse(&input);
        obs.expand(999999);
        Some(
            obs.galaxies
                .iter()
                .enumerate()
                .map(|(i, (x1, y1))| {
                    obs.galaxies[i + 1..]
                        .iter()
                        .map(|(x2, y2)| x1.abs_diff(*x2) + y1.abs_diff(*y2))
                        .sum::<u64>()
                })
                .sum(),
        )
    }
}
