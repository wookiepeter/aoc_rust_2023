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

struct GraphState {
    position: Point,
    heat: usize,
    last_direction: Direction,
}

impl GraphState {
    fn evaluate(&self, grid: &Grid<usize>) {
        manhattan_dist(lhs, rhs)
    }
}

// this would be more ammo efficient if an actual tree was used instead of
// copying all the path's each time, but should work fine for this purpose.
fn recursive_dfs(
    position: Point,
    path: Vec<Direction>,
    visited_nodes: &mut HashMap<Point, (usize, usize)>,
    grid: &Grid<usize>,
) {
    // What data do i need to save for a path?
    // I don't actually need the path, i just need the cost to get there!
    //
    // Instead of past 3 direction i can just save current direction and the
    // amount of steps since the direction changed, could maybe even simplify to horizontal, / vertical
    // the total heat so far
    // current position

    // cool optimisation / simplification i found was to always add all possible states
    // (as in walk 1 or 2 or 3 steps in each direction) and then sort that into an
    // the priority queue using current heat and greedy evaluation to keep the sortation

    // Keep a dict with all already visited positions,
    //      -> # of steps used to reach this node + heat generated on that path?

    // TODO: Try and figure out an iterative solution for DFS because of [this](https://www.algobreath.com/notes/recursion-vs-iteration-in-rust)

    // use a BinaryHeap to do the priority queueing -> just need to flip the order!

    if path.len() == 0 {
        // do start
    }
}

// the smaller the better
fn process_step(position: Point, grid: &Grid<usize>, step: (Direction, usize)) -> Option<()> {
    manhattan_dist(&grid.size, &position) + grid.get(position).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn test_example() {
        let result = process(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
        );
        assert_eq!(result, "4".to_string())
    }
}
