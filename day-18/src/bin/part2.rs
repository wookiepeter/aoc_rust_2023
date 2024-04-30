use std::collections::{HashSet, VecDeque};

use aoc_util::direction::{self, Direction};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let moves = read_moves(input);

    let start_position: (i64, i64) = (0, 0);
    let mut lines = create_lines(start_position, moves);
    lines.sort_by(|a, b| a.start.cmp(&b.start));
    let mut lines: VecDeque<Line> = VecDeque::from(value);
    let mut horizontal_points: HashSet<i64> = HashSet::new();
    lines.iter().for_each(|line| {
        horizontal_points.insert(line.start);
        horizontal_points.insert(line.end);
    });
    let mut horizontal_points: Vec<i64> = horizontal_points.into_iter().collect();
    horizontal_points.sort();
    let mut horizontal_points: VecDeque<i64> = VecDeque::from(horizontal_points);

    let mut prev_point = horizontal_points.pop_front().unwrap();

    while let Some(x) = horizontal_points.pop_front() {}

    let mut area = 0_i64;

    input.to_string()
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
                end: position.1,
            }),
            _ => (),
        }
    }

    lines
}

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
            let direction = match (word.get(5).unwrap()) {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
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
        let result = process("");
        assert_eq!(result, "4".to_string())
    }
}
