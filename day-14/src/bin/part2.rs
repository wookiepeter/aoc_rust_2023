use aoc_util::string_grid::GridInfo;
use day_14::Rock;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

/// Contains fixed positions and ordering functions for one complete cycle, since those don't change
struct CycleInfo {
    map_size: (usize, usize),
    tuples: Vec<(
        fn(&(usize, usize), &(usize, usize)) -> Ordering,
        Vec<Vec<(usize, Rock)>>,
    )>,
}

fn process(input: &str) -> String {
    let map_size = (input.lines().next().unwrap().len(), input.lines().count());
    let cubic_rocks: Vec<(usize, usize)> = aoc_util::find_chars_positions(input, |c| c == '#');
    let mut rolling_rocks: Vec<(usize, usize)> =
        aoc_util::find_chars_positions(input, |c| c == 'O');

    let cycle_info = CycleInfo::create_cycle_info(cubic_rocks.clone(), map_size);
    let mut cycle_map: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut cycle_end = 0;

    // look for the first repetition
    // -> determine the cycle for that repetition
    // -> go to the last occurance of that cycle still in the target iterations
    // -> simulate the remaining cycles to the target iterations (could just retrieve the map?!)
    for i in 0..100000 {
        rolling_rocks = cycle_info.process_cycle(rolling_rocks);
        if cycle_map.contains_key(&rolling_rocks) {
            cycle_end = i;
            break;
        }
        cycle_map.insert(rolling_rocks.clone(), i);
    }

    if cycle_end == 0 {
        panic!("Did not find a cycle in a reasonable time");
    }

    let cycle_start = *cycle_map.get(&rolling_rocks).unwrap();
    let final_cycle_index = (1000000000 - cycle_start) % (cycle_end - cycle_start);
    let final_state_index = final_cycle_index + cycle_start - 1;

    let (final_rocks, _) = cycle_map
        .iter()
        .find(|(_, index)| **index == final_state_index)
        .unwrap();

    let info = GridInfo {
        default_char: '.',
        dimensions: cycle_info.map_size,
        other_chars: vec![('#', cubic_rocks), ('O', final_rocks.clone())],
    };
    let grid = info.create_grid();
    println!("{grid}");

    cycle_info.calculate_weight(final_rocks.clone()).to_string()
}

/// only exists for test cases
fn _process_n_cycles(input: &str, n: usize) -> String {
    let map_size = (input.lines().next().unwrap().len(), input.lines().count());
    let cubic_rocks: Vec<(usize, usize)> = aoc_util::find_chars_positions(input, |c| c == '#');
    let mut rolling_rocks: Vec<(usize, usize)> =
        aoc_util::find_chars_positions(input, |c| c == 'O');

    let cycle_info = CycleInfo::create_cycle_info(cubic_rocks.clone(), map_size);

    for _ in 0..n {
        rolling_rocks = cycle_info.process_cycle(rolling_rocks);
    }

    let info = GridInfo {
        default_char: '.',
        dimensions: cycle_info.map_size,
        other_chars: vec![('#', cubic_rocks), ('O', rolling_rocks)],
    };
    info.create_grid()
}

impl CycleInfo {
    fn process_cycle(&self, rock_positions: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut current_rock_positions = rock_positions.clone();

        // handle north rotation
        {
            let (roll_dir, mut rock_map) = self.tuples[0].clone();
            current_rock_positions.sort_by(roll_dir);
            current_rock_positions.iter().for_each(|(x, y)| {
                let blocking_rock = rock_map[*x]
                    .iter()
                    .enumerate()
                    .filter(|(_, (cur_y, _))| *y > *cur_y)
                    .last()
                    .map(|(index, rock)| (index, *rock));

                match blocking_rock {
                    Some((index, prev_rock)) => {
                        rock_map[*x].insert(index + 1, (prev_rock.0 + 1, Rock::Round))
                    }
                    None => rock_map[*x].insert(0, (0, Rock::Round)),
                }
            });

            current_rock_positions = rock_map
                .iter()
                .enumerate()
                .flat_map(|(row, vec)| {
                    vec.iter()
                        .filter_map(|(y, rock_type)| match rock_type {
                            Rock::Cube => None,
                            _ => Some((row, *y)),
                        })
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect()
        }

        // handle west rotation
        {
            let (roll_dir, mut rock_map) = self.tuples[1].clone();
            current_rock_positions.sort_by(roll_dir);
            current_rock_positions.iter().for_each(|(x, y)| {
                let blocking_rock = rock_map[*y]
                    .iter()
                    .enumerate()
                    .filter(|(_, (cur_x, _))| *x > *cur_x)
                    .last()
                    .map(|(index, rock)| (index, *rock));

                match blocking_rock {
                    Some((index, prev_rock)) => {
                        rock_map[*y].insert(index + 1, (prev_rock.0 + 1, Rock::Round))
                    }
                    None => rock_map[*y].insert(0, (0, Rock::Round)),
                }
            });

            current_rock_positions = rock_map
                .iter()
                .enumerate()
                .flat_map(|(row, vec)| {
                    vec.iter()
                        .filter_map(|(x, rock_type)| match rock_type {
                            Rock::Cube => None,
                            _ => Some((*x, row)),
                        })
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect()
        }

        // handle south rotation
        {
            let (roll_dir, mut rock_map) = self.tuples[2].clone();
            current_rock_positions.sort_by(roll_dir);
            current_rock_positions.iter().for_each(|(x, y)| {
                let blocking_rock = rock_map[*x]
                    .iter()
                    .enumerate()
                    .filter(|(_, (cur_y, _))| *y < *cur_y)
                    .last()
                    .map(|(index, rock)| (index, *rock));

                match blocking_rock {
                    Some((index, prev_rock)) => {
                        rock_map[*x].insert(index + 1, (prev_rock.0 - 1, Rock::Round))
                    }
                    None => rock_map[*x].insert(0, (self.map_size.1 - 1, Rock::Round)),
                }
            });

            current_rock_positions = rock_map
                .iter()
                .enumerate()
                .flat_map(|(row, vec)| {
                    vec.iter()
                        .filter_map(|(y, rock_type)| match rock_type {
                            Rock::Cube => None,
                            _ => Some((row, *y)),
                        })
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect()
        }

        // handle east rotation
        {
            let (roll_dir, mut rock_map) = self.tuples[3].clone();
            current_rock_positions.sort_by(roll_dir);
            current_rock_positions.iter().for_each(|(x, y)| {
                let blocking_rock = rock_map[*y]
                    .iter()
                    .enumerate()
                    .filter(|(_, (cur_x, _))| *x < *cur_x)
                    .last()
                    .map(|(index, rock)| (index, *rock));

                match blocking_rock {
                    Some((index, prev_rock)) => {
                        rock_map[*y].insert(index + 1, (prev_rock.0 - 1, Rock::Round))
                    }
                    None => rock_map[*y].insert(0, (self.map_size.1 - 1, Rock::Round)),
                }
            });

            current_rock_positions = rock_map
                .iter()
                .enumerate()
                .flat_map(|(row, vec)| {
                    vec.iter()
                        .filter_map(|(x, rock_type)| match rock_type {
                            Rock::Cube => None,
                            _ => Some((*x, row)),
                        })
                        .collect::<Vec<(usize, usize)>>()
                })
                .collect()
        }

        current_rock_positions
    }

    pub fn calculate_weight(&self, rock_positions: Vec<(usize, usize)>) -> usize {
        rock_positions
            .iter()
            .map(|(_, y)| self.map_size.1 - y)
            .sum::<usize>()
    }

    pub fn create_cycle_info(
        block_positions: Vec<(usize, usize)>,
        map_size: (usize, usize),
    ) -> CycleInfo {
        // create a 2 dimensional vec (NOT A GRID) where each rock is sorted into the appropriate column
        // Issue: Indexing and ordering has to be dependent on the direction when processing happens
        let (x_size, y_size) = map_size;

        let mut north_sorted = block_positions.clone();
        let mut north_map: Vec<Vec<(usize, Rock)>> = vec![Vec::new(); x_size];
        north_sorted.sort_by(roll_north);
        north_sorted
            .iter()
            .for_each(|(x, y)| north_map[*x].push((*y, Rock::Cube)));

        let mut west_sorted = block_positions.clone();
        let mut west_map: Vec<Vec<(usize, Rock)>> = vec![Vec::new(); y_size];
        west_sorted.sort_by(roll_west);
        west_sorted
            .iter()
            .for_each(|(x, y)| west_map[*y].push((*x, Rock::Cube)));

        let mut south_sorted = block_positions.clone();
        let mut south_map: Vec<Vec<(usize, Rock)>> = vec![Vec::new(); x_size];
        south_sorted.sort_by(roll_south);
        south_sorted
            .iter()
            .for_each(|(x, y)| south_map[*x].push((*y, Rock::Cube)));

        let mut east_sorted = block_positions.clone();
        let mut east_map: Vec<Vec<(usize, Rock)>> = vec![Vec::new(); y_size];
        east_sorted.sort_by(roll_east);
        east_sorted
            .iter()
            .for_each(|(x, y)| east_map[*y].push((*x, Rock::Cube)));

        east_sorted.sort_by(roll_east);
        CycleInfo {
            map_size,
            tuples: vec![
                (roll_north, north_map),
                (roll_west, west_map),
                (roll_south, south_map),
                (roll_east, east_map),
            ],
        }
    }
}

pub fn roll_north(lhs: &(usize, usize), rhs: &(usize, usize)) -> Ordering {
    lhs.1.cmp(&rhs.1)
}

pub fn roll_west(lhs: &(usize, usize), rhs: &(usize, usize)) -> Ordering {
    lhs.0.cmp(&rhs.0)
}

pub fn roll_south(lhs: &(usize, usize), rhs: &(usize, usize)) -> Ordering {
    lhs.1.cmp(&rhs.1).reverse()
}

pub fn roll_east(lhs: &(usize, usize), rhs: &(usize, usize)) -> Ordering {
    lhs.0.cmp(&rhs.0).reverse()
}

#[cfg(test)]
mod tests {
    use crate::{_process_n_cycles, process};

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
        assert_eq!(result, "64".to_string())
    }

    #[test]
    fn test_one_cycle() {
        let result = _process_n_cycles(
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
            1,
        );
        assert_eq!(
            result,
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
                .to_string()
        )
    }

    #[test]
    fn test_two_cycles() {
        let result = _process_n_cycles(
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
            2,
        );
        assert_eq!(
            result,
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
                .to_string()
        )
    }

    #[test]
    fn test_three_cycles() {
        let result = _process_n_cycles(
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
            3,
        );
        assert_eq!(
            result,
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
                .to_string()
        )
    }
}
