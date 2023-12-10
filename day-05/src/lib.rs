use std::{ops::Range, vec};

use itertools::Itertools;

pub fn parse_seeds(line: &str) -> Vec<i64> {
    line.trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

pub fn parse_seed_map(lines: Vec<&str>) -> SeedMap {
    let mut iter = lines.iter();

    dbg!(&lines);

    let (from, to) = iter
        .next()
        .unwrap()
        .trim_end_matches(" map:")
        .split_once("-to-")
        .unwrap();

    let maps: Vec<RangeMap> = iter
        .map(|line| {
            if let [dest_start, source_start, length] = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()[..]
            {
                RangeMap::new(
                    source_start..source_start + length,
                    dest_start - source_start,
                )
            } else {
                panic!("RangeMap could not be created for {from} to {to}");
            }
        })
        .collect();

    SeedMap {
        from: String::from(from),
        to: String::from(to),
        maps,
    }
}

#[derive(PartialEq, Debug)]
pub struct SeedMap {
    from: String,
    to: String,
    maps: Vec<RangeMap>,
}

impl SeedMap {
    pub fn process(&self, seed: i64) -> i64 {
        match self.maps.iter().find(|map| map.in_range(seed)) {
            Some(map) => map.process(seed),
            _ => seed,
        }
    }

    pub fn process_range(&self, seed_range: &Range<i64>) -> Vec<Range<i64>> {
        let mut result = vec![];
        let mut remaining_ranges: Vec<Range<i64>> = vec![seed_range.clone()];

        for map in self.maps.iter() {
            let temp_ranges = remaining_ranges.clone();
            remaining_ranges.clear();
            for range in temp_ranges.iter() {
                let (mapped, mut remaining) = map.process_range(range);
                if let Some(mapped_range) = mapped {
                    result.push(mapped_range);
                }
                remaining_ranges.append(&mut remaining);
            }
        }

        result.append(&mut remaining_ranges);
        result = combine_ranges(result);

        result
    }
}

pub fn combine_ranges(ranges: Vec<Range<i64>>) -> Vec<Range<i64>> {
    let mut result: Vec<Range<i64>> = vec![];
    let mut current_range: Option<Range<i64>> = None;

    for range in ranges.iter().sorted_by_key(|range| range.start) {
        current_range = match &current_range {
            // next range starts in current range -> should be combined
            Some(cur) if range.start < cur.end => Some(cur.start..i64::max(cur.end, range.end)),
            // start of a new range -> push old range
            Some(cur) => {
                result.push(cur.clone());
                Some(range.clone())
            }
            None => Some(range.clone()),
        }
    }

    result.push(current_range.unwrap());

    result
}

#[derive(PartialEq, Eq, Debug)]
pub struct RangeMap {
    source_range: Range<i64>,
    dif: i64,
}

impl RangeMap {
    fn new(source_range: Range<i64>, dif: i64) -> RangeMap {
        RangeMap { source_range, dif }
    }

    pub fn in_range(&self, value: i64) -> bool {
        self.source_range.contains(&value)
    }

    pub fn process(&self, value: i64) -> i64 {
        value + self.dif
    }

    // Returns a tuple with the processed range (should go straight to output) and the remaining ranges
    pub fn process_range(&self, range: &Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
        if self.source_range.contains(&range.start) && self.source_range.contains(&range.end) {
            // range is completely of source range -> everything get's modified
            (Some(range.start + self.dif..range.end + self.dif), vec![])
        } else if range.start < self.source_range.start && self.source_range.contains(&range.end) {
            // range starts before source range and ends in source range -> 1st vec unmodified, 2nd vec modified
            (
                Some(self.source_range.start + self.dif..range.end + self.dif),
                vec![range.start..self.source_range.start],
            )
        } else if range.contains(&self.source_range.start) && range.contains(&self.source_range.end)
        {
            // range contains the source range -> 1st vec unmodified, 2nd vec modified, 3rd vec unmodified
            (
                Some(self.source_range.start + self.dif..self.source_range.end + self.dif),
                vec![
                    range.start..self.source_range.start,
                    self.source_range.end..range.end,
                ],
            )
        } else if self.source_range.contains(&range.start) && range.end >= self.source_range.end {
            // range starts in source range and ends after source range -> 1st vec modiifed
            (
                Some(range.start + self.dif..self.source_range.end + self.dif),
                vec![self.source_range.end..range.end],
            )
        } else {
            // range is outside of the source_range -> unmodified
            (None, vec![range.clone()])
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::{format, Debug},
        ops::Range,
    };

    use crate::{RangeMap, SeedMap};

    #[test]
    fn read_seed_list() {
        assert_eq!(
            format!("{:?}", crate::parse_seeds("seeds: 79 14 55 13")),
            format!("{:?}", vec![79, 14, 55, 13])
        )
    }

    #[test]
    fn test_seed_map() {
        let test_map = crate::parse_seed_map(
            "seed-to-soil map:
50 98 2
52 50 48"
                .lines()
                .collect(),
        );

        let comparison_map = SeedMap {
            from: "seed".to_string(),
            to: "soil".to_string(),
            maps: vec![RangeMap::new(98..100, -48), RangeMap::new(50..98, 2)],
        };

        assert_eq!(format!("{:?}", test_map), format!("{:?}", comparison_map))
    }

    #[test]
    fn test_process_range() {
        let map = RangeMap::new(5..10, -3);

        // test cases
        debug_equals(map.process_range(&(5..9)), (Some(2..6), vec![]))
        /*
        range_vec_equals(map.process_range(&(1..3)), vec![1..3]);
        range_vec_equals(map.process_range(&(1..8)), vec![1..5, 8..11]);
        range_vec_equals(map.process_range(&(5..8)), vec![8..11]);
        range_vec_equals(map.process_range(&(9..13)), vec![12..13, 10..13]);
        range_vec_equals(map.process_range(&(3..15)), vec![3..5, 8..13, 10..15]);
        */
    }

    fn debug_equals<D>(lhs: D, rhs: D)
    where
        D: Debug,
    {
        assert_eq!(format!("{:?}", lhs), format!("{:?}", rhs))
    }
}
