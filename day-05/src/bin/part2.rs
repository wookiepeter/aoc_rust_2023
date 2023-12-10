use std::{fmt::Debug, ops::Range, thread::current};

use day_05::SeedMap;
use itertools::{self, Itertools};
fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = input.lines().peekable();

    let seed_list = day_05::parse_seeds(lines.next().unwrap());
    let seed_ranges: Vec<Range<i64>> = seed_list
        .iter()
        .tuples::<(_, _)>()
        .map(|(start, length)| *start..start + length)
        .collect();

    let seed_ranges = day_05::combine_ranges(seed_ranges);

    let _ = lines.next();
    let mut seed_maps: Vec<SeedMap> = vec![];

    while lines.peek().is_some() {
        let map_string = lines.by_ref().take_while(|line| !line.is_empty()).collect();

        seed_maps.push(day_05::parse_seed_map(map_string));
    }

    let mut result = seed_ranges.clone();
    println!("{:?}", result);

    for map in seed_maps.iter() {
        result = result
            .iter()
            .flat_map(|range| map.process_range(range))
            .collect();

        result = day_05::combine_ranges(result);
    }

    result
        .iter()
        .sorted_by_key(|range| range.start)
        .next()
        .unwrap()
        .start
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "46".to_string())
    }
}
