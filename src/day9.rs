#![allow(unused)]
use crate::util::DaySolver;
use std::iter::once;

pub struct Day9();

impl Day9 {
    pub fn parse(input: &[String]) -> Vec<Vec<i64>> {
        input
            .iter()
            .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
            .collect()
    }

    fn extrapolate_forward(mut items: &mut [i64]) -> i64 {
        if items.iter().all(|i| *i == 0) {
            return 0;
        }
        let new_len = items.len() - 1;
        let last = items[new_len];
        for i in 0..new_len {
            items[i] = items[i + 1] - items[i];
        }
        last + Self::extrapolate_forward(&mut items[..new_len])
    }
    fn extrapolate_back(mut items: &mut [i64]) -> i64 {
        if items.iter().all(|i| *i == 0) {
            return 0;
        }
        let first = items[0];
        let new_len = items.len() - 1;
        for i in 0..new_len {
            items[i] = items[i + 1] - items[i];
        }
        first - Self::extrapolate_back(&mut items[..new_len])
    }
}

type Solution = i64;
impl DaySolver<Solution> for Day9 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut data = Self::parse(&input);
        Some(
            data.iter_mut()
                .map(|mut line| Self::extrapolate_forward(line))
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut data = Self::parse(&input);
        Some(
            data.iter_mut()
                .map(|mut line| Self::extrapolate_back(line))
                .sum(),
        )
    }
}
