use std::{fmt::Debug, vec};

#[derive(Debug)]
pub struct Map2D<T> {
    pub vec: Vec<Vec<T>>,
    pub size: (usize, usize),
}

impl<T: Debug> Map2D<T> {
    pub fn from(map_str: &str, map_element: fn(char) -> T) -> Map2D<T> {
        let vec: Vec<Vec<T>> = map_str
            .lines()
            .map(|line| line.chars().map(map_element).collect())
            .collect();

        let size = (vec[0].len(), vec.len());
        Map2D { vec, size }
    }

    pub fn set_element(&mut self, position: &(usize, usize), element: T) {
        self.vec[position.1][position.0] = element;
    }

    pub fn get_element(&self, position: &(usize, usize)) -> Option<&T> {
        if self.is_inbound(position) {
            Some(&self.vec[position.1][position.0])
        } else {
            None
        }
    }

    pub fn is_inbound(&self, position: &(usize, usize)) -> bool {
        position.0 < self.size.0 && position.1 < self.size.1
    }

    pub fn is_inbound_int(&self, position: &(i32, i32)) -> bool {
        position.0 >= 0
            && position.0 < (self.size.0 as i32)
            && position.1 >= 0
            && position.1 < (self.size.1 as i32)
    }
}

impl<T: PartialEq> Map2D<T> {
    pub fn find_element(&self, elem: T) -> Option<(usize, usize)> {
        self.vec
            .iter()
            .enumerate()
            .find_map(|(index, line)| line.iter().position(|c| elem.eq(c)).map(|pos| (pos, index)))
    }
}

// returns all direct neighbors (left, top, right, bottom) if they are valid (bigger than 0)
pub fn direct_neighbors(position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if position.0 > 0 {
        result.push((position.0 - 1, position.1))
    }
    if position.1 > 0 {
        result.push((position.0, position.1 - 1))
    }
    result.push((position.0, position.1 + 1));
    result.push((position.0 + 1, position.1));
    result
}

// filters the neighbors based on the symbol of this cell
pub fn connected_neighbors(position: &(usize, usize), c: &char) -> Vec<(usize, usize)> {
    let left = usize::checked_sub(position.0, 1).map(|x_pos| (x_pos, position.1));
    let right = Some((position.0 + 1, position.1));
    let top = usize::checked_sub(position.1, 1).map(|y_pos| (position.0, y_pos));
    let bottom = Some((position.0, position.1 + 1));

    let result = match c {
        '|' => vec![top, bottom],
        '-' => vec![left, right],
        'L' => vec![top, right],
        'J' => vec![left, top],
        '7' => vec![left, bottom],
        'F' => vec![bottom, right],
        _ => vec![],
    };

    result.iter().filter_map(|val| *val).collect()
}

#[derive(Clone, Debug)]
pub struct NeigborNode {
    pub neighbors: Vec<(usize, usize)>,
}

impl NeigborNode {
    // Provides 4 vecs that each contain a list of their relative direct neigbors.
    // These neigbors are in local coordinates and need to be transformed to global
    // coordinates before putting them into a map
    // List is ordered right-to-left then top-to-bottom
    pub fn create_relative_neighbors(c: &char) -> Vec<Vec<(i32, i32)>> {
        let left = (-1, 0);
        let up = (0, -1);
        let down = (0, 1);
        let right = (1, 0);

        let all_neighbors = vec![left, up, down, right];
        let all_up = vec![left, up, right];
        let all_left = vec![left, up, down];
        let all_down = vec![left, down, right];
        let all_right = vec![up, right, down];

        match c {
            '|' => vec![
                all_left.clone(),
                all_right.clone(),
                all_left.clone(),
                all_right.clone(),
            ],
            '-' => vec![
                all_up.clone(),
                all_up.clone(),
                all_down.clone(),
                all_down.clone(),
            ],
            'L' => vec![
                all_left.clone(),
                vec![up, right],
                all_neighbors.clone(),
                all_down.clone(),
            ],
            'J' => vec![
                vec![left, up],
                all_right,
                all_down.clone(),
                all_neighbors.clone(),
            ],
            '7' => vec![
                all_up.clone(),
                all_neighbors.clone(),
                vec![left, down],
                all_right.clone(),
            ],
            'F' => vec![
                all_neighbors.clone(),
                all_up.clone(),
                all_left.clone(),
                vec![down, right],
            ],
            _ => vec![all_neighbors.clone(); 4],
        }
    }

    pub fn empty() -> NeigborNode {
        NeigborNode { neighbors: vec![] }
    }

    pub fn create_neighbor_map(char_map: &Map2D<char>) -> Map2D<NeigborNode> {
        let size = (char_map.size.0 * 2, char_map.size.1 * 2);
        let mut result: Map2D<NeigborNode> = Map2D {
            vec: vec![vec![NeigborNode::empty(); size.0]; size.1],
            size,
        };

        // iterate through all nodes and get their relative neighbors
        for x in 0..char_map.size.0 {
            for y in 0..char_map.size.1 {
                let mapped_pos = (x * 2, y * 2);
                let c = char_map.get_element(&(x, y)).unwrap();

                fill_map(mapped_pos, c, &mut result);
            }
        }
        result
    }
}

fn fill_map(mapped_pos: (usize, usize), c: &char, result: &mut Map2D<NeigborNode>) {
    let positions = vec![
        (mapped_pos.0, mapped_pos.1),
        (mapped_pos.0 + 1, mapped_pos.1),
        (mapped_pos.0, mapped_pos.1 + 1),
        (mapped_pos.0 + 1, mapped_pos.1 + 1),
    ];

    positions
        .iter()
        .zip(NeigborNode::create_relative_neighbors(c))
        .for_each(|(pos, relative_neighbors)| {
            let connected_neigbors = relative_neighbors
                .iter()
                .filter_map(|relative| {
                    let neighbor_pos =
                        ((pos.0 as i32) + (relative.0), (pos.1 as i32) + (relative.1));

                    match result.is_inbound_int(&neighbor_pos) {
                        true => Some((neighbor_pos.0 as usize, neighbor_pos.1 as usize)),
                        false => None,
                    }
                })
                .collect();

            result.set_element(
                pos,
                NeigborNode {
                    neighbors: connected_neigbors,
                },
            )
        });
}
