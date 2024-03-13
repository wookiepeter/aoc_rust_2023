use aoc_util::usize_point::{point_add, Point};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub enum MirrorStatus {
    Full,
    RightHalf,
    LeftHalf,
}

pub struct MapData {
    pub chars: Vec<Vec<char>>,
    pub size: (usize, usize),
}

pub fn process_beam(
    position: Point,
    direction: (i32, i32),
    map_data: &MapData,
    energized: &mut HashSet<Point>,
    mirror_map: &mut HashMap<Point, MirrorStatus>,
) {
    if let Some(mirror_pos) = find_next_mirror(position, direction, map_data, energized) {
        let mirror_status = mirror_map.get(&mirror_pos).copied();

        match mirror_status {
            Some(MirrorStatus::Full) => {}
            Some(status) => {
                // Check side and potentially handle  mirror for / or \
                _handle_half_lit_mirror(
                    mirror_pos, direction, map_data, energized, mirror_map, status,
                )
            }
            None => {
                // handle new mirror
                _handle_new_mirror(mirror_pos, direction, map_data, energized, mirror_map)
            }
        }
    }
}

fn _handle_half_lit_mirror(
    position: Point,
    direction: (i32, i32),
    map_data: &MapData,
    energized: &mut HashSet<Point>,
    mirror_map: &mut HashMap<Point, MirrorStatus>,
    status: MirrorStatus,
) {
    let mirror_char = map_data.chars[position.1][position.0];
    let opposite_dir = (-direction.0, -direction.1);
    let nw_dir = vec![(-1, 0), (0, -1)];
    let sw_dir = vec![(-1, 0), (0, 1)];
    let ne_dir = vec![(1, 0), (0, -1)];
    let se_dir = vec![(1, 0), (0, 1)];

    // removes all the cases where the already lit side is hit again
    match status {
        MirrorStatus::LeftHalf
            if (nw_dir.contains(&opposite_dir) && mirror_char == '/')
                || (sw_dir.contains(&opposite_dir) && mirror_char == '\\') =>
        {
            return;
        }
        MirrorStatus::RightHalf
            if (ne_dir.contains(&opposite_dir) && mirror_char == '\\')
                || (se_dir.contains(&opposite_dir) && mirror_char == '/') =>
        {
            return;
        }
        _ => (),
    };

    mirror_map.insert(position, MirrorStatus::Full);
    match mirror_char {
        '/' if nw_dir.contains(&opposite_dir) => {
            process_beam(
                position,
                *nw_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '/' => {
            process_beam(
                position,
                *se_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '\\' if sw_dir.contains(&opposite_dir) => {
            process_beam(
                position,
                *sw_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '\\' => {
            process_beam(
                position,
                *ne_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        c => panic!("Found Encountered invalid character {}", c),
    }
}

// internal helper to declutter depth search
fn _handle_new_mirror(
    position: Point,
    direction: (i32, i32),
    map_data: &MapData,
    energized: &mut HashSet<Point>,
    mirror_map: &mut HashMap<Point, MirrorStatus>,
) {
    energized.insert(position);
    let mirror_char = map_data.chars[position.1][position.0];
    let opposite_dir = (-direction.0, -direction.1);
    let nw_dir = vec![(-1, 0), (0, -1)];
    let sw_dir = vec![(-1, 0), (0, 1)];
    let ne_dir = vec![(1, 0), (0, -1)];
    let se_dir = vec![(1, 0), (0, 1)];
    match mirror_char {
        '-' => {
            mirror_map.insert(position, MirrorStatus::Full);
            if direction != (1, 0) {
                process_beam(position, (-1, 0), map_data, energized, mirror_map);
            }
            if direction != (-1, 0) {
                process_beam(position, (1, 0), map_data, energized, mirror_map);
            }
        }
        '|' => {
            mirror_map.insert(position, MirrorStatus::Full);
            if direction != (0, 1) {
                process_beam(position, (0, -1), map_data, energized, mirror_map);
            }
            if direction != (0, -1) {
                process_beam(position, (0, 1), map_data, energized, mirror_map);
            }
        }
        '/' if nw_dir.contains(&opposite_dir) => {
            mirror_map.insert(position, MirrorStatus::LeftHalf);
            process_beam(
                position,
                *nw_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '/' => {
            mirror_map.insert(position, MirrorStatus::RightHalf);
            process_beam(
                position,
                *se_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '\\' if sw_dir.contains(&opposite_dir) => {
            mirror_map.insert(position, MirrorStatus::LeftHalf);
            process_beam(
                position,
                *sw_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        '\\' => {
            mirror_map.insert(position, MirrorStatus::RightHalf);
            process_beam(
                position,
                *ne_dir
                    .iter()
                    .find(|d: &&(i32, i32)| opposite_dir.ne(*d))
                    .unwrap(),
                map_data,
                energized,
                mirror_map,
            );
        }
        c => panic!("Found Encountered invalid character {}", c),
    }
}

fn find_next_mirror(
    position: Point,
    direction: (i32, i32),
    map_data: &MapData,
    energized: &mut HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    // To simplify calling
    let mut cur_pos = position;
    /*
    let mut cur_pos: (usize, usize) = match point_add(position, direction, map_data.size) {
        Some(pos) => pos,
        None => return None,
    };
    */

    // Handle edge case where first tile was a mirror
    if energized.is_empty() && map_data.chars[position.1][position.0] != '.' {
        return Some(position);
    } else if energized.is_empty() {
        // -> if first tile is not a mirror it still needs to be lit
        energized.insert(position);
    }

    while let Some(pos) = point_add(cur_pos, direction, map_data.size) {
        if map_data.chars[pos.1][pos.0] != '.' {
            return Some(pos);
        }
        energized.insert(pos);
        cur_pos = pos;
    }
    None
}
