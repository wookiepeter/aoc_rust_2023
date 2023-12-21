use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let expanded_input = add_expanded_space(input);
    println!("{}", expanded_input.as_str());

    let positions = aoc_util::find_chars_positions(&expanded_input, |c| c == '#');

    // permutations does include the reverse cases -> to eliminate these duplicate cases we simply half the result!
    let result = positions
        .iter()
        .permutations(2)
        .map(|vec: Vec<&(usize, usize)>| aoc_util::manhattan_dist(vec[0], vec[1]))
        .sum::<usize>();

    (result / 2).to_string()
}

fn add_expanded_space(input: &str) -> String {
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
    let missing_columns: Vec<usize> = (0..line_len)
        .filter(|index| !col_hashset.contains(index))
        .rev()
        .collect();

    // add an extra column at each empty column
    let mut result: Vec<String> = input
        .lines()
        .map(|line| {
            let mut line = String::from(line);
            for i in missing_columns.iter() {
                line.insert(*i, '.')
            }
            line
        })
        .collect();

    // add an extra line at each empty line
    let result_len = result.first().unwrap().len();
    let empty_line = vec!['.'; result_len].iter().collect::<String>();

    for row in empty_lines.iter().rev() {
        result.insert(*row, empty_line.clone());
    }
    dbg!(&result);
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374".to_string())
    }

    #[test]
    fn test_short_dist() {
        let result = process(
            ".#
#.",
        );
        assert_eq!(result, "2".to_string())
    }
}
