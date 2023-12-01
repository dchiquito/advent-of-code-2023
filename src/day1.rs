#![allow(unused)]
use std::collections::HashMap;

use crate::util::DaySolver;

type Solution = u32;

pub struct Day1();

impl Day1 {
    fn calibration_number(line: &str) -> u32 {
        let f = line
            .chars()
            .find(|c| "0123456789".contains(*c))
            .unwrap()
            .to_digit(10)
            .unwrap();
        let l = line
            .chars()
            .filter(|c| "0123456789".contains(*c))
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();
        (f * 10) + l
    }
    fn with_spelling(line: &str) -> Option<u32> {
        let pairs = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];
        for (start, digit) in pairs {
            if line.starts_with(start) {
                return Some(digit);
            }
        }
        None
    }
    fn spelling_calibration_number(line: &str) -> u32 {
        let mut first = None;
        let mut last = None;
        for i in 0..line.len() {
            if let Some(num) = Self::with_spelling(&line[i..]) {
                if first.is_none() {
                    first = Some(num);
                }
                last = Some(num);
            }
        }
        (first.unwrap() * 10) + last.unwrap()
    }
}

impl DaySolver<Solution> for Day1 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        Some(
            input
                .iter()
                .map(|line| Self::calibration_number(line))
                .sum::<u32>(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        Some(
            input
                .iter()
                .map(|line| Self::spelling_calibration_number(line))
                .sum::<u32>(),
        )
    }
}
