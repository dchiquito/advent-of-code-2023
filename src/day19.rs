#![allow(unused)]
use std::collections::HashMap;

use crate::util::DaySolver;

#[derive(Debug)]
pub struct Workflow {
    id: String,
    rules: Vec<(usize, bool, u64, String)>,
    fallback: String,
}
impl From<&String> for Workflow {
    fn from(value: &String) -> Self {
        let (id, rules_str) = value.split_once('{').unwrap();
        let rules_str = &rules_str[..rules_str.len() - 1];
        let rules_vec: Vec<String> = rules_str.split(',').map(str::to_string).collect();
        let rules = rules_vec[..rules_vec.len() - 1]
            .iter()
            .map(|rule| {
                let index = match rule.chars().next().unwrap() {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => unreachable!(),
                };
                let gt = rule.chars().nth(1).unwrap() == '>';
                let (cmp, next) = rule[2..].split_once(':').unwrap();
                (index, gt, cmp.parse().unwrap(), next.to_string())
            })
            .collect();
        let fallback = rules_vec.last().unwrap().to_string();
        Workflow {
            id: id.to_string(),
            rules,
            fallback,
        }
    }
}
impl Workflow {
    pub fn eval(&self, xmas: &Xmas) -> &str {
        for (index, is_cmp_gt, cmp, workflow) in self.rules.iter() {
            let value = xmas[*index];
            if (*is_cmp_gt && value > *cmp) || ((!is_cmp_gt) && value < *cmp) {
                return workflow;
            }
        }
        &self.fallback
    }
}

pub type Workflows = HashMap<String, Workflow>;
pub type Xmas = [u64; 4];
pub type Xmases = Vec<Xmas>;

pub struct Day19();

impl Day19 {
    pub fn parse1(input: &[String]) -> (Workflows, Xmases) {
        let split = input
            .iter()
            .enumerate()
            .find(|(_, line)| line.is_empty())
            .unwrap()
            .0;
        let workflows = input[0..split]
            .iter()
            .map(|line| {
                let w = Workflow::from(line);
                (w.id.to_string(), w)
            })
            .collect();
        let xmases = input[split + 1..]
            .iter()
            .map(|line2| {
                let mut iter = line2[1..line2.len() - 1].split(',');
                let x = iter.next().unwrap()[2..].parse().unwrap();
                let m = iter.next().unwrap()[2..].parse().unwrap();
                let a = iter.next().unwrap()[2..].parse().unwrap();
                let s = iter.next().unwrap()[2..].parse().unwrap();
                [x, m, a, s]
            })
            .collect();
        (workflows, xmases)
    }
    pub fn parse2(input: &[String]) -> Workflows {
        let split = input
            .iter()
            .enumerate()
            .find(|(_, line)| line.is_empty())
            .unwrap()
            .0;
        input
            .iter()
            .take(split)
            .map(|line| {
                let w = Workflow::from(line);
                (w.id.to_string(), w)
            })
            .collect()
    }
    pub fn eval(workflows: &Workflows, id: &str, xmas: &Xmas) -> bool {
        let mut id = id;
        while id != "A" && id != "R" {
            id = workflows.get(id).unwrap().eval(xmas);
        }
        id == "A"
    }
    pub fn area(low: &Xmas, high: &Xmas) -> u64 {
        low.iter()
            .zip(high.iter())
            .map(|(l, h)| h - l)
            .reduce(|a, b| a * b)
            .unwrap()
    }
    pub fn range_count(workflows: &Workflows, id: &str, low: &Xmas, high: &Xmas) -> u64 {
        if id == "R" {
            return 0;
        }
        if id == "A" {
            // Get the dimensions of the range and multiply them together
            return Self::area(low, high);
        }
        let mut low = *low;
        let mut high = *high;
        let workflow = workflows.get(id).unwrap();
        let mut sum = 0;
        // We have a range [low..high) for each XMAS index
        // Assume arbitrary values in these ranges are "x", where low <= x < high
        // Because my brain is small, low=10 and high = 15 and x = (10,11,12,13,14)
        for (index, is_cmp_gt, cmp, next) in workflow.rules.iter() {
            // if x > cmp for some low<=x<high, the rule applies
            // if cmp is 14 or greater, then the rule cannot apply.
            // Only proceed if high-1 > cmp
            if *is_cmp_gt && high[*index] - 1 > *cmp {
                // If cmp is 9 or less, then the rule applies to the entire range
                if *cmp < low[*index] {
                    return sum + Self::range_count(workflows, next, &low, &high);
                // Some of the range applies, and some doesn't.
                // Let's set cmp=12.
                } else {
                    // First, recursively figure out the area of the higher range, since it
                    // applies.
                    // x > 12 means x=(13,14)=[13..15) means low=cmp+1, high=high
                    let mut new_low = low;
                    new_low[*index] = *cmp + 1;
                    sum += Self::range_count(workflows, next, &new_low, &high);
                    // Now adjust the range to the lower one, x=(10,11,12)=[10..13) means
                    // low=low,high=cmp+1
                    high[*index] = *cmp + 1;
                }
            // if x < cmp for some low<=x<high, the rule applies
            // if cmp is 10 or lesser, then the rule cannot apply.
            // Only proceed if low < cmp
            } else if (!is_cmp_gt) && low[*index] < *cmp {
                // If high > cmp, then low < cmp < high, meaning we need to split the range
                // [low..cmp) applies to this rule
                // [cmp..high) needs to be checked against subsequent rules
                if high[*index] > *cmp {
                    let mut new_high = high;
                    new_high[*index] = *cmp;
                    sum += Self::range_count(workflows, next, &low, &new_high);
                    low[*index] = *cmp;
                // low <= x < high <= cmp, meaning the whole range matches the condition
                // It's impossible to continue, so just return
                } else {
                    return sum + Self::range_count(workflows, next, &low, &high);
                }
            }
        }
        sum + Self::range_count(workflows, &workflow.fallback, &low, &high)
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day19 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let (workflows, xmases) = Self::parse1(&input);
        Some(
            xmases
                .iter()
                .filter(|xmas| Self::eval(&workflows, "in", xmas))
                .map(|xmas| xmas.iter().sum::<u64>())
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let workflows = Self::parse2(&input);
        Some(Self::range_count(&workflows, "in", &[1; 4], &[4001; 4]))
    }
}
