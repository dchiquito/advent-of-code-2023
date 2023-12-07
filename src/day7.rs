#![allow(unused)]
use crate::util::DaySolver;

type Solution = usize;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Card(usize);

impl From<char> for Card {
    fn from(value: char) -> Self {
        Card(match value {
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

impl Card {
    fn from2(value: char) -> Self {
        Card(match value {
            // J is remapped to 1 in part 2
            'J' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hand {
    hand_type: usize,
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    // A neat hack: the more valuable hands are more densely connected
    pub fn new1(cards: [Card; 5], bid: usize) -> Hand {
        let mut sum = 0;
        for i in 0..5 {
            for j in i + 1..5 {
                if cards[i] == cards[j] {
                    sum += 1;
                }
            }
        }
        Hand {
            hand_type: sum,
            cards,
            bid,
        }
    }
    pub fn new2(cards: [Card; 5], bid: usize) -> Hand {
        // If there are no jokers, just use the constructor from part 1
        if !cards.contains(&Card(1)) {
            return Self::new1(cards, bid);
        }
        let mut max_hand = Hand {
            hand_type: Self::hand_type_for_joker(&cards, cards[0].0),
            cards,
            bid,
        };
        let mut this_hand = max_hand.clone();

        for i in 1..5 {
            let j_value = max_hand.cards[i].0;
            // A joker is never the most valuable as a joker, it should always impersonate another
            // card
            if j_value == 1 {
                continue;
            }
            this_hand.hand_type = Self::hand_type_for_joker(&max_hand.cards, j_value);
            if this_hand > max_hand {
                max_hand.hand_type = this_hand.hand_type;
            }
        }
        max_hand
    }
    pub fn hand_type_for_joker(cards: &[Card; 5], j_value: usize) -> usize {
        let mut sum = 0;
        for i in 0..5 {
            for j in i + 1..5 {
                if cards[i] == cards[j]
                    || (cards[i].0 == 1 && cards[j].0 == j_value)
                    || (cards[j].0 == 1 && cards[i].0 == j_value)
                {
                    sum += 1;
                }
            }
        }
        sum
    }
}

pub struct Day7();

impl Day7 {
    pub fn parse1(input: &[String]) -> Vec<Hand> {
        input
            .iter()
            .map(|line| {
                let mut i = line.chars();
                let cards = [
                    Card::from(i.next().unwrap()),
                    Card::from(i.next().unwrap()),
                    Card::from(i.next().unwrap()),
                    Card::from(i.next().unwrap()),
                    Card::from(i.next().unwrap()),
                ];
                let bid = line[6..].parse().unwrap();
                Hand::new1(cards, bid)
            })
            .collect()
    }
    pub fn parse2(input: &[String]) -> Vec<Hand> {
        input
            .iter()
            .map(|line| {
                let mut i = line.chars();
                let cards = [
                    Card::from2(i.next().unwrap()),
                    Card::from2(i.next().unwrap()),
                    Card::from2(i.next().unwrap()),
                    Card::from2(i.next().unwrap()),
                    Card::from2(i.next().unwrap()),
                ];
                let bid = line[6..].parse().unwrap();
                Hand::new2(cards, bid)
            })
            .collect()
    }
}

impl DaySolver<Solution> for Day7 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut hands = Self::parse1(&input);
        hands.sort();
        Some(hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum())
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let mut hands = Self::parse2(&input);
        hands.sort();
        Some(hands.iter().enumerate().map(|(i, h)| (i + 1) * h.bid).sum())
    }
}
