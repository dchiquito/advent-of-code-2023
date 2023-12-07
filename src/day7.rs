#![allow(unused)]
use crate::util::DaySolver;

type Solution = usize;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card(usize);

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card(match value {
            // J is remapped to 1 in part 2
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        })
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct HandValue(usize, usize, usize, usize, usize, usize);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    // A neat hack: the more valuable hands are more densely connected
    pub fn hand_type(&self) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in i + 1..5 {
                if self.cards[i] == self.cards[j] {
                    sum += 1;
                }
            }
        }
        sum
    }
    pub fn hand_type_2(&self, j_value: usize) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in i + 1..5 {
                if self.cards[i] == self.cards[j]
                    || (self.cards[i].0 == 1 && self.cards[j].0 == j_value)
                    || (self.cards[j].0 == 1 && self.cards[i].0 == j_value)
                {
                    sum += 1;
                }
            }
        }
        sum
    }
    pub fn value(&self) -> HandValue {
        HandValue(
            self.hand_type(),
            self.cards[0].0,
            self.cards[1].0,
            self.cards[2].0,
            self.cards[3].0,
            self.cards[4].0,
        )
    }
    pub fn value_2(&self) -> HandValue {
        let mut max_value = HandValue(
            self.hand_type_2(2),
            self.cards[0].0,
            self.cards[1].0,
            self.cards[2].0,
            self.cards[3].0,
            self.cards[4].0,
        );

        for j_value in 3..=14 {
            max_value = max_value.max(HandValue(
                self.hand_type_2(j_value),
                self.cards[0].0,
                self.cards[1].0,
                self.cards[2].0,
                self.cards[3].0,
                self.cards[4].0,
            ));
        }
        max_value
    }
}

impl From<&String> for Hand {
    fn from(value: &String) -> Self {
        let mut i = value.chars();
        let cards = [
            i.next().unwrap().into(),
            i.next().unwrap().into(),
            i.next().unwrap().into(),
            i.next().unwrap().into(),
            i.next().unwrap().into(),
        ];
        let bid = value[6..].parse().unwrap();
        Hand { cards, bid }
    }
}

pub struct Day7();

impl Day7 {
    fn parse(input: Vec<String>) -> Vec<Hand> {
        input.iter().map(Hand::from).collect()
    }
}

impl DaySolver<Solution> for Day7 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut hands = Self::parse(input);
        hands.sort_by_key(Hand::value);
        Some(hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut hands = Self::parse(input);
        // Demote jokers to the lowest card value
        hands.iter_mut().for_each(|h| {
            h.cards.iter_mut().for_each(|c| {
                if c.0 == 11 {
                    c.0 = 1;
                }
            })
        });
        hands.sort_by_key(Hand::value_2);
        Some(hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum())
    }
}
