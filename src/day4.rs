#![allow(unused)]
use std::collections::HashSet;

use regex::Regex;

use crate::util::DaySolver;

type Solution = usize;

pub struct Day4();

#[derive(Debug)]
pub struct Card {
    number: u32,
    winners: HashSet<u32>,
    have: Vec<u32>,
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
        //
        // Card   1: 84 17 45 77 11 66 94 28 71 70 | 45 51 86 83 53 58 64 30 67 96 41 89  8 17 33 50 80 84  6  2 87 72 27 63 77
        let number: u32 = value[5..8].trim().parse().unwrap();
        let winners = HashSet::from([
            value[10..12].trim().parse().unwrap(),
            value[13..15].trim().parse().unwrap(),
            value[16..18].trim().parse().unwrap(),
            value[19..21].trim().parse().unwrap(),
            value[22..24].trim().parse().unwrap(),
            value[25..27].trim().parse().unwrap(),
            value[28..30].trim().parse().unwrap(),
            value[31..33].trim().parse().unwrap(),
            value[34..36].trim().parse().unwrap(),
            value[37..39].trim().parse().unwrap(),
        ]);
        let have = vec![
            value[42..44].trim().parse().unwrap(),
            value[45..47].trim().parse().unwrap(),
            value[48..50].trim().parse().unwrap(),
            value[51..53].trim().parse().unwrap(),
            value[54..56].trim().parse().unwrap(),
            value[57..59].trim().parse().unwrap(),
            value[60..62].trim().parse().unwrap(),
            value[63..65].trim().parse().unwrap(),
            value[66..68].trim().parse().unwrap(),
            value[69..71].trim().parse().unwrap(),
            value[72..74].trim().parse().unwrap(),
            value[75..77].trim().parse().unwrap(),
            value[78..80].trim().parse().unwrap(),
            value[81..83].trim().parse().unwrap(),
            value[84..86].trim().parse().unwrap(),
            value[87..89].trim().parse().unwrap(),
            value[90..92].trim().parse().unwrap(),
            value[93..95].trim().parse().unwrap(),
            value[96..98].trim().parse().unwrap(),
            value[99..101].trim().parse().unwrap(),
            value[102..104].trim().parse().unwrap(),
            value[105..107].trim().parse().unwrap(),
            value[108..110].trim().parse().unwrap(),
            value[111..113].trim().parse().unwrap(),
            value[114..116].trim().parse().unwrap(),
        ];
        Card {
            number,
            winners,
            have,
        }
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
    pub fn parse(input: Vec<String>) -> Vec<Card> {
        input.iter().map(|line| line.into()).collect()
    }
}

impl DaySolver<Solution> for Day4 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let cards = Self::parse(input);
        Some(
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
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let cards = Self::parse(input);
        let mut copies = vec![1; cards.len()];
        for i in 0..cards.len() {
            for j in 1..=cards[i].matches() {
                copies[i + j] += copies[i];
            }
        }
        Some(copies.iter().sum())
    }
}
