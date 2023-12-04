#![allow(unused)]
use std::collections::HashSet;

use regex::Regex;

use crate::util::DaySolver;

type Solution = usize;

pub struct Day4();

pub struct Card {
    number: u32,
    winners: [u32; 10],
    have: [u32; 25],
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let number: u32 = value[5..8].trim().parse().unwrap();
        let winners = [
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
        ];
        let have = [
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
