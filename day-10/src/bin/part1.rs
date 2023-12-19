use std::collections::HashMap;

use day_10::{connected_neighbors, direct_neighbors};

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let map = day_10::Map2D::from(input, |c| c);
    let start_pos = map.find_element('S').unwrap();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    visited.insert(start_pos, 0);

    // find the starting neighbors
    let mut neighbors: Vec<(usize, usize)> = direct_neighbors(start_pos)
        .into_iter()
        .filter(|pos| map.is_inbound(pos))
        .filter(|pos| {
            connected_neighbors(pos, map.get_element(pos).unwrap())
                .iter()
                .any(|neighbor| *neighbor == start_pos)
        })
        .collect();
    let mut step_count = 1usize;
    neighbors
        .iter()
        .for_each(|neighbor| _ = visited.insert(*neighbor, step_count));

    while !neighbors.is_empty() {
        neighbors = neighbors
            .iter()
            .filter_map(|pos| map.get_element(pos).map(|c| connected_neighbors(pos, c)))
            .flatten()
            .filter(|pos| !visited.contains_key(pos))
            .collect();

        step_count += 1;
        neighbors
            .iter()
            .for_each(|neighbor| _ = visited.insert(*neighbor, step_count))
    }

    visited.values().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_square_loop() {
        let result = process(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, "4".to_string())
    }

    #[test]
    fn test_extended_square_loop() {
        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8".to_string())
    }
}
