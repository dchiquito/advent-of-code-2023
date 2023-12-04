#![allow(unused)]
use std::collections::HashSet;

use regex::Regex;

use crate::util::DaySolver;

type Solution = usize;

pub struct Day4();

#[derive(Debug)]
struct Card {
    number: u32,
    winners: HashSet<u32>,
    have: Vec<u32>,
}

impl From<&String> for Card {
    fn from(value: &String) -> Self {
        let pattern = Regex::new("Card +([0-9]+): ([0-9 ]+)\\|([0-9 ]+)").unwrap();
        let captures = pattern.captures(value).unwrap();

        let number = captures.get(1).unwrap().as_str().parse().unwrap();
        let winners = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|d| d != &"")
            .map(|d| d.parse().unwrap())
            .collect();
        let have = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(' ')
            .filter(|d| d != &"")
            .map(|d| d.parse().unwrap())
            .collect();
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
    fn parse(input: Vec<String>) -> Vec<Card> {
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
