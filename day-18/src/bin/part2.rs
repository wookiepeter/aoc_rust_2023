use std::collections::{HashSet, VecDeque};

use aoc_util::direction::{self, Direction};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    // perform linesweep algorithm over to determine total contained area
    let moves = read_moves(input);

    let start_position: (i64, i64) = (0, 0);

    let mut lines = create_lines(start_position, moves.clone());
    // sort lines and put them into a VecDeque for easier handling during the line sweep algorithm
    lines.sort_by(|a, b| a.start.cmp(&b.start));
    let mut lines: VecDeque<Line> = VecDeque::from(lines);
    // gather all unique horizontal points used used in the lines and sort them
    let mut horizontal_points: HashSet<i64> = HashSet::new();
    lines.iter().for_each(|line| {
        horizontal_points.insert(line.start);
        horizontal_points.insert(line.end);
    });
    let mut horizontal_points: Vec<i64> = horizontal_points.into_iter().collect();
    horizontal_points.sort();
    let mut horizontal_points: VecDeque<i64> = VecDeque::from(horizontal_points);

    let mut active_lines: Vec<Line> = Vec::new();
    let mut previous_x = horizontal_points.pop_front().unwrap();
    while let Some(line) = lines.front() {
        if line.start > previous_x {
            break;
        }
        active_lines.push(lines.pop_front().unwrap());
    }

    let mut area = 0;

    while let Some(x) = horizontal_points.pop_front() {
        let h_dist = x - previous_x;

        // tuples of all active lines should represent rectangles making up the space
        // between the horizontal points and therefore the entire area inside the border
        active_lines.sort_by(|a, b| a.y_pos.cmp(&b.y_pos));
        for (a, b) in itertools::Itertools::tuples(active_lines.iter()) {
            area += h_dist * i64::abs_diff(a.y_pos, b.y_pos) as i64
        }

        // remove all active lines where the current end is the current x
        active_lines.retain(|line| line.end > x);
        // grab all new lines with the same start point
        while let Some(line) = lines.front() {
            if line.start > x {
                break;
            }
            active_lines.push(lines.pop_front().unwrap());
        }
        previous_x = x;
    }

    let border: i64 = moves.iter().map(|mv| mv.1).sum();

    // This is required because of [PICKS THEOREM](https://en.wikipedia.org/wiki/Pick%27s_theorem)
    // -> we count grid cells and not pure area -> need to include half the border + ...
    let result: i64 = area + border / 2 + 1;

    result.to_string()
}

fn create_lines(start_position: (i64, i64), moves: Vec<(Direction, i64)>) -> Vec<Line> {
    let mut position = start_position;
    let mut lines: Vec<Line> = vec![];

    for (direction, length) in moves {
        let old_position = position;
        let dir_raw: (i32, i32) = direction.into();
        let dir = ((dir_raw.0 as i64) * length, (dir_raw.1 as i64) * length);
        position = (position.0 + dir.0, position.1 + dir.1);

        match direction {
            Direction::Right => lines.push(Line {
                y_pos: old_position.1,
                start: old_position.0,
                end: position.0,
            }),
            Direction::Left => lines.push(Line {
                y_pos: old_position.1,
                start: position.0,
                end: old_position.0,
            }),
            _ => (),
        }
    }

    lines
}

#[derive(Clone, Copy)]
struct Line {
    y_pos: i64,
    start: i64,
    end: i64,
}

pub fn read_moves(input: &str) -> Vec<(Direction, i64)> {
    input
        .lines()
        .map(|line| {
            let vec: Vec<&str> = line.split('#').collect();
            let word = vec[1];
            let length = i64::from_str_radix(&word[0..5], 16).unwrap();

            let direction = match word.chars().nth(5).unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Case should never be reached"),
            };

            (direction, length)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
        );
        assert_eq!(result, "952408144115".to_string())
    }
}
