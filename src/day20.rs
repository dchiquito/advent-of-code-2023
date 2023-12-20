#![allow(unused)]
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
};

use num::Integer;

use crate::util::DaySolver;

pub type Pulse = (u64, u64, bool);
pub type Pulses = VecDeque<Pulse>;

#[derive(Debug, Eq, PartialEq)]
pub enum Kind {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<u64, bool>),
    Rx(bool),
}

#[derive(Debug)]
pub struct Module {
    id: u64,
    kind: Kind,
    outputs: Vec<u64>,
}

impl From<&String> for Module {
    fn from(value: &String) -> Self {
        let (id, outputs) = value.split_once(" -> ").unwrap();
        let outputs = outputs.split(", ").map(Day20::hash).collect();
        match id.chars().next().unwrap() {
            'b' => Module {
                id: Day20::hash("broadcaster"),
                kind: Kind::Broadcaster,
                outputs,
            },
            '%' => Module {
                id: Day20::hash(&id[1..]),
                kind: Kind::FlipFlop(false),
                outputs,
            },
            '&' => Module {
                id: Day20::hash(&id[1..]),
                kind: Kind::Conjunction(HashMap::new()),
                outputs,
            },
            _ => unreachable!(),
        }
    }
}

impl Module {
    fn send(&self, pulses: &mut Pulses, value: bool) {
        self.outputs
            .iter()
            .for_each(|output| pulses.push_back((self.id, *output, value)));
    }
    pub fn pulse(&mut self, pulses: &mut Pulses, from: &u64, value: bool) {
        match &mut self.kind {
            Kind::Broadcaster => self.send(pulses, value),
            Kind::FlipFlop(state) => {
                if !value {
                    let inversion = !*state;
                    self.send(pulses, inversion);
                    self.kind = Kind::FlipFlop(inversion);
                }
            }
            Kind::Conjunction(inputs) => {
                inputs.insert(*from, value);
                let all_high = inputs.values().all(|b| *b);
                self.send(pulses, !all_high);
            }
            Kind::Rx(state) => {
                if !value {
                    self.kind = Kind::Rx(true)
                }
            }
        }
    }
}

pub type Modules = HashMap<u64, Module>;

pub struct Day20();

impl Day20 {
    fn hash(s: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
    pub fn parse1(input: &[String]) -> Modules {
        let mut modules: Modules = input.iter().map(Module::from).map(|m| (m.id, m)).collect();
        // Manually inject the rx module
        modules.insert(
            Self::hash("rx"),
            Module {
                id: Self::hash("rx"),
                kind: Kind::Rx(false),
                outputs: vec![],
            },
        );
        let output_map: Vec<(u64, Vec<u64>)> = modules
            .iter()
            .map(|(id, module)| (*id, module.outputs.clone()))
            .collect();
        output_map.iter().for_each(|(id, outputs)| {
            outputs.iter().for_each(|output| {
                if let Some(module) = modules.get_mut(output) {
                    if let Kind::Conjunction(inputs) = &mut module.kind {
                        inputs.insert(*id, false);
                    }
                }
            })
        });
        modules
    }
    pub fn parse2(input: &[String], without: &[u64]) -> Modules {
        let mut modules: Modules = input
            .iter()
            // .filter(|line| without.iter().all(|w| !line.starts_with(&format!("&{w}"))))
            .map(Module::from)
            .filter(|m| !without.contains(&m.id))
            .map(|m| (m.id, m))
            .collect();
        // Manually inject the rx module
        modules.insert(
            Day20::hash("rx"),
            Module {
                id: Day20::hash("rx"),
                kind: Kind::Rx(false),
                outputs: vec![],
            },
        );
        let output_map: Vec<(u64, Vec<u64>)> = modules
            .iter()
            .map(|(id, module)| (*id, module.outputs.clone()))
            .collect();
        output_map.iter().for_each(|(id, outputs)| {
            outputs.iter().for_each(|output| {
                if let Some(module) = modules.get_mut(output) {
                    if let Kind::Conjunction(inputs) = &mut module.kind {
                        inputs.insert(*id, false);
                    }
                }
            })
        });
        modules
    }
    pub fn count_pulses(modules: &mut Modules) -> (u64, u64) {
        let mut lows = 0;
        let mut highs = 0;
        let mut pulses: VecDeque<Pulse> =
            VecDeque::from([(Self::hash("button"), Self::hash("broadcaster"), false)]);
        while let Some(pulse) = pulses.pop_front() {
            if pulse.2 {
                highs += 1;
            } else {
                lows += 1;
            }
            if let Some(module) = modules.get_mut(&pulse.1) {
                module.pulse(&mut pulses, &pulse.0, pulse.2);
            }
        }
        (lows, highs)
    }
}

type Solution = u64;
impl DaySolver<Solution> for Day20 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let mut modules = Self::parse1(&input);
        let (mut lows, mut highs) = (0, 0);
        for _ in 0..1000 {
            let (l, h) = Self::count_pulses(&mut modules);
            lows += l;
            highs += h;
        }
        Some(lows * highs)
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        // Handy for debugging with https://csacademy.com/app/graph_editor/
        // for module in modules.values() {
        //     for output in module.outputs.iter() {
        //         println!("{} {}", module.id, output);
        //     }
        // }
        // This heavily relies on some insights from staring at the graph.
        // The graph consists of four subgraphs that are triggered by the broadcaster, then are
        // collected in a module that triggers rx.
        // Each submodule has a cycle of roughly 4000, but the collection hub only triggers when
        // the cycles align, which only happens after (the least common multiple of each cycle)
        // cycles.
        // We abuse this by identifying the exits of each node, deactivating all but one, and
        // running until rx is triggered.
        let mut modules = Self::parse1(&input);
        let hub = modules
            .values()
            .find(|module| module.outputs.iter().any(|o| *o == Self::hash("rx")))
            .unwrap();
        if let Kind::Conjunction(inputs) = &hub.kind {
            let mut lcm = 1;
            for chokepoint in inputs.keys() {
                let without: Vec<u64> = inputs
                    .keys()
                    .filter(|s| s != &chokepoint)
                    .copied()
                    .collect();
                let mut modified_modules = Self::parse2(&input, &without);
                let mut count = 0;
                while modified_modules.get(&Self::hash("rx")).unwrap().kind == Kind::Rx(false) {
                    Self::count_pulses(&mut modified_modules);
                    count += 1;
                }
                lcm = lcm.lcm(&count);
            }
            Some(lcm)
        } else {
            unreachable!()
        }
    }
}
