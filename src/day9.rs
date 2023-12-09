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
    fn extrapolate_back<I>(mut iter: I) -> i64
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
        let next_level = once(b - a)
            .chain(once(c - b))
            .chain(iter.scan(ScanState(c), |mut state, item| {
                let ret = item - state.0;
                state.0 = item;
                Some(ret)
            }))
            .collect::<Vec<i64>>();
        let next = Self::extrapolate_back(next_level.iter().copied());
        println!("{} {:?}", next, next_level);
        a - next
    }
}

type Solution = i64;
impl DaySolver<Solution> for Day9 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        Some(
            data.iter()
                .map(|line| Self::extrapolate_forward(line.iter().rev().copied()))
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let data = Self::parse(input);
        // let a = Some(
        //     data.iter()
        //         .map(|line| {
        //             let x = Self::extrapolate_back(line.iter().copied());
        //             println!("{x} {line:?}");
        //             x
        //         })
        //         .sum(),
        // );
        Some(
            data.iter()
                .map(|line| {
                    let x = Self::extrapolate_forward(line.iter().copied().map(|x| -x));
                    println!("{x} {line:?}");
                    x
                })
                .sum(),
        )
    }
}
