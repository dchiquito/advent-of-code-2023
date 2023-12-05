#![allow(unused)]
use crate::util::DaySolver;

type Solution = u64;

pub struct Day5();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MapRange {
    source: u64,
    dest: u64,
    range: u64,
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source.cmp(&other.source)
    }
}
impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl From<&String> for MapRange {
    fn from(value: &String) -> Self {
        let mut split = value.split(' ');
        let dest = split.next().unwrap().parse().unwrap();
        let source = split.next().unwrap().parse().unwrap();
        let range = split.next().unwrap().parse().unwrap();
        MapRange {
            source,
            dest,
            range,
        }
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MapRange>,
}

impl From<&[String]> for Mapping {
    fn from(value: &[String]) -> Self {
        let ranges = value.iter().map(MapRange::from).collect();
        Mapping { ranges }
    }
}

impl Mapping {
    fn apply(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if range.source <= value && value <= range.source + range.range {
                return value + range.dest - range.source;
            }
        }
        value
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Mapping>,
}

impl From<Vec<String>> for Almanac {
    fn from(value: Vec<String>) -> Self {
        let seeds = value[0]
            .strip_prefix("seeds: ")
            .unwrap()
            .split(' ')
            .map(|s| s.parse().unwrap())
            .collect();
        let mut maps = Vec::with_capacity(7);
        let mut mapping = Mapping { ranges: vec![] };
        let mut start = 3;
        let mut end = 3;
        while end < value.len() {
            if value[end].is_empty() {
                mapping.ranges.sort();
                maps.push(mapping);
                mapping = Mapping { ranges: vec![] };
                start = end + 2;
                end = start;
            } else {
                mapping.ranges.push(MapRange::from(&value[end]));
                end += 1;
            }
        }
        mapping.ranges.sort();
        maps.push(mapping);
        Almanac { seeds, maps }
    }
}

impl Almanac {
    fn apply(&self, value: u64) -> u64 {
        self.maps.iter().fold(value, |v, mapping| mapping.apply(v))
    }
}

impl Day5 {}

impl DaySolver<Solution> for Day5 {
    fn part1(input: Vec<String>) -> Option<Solution> {
        let almanac: Almanac = input.into();
        Some(
            almanac
                .seeds
                .iter()
                .map(|s| almanac.apply(*s))
                .min()
                .unwrap(),
        )
    }
    fn part2(input: Vec<String>) -> Option<Solution> {
        None
    }
}
