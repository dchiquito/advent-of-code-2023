#![allow(unused)]
use crate::util::DaySolver;

use num::{BigInt, BigRational, Zero};
use std::str::FromStr;

#[derive(Debug)]
pub struct Hail {
    x: BigInt,
    y: BigInt,
    z: BigInt,
    vx: BigInt,
    vy: BigInt,
    vz: BigInt,
}

impl From<&String> for Hail {
    fn from(value: &String) -> Self {
        let (coords, velocities) = value.split_once(" @ ").unwrap();
        let mut c_iter = coords.split(", ");
        let x = BigInt::from_str(c_iter.next().unwrap()).unwrap();
        let y = BigInt::from_str(c_iter.next().unwrap()).unwrap();
        let z = BigInt::from_str(c_iter.next().unwrap()).unwrap();
        let mut v_iter = velocities.split(", ");
        let vx = BigInt::from_str(v_iter.next().unwrap()).unwrap();
        let vy = BigInt::from_str(v_iter.next().unwrap()).unwrap();
        let vz = BigInt::from_str(v_iter.next().unwrap()).unwrap();
        Hail {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

impl Hail {
    pub fn intersection_1(
        &self,
        other: &Hail,
    ) -> Option<(BigRational, BigRational, BigRational, BigRational)> {
        if self.vx == BigInt::zero() {
            return None;
        }
        if other.vx == BigInt::zero() {
            return None;
        }
        let v1 = BigRational::new(self.vy.clone(), self.vx.clone());
        let v2 = BigRational::new(other.vy.clone(), other.vx.clone());
        if &v1 - &v2 == BigRational::zero() {
            return None;
        }
        let x = ((&v1 * &self.x) - (&v2 * &other.x) + &other.y - &self.y) / (&v1 - &v2);
        let y = (&v1 * (&x - &self.x)) + &self.y;
        let t1 = (&x - &self.x) / &self.vx;
        let t2 = (&x - &other.x) / &other.vx;
        // let t11 = (&y - &self.y) / &self.vy;
        // let t22 = (&y - &other.y) / &other.vy;
        // println!("{t1} {t11} {t2} {t22}");
        Some((x, y, t1, t2))
    }
}

pub struct Day24();
impl Day24 {
    pub fn parse(input: &[String]) -> Vec<Hail> {
        input.iter().map(Hail::from).collect()
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day24 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let hails = Self::parse(&input);
        let zero = BigRational::new(BigInt::from(0), BigInt::from(1));
        let min = BigRational::new(BigInt::from(200000000000000_u64), BigInt::from(1));
        let max = BigRational::new(BigInt::from(400000000000000_u64), BigInt::from(1));
        // let min = BigRational::new(BigInt::from(7), BigInt::from(1));
        // let max = BigRational::new(BigInt::from(27), BigInt::from(1));
        let mut sum = 0;
        for i in 0..hails.len() {
            for j in i + 1..hails.len() {
                if let Some((x, y, t1, t2)) = hails[i].intersection_1(&hails[j]) {
                    // println!("Intersection of {i} and {j} is {x} {y} {t1} {t2}");
                    if t1 >= zero && t2 >= zero && min <= x && x <= max && min <= y && y <= max {
                        sum += 1;
                    }
                }
            }
        }
        Some(sum)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
