#![allow(unused)]
use crate::util::DaySolver;

use num::{BigInt, BigRational, Integer, Zero};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
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
    pub fn p(&self, index: usize) -> &BigInt {
        assert!(index < 3);
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
    pub fn v(&self, index: usize) -> &BigInt {
        assert!(index < 3);
        match index {
            0 => &self.vx,
            1 => &self.vy,
            2 => &self.vz,
            _ => unreachable!(),
        }
    }
}

pub struct Day24();
impl Day24 {
    pub fn parse(input: &[String]) -> Vec<Hail> {
        input.iter().map(Hail::from).collect()
    }
}

impl Day24 {
    fn find_x(vx: &BigInt, vy: &BigInt, a: &Hail, b: &Hail) -> Option<BigInt> {
        let xa = &a.x;
        let xb = &b.x;
        let ya = &a.y;
        let yb = &b.y;
        let cxa = &(&a.vx - vx);
        let cxb = &(&b.vx - vx);
        let cya = &(&a.vy - vy);
        let cyb = &(&b.vy - vy);
        let numerator = ((yb - ya) * cxa * cxb) + (xa * cya * cxb) - (xb * cyb * cxa);
        let denominator = (cya * cxb) - (cyb * cxa);
        if (!denominator.is_zero()) && numerator.is_multiple_of(&denominator) {
            Some(numerator / denominator)
        } else {
            None
        }
    }
    fn find_t(x: &BigInt, vx: &BigInt, h: &Hail) -> Option<BigInt> {
        let numerator = x - &h.x;
        let denominator = &h.vx - vx;
        // Time must be a positive integer
        if (!denominator.is_zero())
            && numerator.sign() == denominator.sign()
            && numerator.is_multiple_of(&denominator)
        {
            Some(numerator / denominator)
        } else {
            None
        }
    }
    fn solve(vx: &BigInt, vy: &BigInt, vz: &BigInt, h: &Hail, t: &BigInt) -> Hail {
        let x = &h.x + (&h.vx * t) - (vx * t);
        let y = &h.y + (&h.vy * t) - (vy * t);
        let z = &h.z + (&h.vz * t) - (vz * t);
        Hail {
            x,
            y,
            z,
            vx: vx.clone(),
            vy: vy.clone(),
            vz: vz.clone(),
        }
    }
    fn verify_solution(rock: &Hail, hails: &[Hail]) -> bool {
        hails.iter().all(|h| {
            if let Some(t) = Self::find_t(&rock.x, &rock.vx, h) {
                rock == &Self::solve(&rock.vx, &rock.vy, &rock.vz, h, &t)
            } else {
                false
            }
        })
    }
}

fn int_it(n: &BigInt) -> i64 {
    let (sign, digits) = n.to_u64_digits();
    match sign {
        num::bigint::Sign::Minus => 0 - digits[0] as i64,
        num::bigint::Sign::NoSign => 0,
        num::bigint::Sign::Plus => digits[0] as i64,
    }
}
fn float_it(n: &BigInt) -> f64 {
    int_it(n) as f64
}

type Solution = u64;
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
        let hails = Self::parse(&input);
        let min_v = -500;
        let max_v = 500;
        for vx in min_v..max_v {
            let vx = BigInt::from(vx);
            for vy in min_v..max_v {
                let vy = BigInt::from(vy);
                if let Some(x) = Self::find_x(&vx, &vy, &hails[0], &hails[1]) {
                    if let Some(t) = Self::find_t(&x, &vx, &hails[0]) {
                        for vz in min_v..max_v {
                            let vz = BigInt::from(vz);
                            let rock = Self::solve(&vx, &vy, &vz, &hails[0], &t);
                            if Self::verify_solution(&rock, &hails) {
                                return Some((&rock.x + &rock.y + &rock.z).to_u64_digits().1[0]);
                            }
                        }
                    }
                }
            }
        }
        unreachable!()
    }
}
