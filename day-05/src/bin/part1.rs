use day_05::{RangeMap, SeedMap};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut lines = input.lines().peekable();

    let seed_list = day_05::parse_seeds(lines.next().unwrap());

    let mut seed_maps: Vec<SeedMap> = vec![];

    let _ = lines.next();

    while lines.peek().is_some() {
        let map_string = lines.by_ref().take_while(|line| !line.is_empty()).collect();

        seed_maps.push(day_05::parse_seed_map(map_string));
    }

    let mut mapped_seeds: Vec<i64> = seed_list
        .iter()
        .map(|seed| {
            seed_maps
                .iter()
                .fold(*seed, |cur_seed, map| map.process(cur_seed))
        })
        .collect();

    mapped_seeds.sort();

    mapped_seeds.first().unwrap().to_string()
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
        assert_eq!(result, "35".to_string())
    }
}
