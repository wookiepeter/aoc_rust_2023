use std::collections::HashSet;

use aoc_util::{direction::Direction, grid::Grid};

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let steps: usize = 64;
    let result = simulate_steps(input, steps);

    result.to_string()
}

fn simulate_steps(input: &str, steps: usize) -> usize {
    let grid = Grid::<char>::new_char_grid(input);

    let binding = grid.find_positions(&'S');
    let starting_point = binding[0];

    let mut active_positions: Vec<(usize, usize)> = vec![starting_point];
    let mut future_positions: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..steps {
        active_positions.into_iter().for_each(|position| {
            DIRECTIONS
                .into_iter()
                .map(|dir| grid.get_direct_neighbor(position, dir))
                .for_each(|neighbor| match neighbor {
                    Some((position, c)) if !c.eq(&'#') => {
                        future_positions.insert(position);
                    }
                    _ => (),
                });
        });

        active_positions = future_positions.clone().into_iter().collect();
        future_positions.clear();
    }

    active_positions.len()
}

#[cfg(test)]
mod tests {
    use crate::{process, simulate_steps};

    #[test]
    fn test_example() {
        let result = simulate_steps(
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
            6,
        );
        assert_eq!(result, 16)
    }
}
