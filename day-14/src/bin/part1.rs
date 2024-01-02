use std::collections::{HashMap, HashSet};

use day_14::Rock;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut cubic_rocks: Vec<(usize, usize)> = aoc_util::find_chars_positions(input, |c| c == '#');

    cubic_rocks.sort_by_key(|(_, y)| *y);

    let line_len = input.lines().next().unwrap().len();
    let line_count = input.lines().count();
    let mut rock_map: Vec<Vec<(usize, Rock)>> = vec![Vec::new(); line_len];

    cubic_rocks
        .iter()
        .for_each(|(x, y)| rock_map[*x].push((*y, Rock::Cube)));

    let mut round_rocks: Vec<(usize, usize)> = aoc_util::find_chars_positions(input, |c| c == 'O');
    round_rocks.sort_by_key(|(_, y)| *y);

    round_rocks.iter().for_each(|(x, y)| {
        // filter all rocks below y
        let blocking_rock = rock_map[*x]
            .iter()
            .enumerate()
            .filter(|(_, (cur_y, _))| *y > *cur_y)
            .last()
            // rock needs to be dereferenced and it's types need to be copy to avoid an immutable borrow
            .map(|(index, rock)| (index, *rock));

        // if there is none -> insert at index 0
        // otherwise insert after the last rock that is below y
        match blocking_rock {
            Some((index, prev_rock)) => {
                rock_map[*x].insert(index + 1, (prev_rock.0 + 1, Rock::Round))
            }
            None => rock_map[*x].insert(0, (0, Rock::Round)),
        }
    });

    rock_map
        .iter()
        .flatten()
        .filter(|(_, rock_type)| *rock_type == Rock::Round)
        .map(|(line, _)| line_count - line)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, "136".to_string())
    }
}
