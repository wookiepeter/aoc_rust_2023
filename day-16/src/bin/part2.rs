use std::collections::{HashMap, HashSet};

use aoc_util::usize_point::Point;
use day_16::{process_beam, MapData, MirrorStatus};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let char_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map_size = (char_map[0].len(), char_map.len());

    let map_data = MapData {
        chars: char_map.clone(),
        size: map_size,
    };

    let start_settings: Vec<(Point, (i32, i32))> = std::iter::empty()
        .chain((0..map_size.0).map(|x| ((x, 0), (0, 1))))
        .chain((0..map_size.0).map(|x| ((x, map_size.1 - 1), (0, -1))))
        .chain((0..map_size.1).map(|y| ((0, y), (1, 0))))
        .chain((0..map_size.1).map(|y| ((map_size.0 - 1, y), (-1, 0))))
        .collect();

    let mut n_energized: Vec<usize> = start_settings
        .iter()
        .map(|(position, direction)| process_setting(*position, *direction, &map_data))
        .collect();

    n_energized.sort();
    n_energized.last().unwrap().to_string()
}

fn process_setting(position: Point, direction: (i32, i32), map_data: &MapData) -> usize {
    let mut mirror_map: HashMap<(usize, usize), MirrorStatus> = HashMap::new();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();

    process_beam(
        position,
        direction,
        map_data,
        &mut energized,
        &mut mirror_map,
    );

    energized.len()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        );
        assert_eq!(result, "51".to_string())
    }
}
