use std::{collections::HashMap, path};

use aoc_util::{
    direction::*,
    grid::*,
    manhattan_dist,
    usize_point::{self, Point},
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid: Grid<usize> = Grid::new(input, |c: char| c as usize);
    let mut visited_nodes: HashMap<Point, (usize, usize)> = HashMap::new();
    let path_vec: Vec<Direction> = vec![];
    let start: Point = (0, 0);

    recursive_dfs(start, path_vec, &mut visited_nodes, &grid);

    input.to_string()
}

// this would be more ammo efficient if an actual tree was used instead of
// copying all the path's each time, but should work fine for this purpose.
fn recursive_dfs(
    position: Point,
    path: Vec<Direction>,
    visited_nodes: &mut HashMap<Point, (usize, usize)>,
    grid: &Grid<usize>,
) {
    // What data do i need to save for a path
    //
    // definitely the past 3 directions (maybe just all directions?)
    // definitely the total heat so far
    // current position
    // Keep a dict with all already visited positions, that has info required to throw away the current path
    //      -> # of steps used to reach this node + heat generated on that path

    // TODO: add an Option result containing the actual part and then just compare
    // when going back any valid results and return them up the chain!

    // TODO: Try and figure out an iterative solution for DFS because of [this](https://www.algobreath.com/notes/recursion-vs-iteration-in-rust)

    if path.len() == 0 {
        // do start
    }
}

// the smaller the better
fn greedy_evaluation(position: Point, grid: &Grid<usize>) -> usize {
    manhattan_dist(&grid.size, &position) + grid.get(position).unwrap()
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
