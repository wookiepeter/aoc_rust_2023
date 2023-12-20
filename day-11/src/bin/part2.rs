use std::collections::HashSet;

use aoc_util::find_chars_positions;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    // use 999999 instead of 1000000 since we don't actually replace the lines / columns
    let mapped_positions = get_mapped_positions(input, 1000000 - 1);

    // permutations does include the reverse cases -> to eliminate these duplicate cases we simply half the result!
    let result = mapped_positions
        .iter()
        .permutations(2)
        .map(|vec| aoc_util::manhattan_dist_u64(vec[0], vec[1]))
        .sum::<u64>();

    (result / 2).to_string()
}

pub fn get_mapped_positions(input: &str, expand_dist: u64) -> Vec<(u64, u64)> {
    let empty_lines: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| match !line.contains('#') {
            true => Some(index),
            false => None,
        })
        .collect();

    let line_len = input.lines().next().unwrap().len();
    let col_hashset: HashSet<usize> = input
        .lines()
        .flat_map(|line| {
            line.char_indices().filter_map(|(index, c)| match c {
                '#' => Some(index),
                _ => None,
            })
        })
        .collect();
    let empty_columns: Vec<usize> = (0..line_len)
        .filter(|index| !col_hashset.contains(index))
        .rev()
        .collect();

    let char_positions = find_chars_positions(input, |c| c == '#');

    char_positions
        .iter()
        .map(|(x, y)| {
            let prev_columns = empty_columns.iter().filter(|column| *column < x).count();
            let prev_rows = empty_lines.iter().filter(|line| *line < y).count();

            (
                *x as u64 + prev_columns as u64 * expand_dist,
                *y as u64 + prev_rows as u64 * expand_dist,
            )
        })
        .collect::<Vec<(u64, u64)>>()
}

#[cfg(test)]
mod tests {
    use crate::{get_mapped_positions, process};

    #[test]
    fn basic_example() {
        let result = get_mapped_positions(
            "..#
...
#..",
            10u64,
        );
        assert_eq!(result, vec![(12, 0), (0, 12)]);
    }
}
