#![allow(unused)]
use std::collections::HashSet;

use regex::Regex;

use crate::util::DaySolver;

type Solution = usize;

pub struct Day4();

pub struct Card {
    winners: [u32; 10],
    have: [u32; 25],
}

macro_rules! parse_digits {
    ($value:expr, $a:expr) => {{
        let mut cs = $value[$a..$a + 2].chars();
        let x = cs.next().unwrap();
        let y = cs.next().unwrap();
        if x == ' ' {
            y.to_digit(10).unwrap()
        } else {
            x.to_digit(10).unwrap() * 10 + y.to_digit(10).unwrap()
        }
        // $value[$a..$a + 2].trim_start().parse().unwrap()
    }};
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let winners = [
            parse_digits!(value, 10),
            parse_digits!(value, 13),
            parse_digits!(value, 16),
            parse_digits!(value, 19),
            parse_digits!(value, 22),
            parse_digits!(value, 25),
            parse_digits!(value, 28),
            parse_digits!(value, 31),
            parse_digits!(value, 34),
            parse_digits!(value, 37),
        ];
        let have = [
            parse_digits!(value, 42),
            parse_digits!(value, 45),
            parse_digits!(value, 48),
            parse_digits!(value, 51),
            parse_digits!(value, 54),
            parse_digits!(value, 57),
            parse_digits!(value, 60),
            parse_digits!(value, 63),
            parse_digits!(value, 66),
            parse_digits!(value, 69),
            parse_digits!(value, 72),
            parse_digits!(value, 75),
            parse_digits!(value, 78),
            parse_digits!(value, 81),
            parse_digits!(value, 84),
            parse_digits!(value, 87),
            parse_digits!(value, 90),
            parse_digits!(value, 93),
            parse_digits!(value, 96),
            parse_digits!(value, 99),
            parse_digits!(value, 102),
            parse_digits!(value, 105),
            parse_digits!(value, 108),
            parse_digits!(value, 111),
            parse_digits!(value, 114),
        ];
        Card { winners, have }
    }
}

impl Card {
    fn matches(&self) -> usize {
        self.have
            .iter()
            .filter(|h| self.winners.contains(h))
            .count()
    }
}

impl Day4 {
    pub fn parse(input: &[String]) -> Vec<Card> {
        input.iter().map(|line| line.into()).collect()
    }
    pub fn _part1(cards: &[Card]) -> Solution {
        cards
            .iter()
            .map(|card| {
                let exp = card.matches() as u32;
                if exp == 0 {
                    0
                } else {
                    2_usize.pow(exp - 1)
                }
            })
            .sum()
    }
    pub fn _part2(cards: &[Card]) -> Solution {
        let mut copies = vec![1; cards.len()];
        for i in 0..cards.len() {
            for j in 1..=cards[i].matches() {
                copies[i + j] += copies[i];
            }
        }
        copies.iter().sum()
    }
}

impl DaySolver<Solution> for Day4 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let cards = Self::parse(&input);
        Some(Self::_part1(&cards))
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let cards = Self::parse(&input);
        Some(Self::_part2(&cards))
    }
}
