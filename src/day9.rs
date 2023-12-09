#![allow(unused)]
use crate::util::DaySolver;
use std::iter::once;

pub struct Day9();

impl Day9 {
    fn parse(input: Vec<String>) -> Vec<Vec<i64>> {
        input
            .iter()
            .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
            .collect()
    }

    fn extrapolate_forward<I>(mut iter: I) -> i64
    where
        I: Iterator<Item = i64>,
    {
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let mut c = -999; // Dumb placeholder :(
        if a == 0 && b == 0 {
            if let Some(cc) = iter.next() {
                if cc == 0 {
                    return 0;
                } else {
                    c = cc;
                }
            } else {
                return 0;
            }
        } else {
            c = iter.next().unwrap();
        }
        struct ScanState(i64);
        let next_level = once(a - b)
            .chain(once(b - c))
            .chain(iter.scan(ScanState(c), |mut state, item| {
                let ret = state.0 - item;
                state.0 = item;
                Some(ret)
            }))
            .collect::<Vec<i64>>();
        println!("{:?}", next_level);
        let next = Self::extrapolate_forward(next_level.iter().copied());
        next + a
    }
    fn extrapolate_back(items: &[i64]) -> i64 {
        if items.iter().all(|i| *i == 0) {
            return 0;
        }
        struct ScanState(i64);
        let next_items = items
            .iter()
            .skip(1)
            .scan(ScanState(items[0]), |mut state, item| {
                let ret = item - state.0;
                state.0 = *item;
                Some(ret)
            })
            .collect::<Vec<i64>>();
        let x = items[0] - Self::extrapolate_back(&next_items);
        // println!("Extrapolating {x} {:?}", items);
        x
    }
}

type Solution = i64;
impl DaySolver<Solution> for Day9 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        // Some(
        //     data.iter()
        //         .map(|line| Self::extrapolate_forward(line.iter().rev().copied()))
        //         .sum(),
        // )
        let rev_data: Vec<Vec<i64>> = data
            .iter()
            .map(|line| line.iter().rev().map(|x| -x).collect())
            .collect();
        Some(
            -rev_data
                .iter()
                .map(|line| Self::extrapolate_back(line))
                .sum::<i64>(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        let datums: Vec<i64> = data
            .iter()
            .map(|line| Self::extrapolate_back(line))
            .collect();
        let x = Some(data.iter().map(|line| Self::extrapolate_back(line)).sum());
        println!("{datums:?}");
        println!("{:?}", datums.iter().sum::<i64>());
        x
    }
}
