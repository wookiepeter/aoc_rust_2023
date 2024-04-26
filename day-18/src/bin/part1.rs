use std::collections::{HashSet, VecDeque};

use aoc_util::{
    direction::*,
    string_grid::GridInfo,
    usize_point::{point_add, Point},
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let moves: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let vec: Vec<&str> = line.split_whitespace().collect();

            let direction = match vec[0] {
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                "L" => Direction::Left,
                other => panic!("Direction {} was not recognized", other),
            };

            let steps = vec[1].parse::<usize>().unwrap();
            (direction, steps)
        })
        .collect();

    let (border_positions, origin, bounds) = compute_border_positions(moves);

    let filled_positions = find_inside_positions(origin, bounds, &border_positions);

    /*     let border_vec = border_positions.clone().into_iter().collect();
    let inside_vec = filled_positions.clone().into_iter().collect();

    let info = GridInfo {
        default_char: '.',
        dimensions: bounds,
        other_chars: vec![('#', border_vec), ('$', inside_vec)],
    };

    println!("{}", info.create_grid()); */

    (filled_positions.len() + border_positions.len()).to_string()
}

fn find_inside_positions(
    origin: (usize, usize),
    bounds: (usize, usize),
    border_positions: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let inside_positions = [
        point_add(origin, (-1, -1), bounds),
        point_add(origin, (1, -1), bounds),
        point_add(origin, (1, 1), bounds),
        point_add(origin, (-1, 1), bounds),
    ];
    let inside_positions: Vec<Point> = inside_positions.iter().filter_map(|pos| *pos).collect();

    let mut filled_positions: HashSet<Point> = HashSet::new();
    for initial_position in inside_positions {
        // flood fill (using priority queue)
        let mut queue: VecDeque<Point> = VecDeque::new();
        filled_positions.clear();

        filled_positions.insert(initial_position);
        queue.push_back(initial_position);

        while let Some(position) = queue.pop_front() {
            let neighbors = get_direct_neighbors(&position, &bounds);
            // if neighbors contains a value outside of the bounds -> this isn't the inner bounds
            // discard this set and move on
            if neighbors.contains(&None) {
                break;
            }

            let filtered_neighbors: Vec<Option<Point>> = neighbors
                .into_iter()
                .filter(|neighbor| {
                    !border_positions.contains(&neighbor.unwrap())
                        && !filled_positions.contains(&neighbor.unwrap())
                })
                .collect();

            filtered_neighbors.iter().for_each(|neighbor| {
                filled_positions.insert(neighbor.unwrap());
                queue.push_back(neighbor.unwrap());
            });

            if queue.is_empty() {
                return filled_positions;
            }
        }
    }
    filled_positions
}

pub fn get_direct_neighbors(position: &Point, bounds: &Point) -> Vec<Option<Point>> {
    // this turn elements below the inner bounds (0, 0) to None
    let result = vec![
        position.0.checked_sub(1).zip(Some(position.1)),
        Some(position.0).zip(position.1.checked_sub(1)),
        position.0.checked_add(1).zip(Some(position.1)),
        Some(position.0).zip(position.1.checked_add(1)),
    ];

    // turn elements over the outer bounds to None aswell
    result
        .iter()
        .map(|elem| match elem {
            Some(point) if point.0 > bounds.0 || point.1 > bounds.1 => None,
            _ => *elem,
        })
        .collect()
}

/// returns a tuple containing of:
/// - the border positions as usize Points (therefore translated),
/// - origin point
/// - upper bounds of the points
pub fn compute_border_positions(moves: Vec<(Direction, usize)>) -> (HashSet<Point>, Point, Point) {
    let mut position: (i32, i32) = (0, 0);
    let mut set = HashSet::new();

    set.insert(position);

    for (dir, steps) in moves {
        for _ in 0..steps {
            let dir: (i32, i32) = dir.into();
            position = (position.0 + dir.0, position.1 + dir.1);
            set.insert(position);
        }
    }

    // translate borders to inside usize bounds
    let mut high_x = 0;
    let mut low_x = 0;
    let mut high_y = 0;
    let mut low_y = 0;
    for (x, y) in set.iter() {
        high_x = high_x.max(*x);
        low_x = low_x.min(*x);
        high_y = high_y.max(*y);
        low_y = low_y.min(*y);
    }

    let translation = (-low_x, -low_y);
    let result: HashSet<Point> = set
        .iter()
        .map(|i_point| {
            (
                (i_point.0 + translation.0) as usize,
                (i_point.1 + translation.1) as usize,
            )
        })
        .collect();

    (
        result,
        (translation.0 as usize, translation.1 as usize),
        (
            (high_x + translation.0 + 1) as usize,
            (high_y + translation.1 + 1) as usize,
        ),
    )
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
        assert_eq!(result, "62".to_string())
    }
}
