use std::collections::HashSet;

use day_10::{connected_neighbors, direct_neighbors, Map2D, NeigborNode};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut map = day_10::Map2D::from(input, |c| c);
    let start_pos = map.find_element('S').unwrap();

    // transform S on map into it's actual pipes?!
    // find the starting neighbors
    let neighbors: Vec<(usize, usize)> = direct_neighbors(start_pos)
        .into_iter()
        .filter(|pos| map.is_inbound(pos))
        .filter(|pos| {
            connected_neighbors(pos, map.get_element(pos).unwrap())
                .iter()
                .any(|neighbor| *neighbor == start_pos)
        })
        .collect();

    let start_char = reverse_connected_neighbors(start_pos, neighbors);
    map.set_element(&start_pos, start_char);

    let neighbor_map = day_10::NeigborNode::create_neighbor_map(&map);

    let unreachable_neighbors = find_unreachable_fields(&neighbor_map);

    let all_positions: Vec<(usize, usize)> = (0..map.size.1)
        .flat_map(|y| (0..map.size.0).zip((y..y + 1).cycle()))
        .collect();

    all_positions
        .iter()
        .filter(|pos| {
            let p = (pos.0 * 2, pos.1 * 2);
            let mapped_positions = vec![p, (p.0 + 1, p.1), (p.0, p.1 + 1), (p.0 + 1, p.1 + 1)];
            mapped_positions
                .iter()
                .all(|pos| unreachable_neighbors.contains(pos))
        })
        .count()
        .to_string()
}

fn find_unreachable_fields(map: &Map2D<NeigborNode>) -> HashSet<(usize, usize)> {
    let mut result: HashSet<(usize, usize)> = (0..map.size.1)
        .flat_map(|y| (0..map.size.0).zip((y..y + 1).cycle()))
        .collect();

    let border_positions: Vec<(usize, usize)> = ((0..1).cycle().zip(0..map.size.1))
        .chain((0..map.size.0).zip((0..1).cycle()))
        .chain((0..map.size.0).zip(((map.size.1 - 1)..map.size.1).cycle()))
        .chain(((map.size.0 - 1)..map.size.0).cycle().zip(0..map.size.1))
        .collect();

    border_positions.into_iter().for_each(|pos| {
        if result.contains(&pos) {
            let mut nodes_to_check = vec![pos];

            while !nodes_to_check.is_empty() {
                nodes_to_check = nodes_to_check
                    .iter()
                    .flat_map(|node_pos| {
                        map.get_element(node_pos)
                            .unwrap()
                            .neighbors
                            .clone()
                            .into_iter()
                    })
                    .filter(|node_pos| result.contains(node_pos))
                    .collect();

                nodes_to_check.retain(|node_pos| match result.contains(node_pos) {
                    true => {
                        result.remove(node_pos);
                        true
                    }
                    false => false,
                })
            }
        }
    });

    result
}

fn reverse_connected_neighbors(start_pos: (usize, usize), neighbors: Vec<(usize, usize)>) -> char {
    let relative_positions: Vec<(i32, i32)> = neighbors
        .iter()
        .map(|neighbor| {
            (
                neighbor.0 as i32 - start_pos.0 as i32,
                neighbor.1 as i32 - start_pos.1 as i32,
            )
        })
        .collect();

    match relative_positions[..] {
        [(-1, 0), (1, 0)] => '-',
        [(0, -1), (0, 1)] => '|',
        [(-1, 0), (0, -1)] => 'J',
        [(-1, 0), (0, 1)] => '7',
        [(0, -1), (1, 0)] => 'L',
        [(0, 1), (1, 0)] => 'F',
        _ => ' ',
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn super_basic_example() {
        let result = process(
            "F-7
|.|
S-J",
        );
        assert_eq!(result, "1".to_string());
    }

    #[test]
    fn test_simple_example() {
        let result = process(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, "4".to_string())
    }

    #[test]
    fn test_example() {
        let result = process(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, "8".to_string())
    }

    #[test]
    fn test_complex_example() {
        let result = process(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, "10".to_string())
    }
}
