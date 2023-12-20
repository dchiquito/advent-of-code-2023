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
    pub fn parse(input: &[String]) -> (Workflows, Xmases) {
        let mut workflows = Workflows::default();
        for (i, line) in input.iter().enumerate() {
            if line.is_empty() {
                let xmases = input[i + 1..]
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
                return (workflows, xmases);
            } else {
                let workflow = Workflow::from(line);
                workflows.insert(workflow.id.clone(), workflow);
            }
        }
        unreachable!()
    }
    pub fn eval(workflows: &Workflows, id: &str, xmas: &Xmas) -> bool {
        let mut id = id;
        while id != "A" && id != "R" {
            id = workflows.get(id).unwrap().eval(xmas);
        }
        id == "A"
    }
    pub fn range_count(workflows: &Workflows, id: &str, low: &Xmas, high: &Xmas) -> u64 {
        if id == "R" {
            return 0;
        }
        if id == "A" {
            // Get the dimensions of the range and multiply them together
            let area = low
                .iter()
                .zip(high.iter())
                .map(|(l, h)| h - l)
                .reduce(|a, b| a * b)
                .unwrap();
            println!("\tIncrementing {low:?}..{high:?} = {area}");
            return area;
        }
        let mut low = *low;
        let mut high = *high;
        let workflow = workflows.get(id).unwrap();
        let mut sum = 0;
        // We have a range [low..high) for each XMAS index
        // Assume arbitrary values in these ranges are "x", where low <= x < high
        for (index, is_cmp_gt, cmp, next) in workflow.rules.iter() {
            println!(
                "{id}:\tChecking range {low:?}..{high:?}:  ({index}) x {} {cmp} for {} <= x < {} => {next}",
                if *is_cmp_gt { '>' } else { '<' },
                low[*index],
                high[*index]
            );
            // if x > cmp for some low<=x<high, the rule applies
            // If the high is less than or equal to cmp, then the rule never applies
            // Only proceed if high > cmp
            if *is_cmp_gt && high[*index] > *cmp {
                // If low <= cmp, then low <= cmp < high, meaning we need to split the range
                // [low..cmp+1) needs to be checked against subsequent rules
                // [cmp+1..high) applies to this rule
                if low[*index] <= *cmp {
                    let tmp_low = low[*index];
                    low[*index] = *cmp + 1;
                    sum += Self::range_count(workflows, next, &low, &high);
                    low[*index] = tmp_low;
                    high[*index] = *cmp + 1;
                // cmp < low <= x < high, meaning the whole range matches the condition
                // It's impossible to continue, so just return
                } else {
                    return sum + Self::range_count(workflows, next, &low, &high);
                }
            // if x < cmp for some low<=x<high, the rule applies
            // If the low is higher than cmp, then the rule never applies
            // Only proceed if low < cmp
            } else if (!is_cmp_gt) && low[*index] < *cmp {
                // If high > cmp, then low < cmp < high, meaning we need to split the range
                // [low..cmp) applies to this rule
                // [cmp..high) needs to be checked against subsequent rules
                if high[*index] > *cmp {
                    let tmp_high = high[*index];
                    high[*index] = *cmp;
                    sum += Self::range_count(workflows, next, &low, &high);
                    high[*index] = tmp_high;
                    low[*index] = *cmp;
                // low <= x < high <= cmp, meaning the whole range matches the condition
                // It's impossible to continue, so just return
                } else {
                    return sum + Self::range_count(workflows, next, &low, &high);
                }
            }
        }
        println!("\tFalling back to {}", workflow.fallback);
        sum + Self::range_count(workflows, &workflow.fallback, &low, &high)
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day19 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let (workflows, xmases) = Self::parse(&input);
        Some(
            xmases
                .iter()
                .filter(|xmas| Self::eval(&workflows, "in", xmas))
                .map(|xmas| xmas.iter().sum::<u64>())
                .sum(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        let (workflows, _) = Self::parse(&input);
        Some(Self::range_count(&workflows, "in", &[0; 4], &[4001; 4]))
    }
}
