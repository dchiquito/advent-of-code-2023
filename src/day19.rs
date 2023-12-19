#![allow(unused)]
use std::collections::HashMap;

use crate::util::DaySolver;

#[derive(Debug)]
pub struct Workflow {
    id: String,
    rules: Vec<(char, bool, u64, String)>,
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
                let xmas = rule.chars().next().unwrap();
                let gt = rule.chars().nth(1).unwrap() == '>';
                let (cmp, next) = rule[2..].split_once(':').unwrap();
                (xmas, gt, cmp.parse().unwrap(), next.to_string())
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
        for (the_char, is_cmp_gt, cmp, workflow) in self.rules.iter() {
            let value = match the_char {
                'x' => xmas[0],
                'm' => xmas[1],
                'a' => xmas[2],
                's' => xmas[3],
                _ => unreachable!(),
            };
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
    pub fn eval(workflows: &Workflows, workflow: &str, xmas: &Xmas) -> bool {
        let mut workflow = workflow;
        while workflow != "A" && workflow != "R" {
            workflow = workflows.get(workflow).unwrap().eval(xmas);
        }
        workflow == "A"
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
        None
    }
}
