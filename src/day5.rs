#![allow(unused)]
use crate::util::DaySolver;

type Solution = u64;

pub struct Day5();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }
    /// Range ends are exclusive
    fn contains(&self, value: u64) -> bool {
        self.start <= value && value < self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MapRange {
    range: Range,
    dest: u64,
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.range.start.cmp(&other.range.start)
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
        let dest: u64 = split.next().unwrap().parse().unwrap();
        let source: u64 = split.next().unwrap().parse().unwrap();
        let width: u64 = split.next().unwrap().parse().unwrap();
        MapRange {
            range: Range::new(source, source + width),
            dest,
        }
    }
}

impl MapRange {
    fn apply(&self, value: u64) -> u64 {
        value + self.dest - self.range.start
    }
}

#[derive(Debug)]
pub struct Mapping {
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
            if range.range.contains(value) {
                return range.apply(value);
            }
        }
        value
    }
    fn apply_range(&self, range: &Range) -> Vec<Range> {
        let mut range: Range = *range;
        let mut applied_ranges = vec![];
        for map_range in &self.ranges {
            // Just completely skip any mapped ranges that end before our range starts.
            if map_range.range.end < range.start {
                continue;
            }
            // We have just encountered the first mapped range that is entirely after our range.
            if map_range.range.start >= range.end {
                break;
            }
            // We have found a mapped range that starts somewhere inside of our range.
            // The prefix of our range before the mapped range starts needs to be added.
            // Then we move the start of our range to the start of the mapped range.
            if range.start < map_range.range.start {
                applied_ranges.push(Range::new(range.start, map_range.range.start));
                range.start = map_range.range.start;
            }
            // Our range contains the end of the mapped range.
            // Map the range accordingly.
            if map_range.range.end < range.end {
                applied_ranges.push(Range::new(
                    map_range.apply(range.start),
                    map_range.apply(map_range.range.end),
                ));
                range.start = map_range.range.end;
            // Our range does not contain the end of the mapped range, which means our remaining
            // range must be completely contained by the mapped range.
            // Map the whole thing, then return; any more mapped ranges would be after our range.
            } else {
                applied_ranges.push(Range::new(
                    map_range.apply(range.start),
                    map_range.apply(range.end),
                ));
                return applied_ranges;
            }
        }
        // To get here, we must have had a remaining range that didn't end inside a mapped range.
        // Append it and return.
        applied_ranges.push(range);
        applied_ranges
    }
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Mapping>,
}

impl From<&Vec<String>> for Almanac {
    fn from(value: &Vec<String>) -> Self {
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
        let almanac: Almanac = (&input).into();
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
        let almanac: Almanac = (&input).into();
        let mut ranges = vec![];
        let mut i = almanac.seeds.iter();
        while let Some(start) = i.next() {
            let width = i.next().unwrap();
            ranges.push(Range::new(*start, start + width));
        }
        for mapping in &almanac.maps {
            let mut new_ranges = vec![];
            for range in &ranges {
                new_ranges.append(&mut mapping.apply_range(range));
            }
            ranges = new_ranges;
        }
        Some(ranges.iter().map(|r| r.start).min().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_apply_range_mapping_contained() {
        let mapping = Mapping {
            ranges: vec![MapRange::from(&"500 5 1".to_string())],
        };
        assert_eq!(
            mapping.apply_range(&Range::new(0, 10)),
            vec![Range::new(0, 5), Range::new(500, 501), Range::new(6, 10)]
        );
    }
    #[test]
    fn test_apply_range_mapping_contains() {
        let mapping = Mapping {
            ranges: vec![MapRange::from(&"1000 0 100".to_string())],
        };
        assert_eq!(
            mapping.apply_range(&Range::new(10, 20)),
            vec![Range::new(1010, 1020)]
        );
        assert_eq!(
            mapping.apply_range(&Range::new(0, 20)),
            vec![Range::new(1000, 1020)]
        );
        assert_eq!(
            mapping.apply_range(&Range::new(0, 100)),
            vec![Range::new(1000, 1100)]
        );
    }
}
