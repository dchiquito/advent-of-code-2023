#![allow(unused)]
use crate::util::DaySolver;

use num::{BigInt, BigRational, Integer, Zero};
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
    fn valid_vs(hails: &[Hail], index: usize) -> Vec<i64> {
        (-1999_i64..1999_i64)
            // (-499_i64..499_i64)
            // let valid_vx: Vec<i64> = (-350_i64..300_i64)
            .filter(|v| {
                let mut min = BigInt::zero();
                let mut max = BigInt::from(i64::MAX);
                for hail in hails.iter() {
                    let v = BigInt::from(*v);
                    if &v == hail.v(index) {
                        // then v = hail.v
                        min = min.max(hail.p(index).clone());
                        max = max.min(hail.p(index).clone());
                    } else if hail.v(index) > &v {
                        min = min.max(hail.p(index).clone());
                    } else {
                        max = max.min(hail.p(index).clone());
                    }
                    if max < min {
                        break;
                    }
                }
                // println!("{v} [{}..{}]\t{} things to check", &min, &max, &max - &min);
                max > min
            })
            .collect()
    }
}

type Solution = usize;
impl DaySolver<Solution> for Day24 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        return Some(666);
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
        let xs = Self::valid_vs(&hails, 0);
        let ys = Self::valid_vs(&hails, 1);
        let zs = Self::valid_vs(&hails, 2);
        // let mut potential_t1_vxs = vec![];
        for vx in xs.iter() {
            for vy in ys.iter() {
                let x1 = &hails[3].x;
                let x2 = &hails[4].x;
                let vx1 = &hails[3].vx;
                let vx2 = &hails[4].vx;
                let y1 = &hails[3].y;
                let y2 = &hails[4].y;
                let vy1 = &hails[3].vy;
                let vy2 = &hails[4].vy;
                let numerator = ((vx2 - vx) * (y1 - y2)) - ((vy2 - vy) * (x1 - x2));
                let denominator = ((vx1 - vx) * (vy2 - vy)) - ((vy1 - vy) * (vx2 - vx));
                if denominator.is_zero() {
                    continue;
                }
                if numerator.is_multiple_of(&denominator) {
                    let t1 = &numerator / &denominator;
                    if t1 > BigInt::zero() {
                        // println!("{}/{} = {}", &numerator, &denominator, &t1);
                        // potential_t1_vxs.push((t1, vx));
                        let x = x1 + &t1 * (vx1 - vx);
                        let y = y1 + &t1 * (vy1 - vy);
                        let c = hails
                            .iter()
                            .filter(|hail| {
                                let num = &x - &hail.x;
                                let denom = &hail.vx - vx;
                                (!denom.is_zero()) && num.is_multiple_of(&denom)
                            })
                            .count();
                        // println!("\t\t\t\t\t\tCounted {c} out of {}", hails.len());
                        if c > 50 {
                            println!("FOUND IT {x} {c}");
                            println!("{} ==== {}", &x + (&t1 * vx), x1 + (&t1 * vx1));
                            println!("{} ==== {}", &y + (&t1 * vy), y1 + (&t1 * vy1));
                            hails.iter().for_each(|hail| {
                                let num = &x - &hail.x;
                                let denom = &hail.vx - vx;
                                if (!denom.is_zero()) && num.is_multiple_of(&denom) {
                                    println!("yay {}", num / denom);
                                } else {
                                    println!("no {num} {denom} {}", num.is_multiple_of(&denom));
                                }
                            });
                            panic!()
                        }
                    }
                }
            }
        }
        None
    }
}
