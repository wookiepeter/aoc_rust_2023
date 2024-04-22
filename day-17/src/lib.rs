use std::{
    cmp::Ordering,
    collections::{btree_map::Range, BinaryHeap, HashMap},
};

use aoc_util::{
    direction::Direction,
    grid::Grid,
    manhattan_dist,
    usize_point::{point_add, Point},
};

pub struct Data {
    pub queue: BinaryHeap<GraphState>,
    pub grid: Grid<usize>,
    pub visited: HashMap<(Point, bool), usize>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct GraphState {
    pub position: Point,
    pub heat: usize,
    pub last_step_horizontal: bool,
    evaluation: usize,
}

impl Ord for GraphState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluation.cmp(&other.evaluation).reverse()
    }
}

// probably not the way to do things but this is just used to satisfy trait bounds
impl PartialOrd for GraphState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl GraphState {
    pub fn new(
        position: Point,
        heat: usize,
        last_dir: Direction,
        grid: &Grid<usize>,
    ) -> GraphState {
        GraphState {
            position,
            heat,
            last_step_horizontal: last_dir.is_horizontal(),
            evaluation: heat + manhattan_dist(&position, &grid.size),
        }
    }
}

pub fn add_steps_in_direction(
    state: &GraphState,
    data: &mut Data,
    dir: Direction,
    range: std::ops::Range<usize>,
) {
    let mut current_state = *state;
    for i in 0..range.end {
        if let Some(position) = point_add(current_state.position, dir.into(), data.grid.size) {
            let new_state = GraphState::new(
                position,
                current_state.heat + data.grid.get(position).unwrap(),
                dir,
                &data.grid,
            );

            if range.contains(&i) {
                match data
                    .visited
                    .get(&(new_state.position, new_state.last_step_horizontal))
                {
                    Some(value) if *value <= new_state.heat => {}
                    None | Some(_) => {
                        data.visited.insert(
                            (new_state.position, new_state.last_step_horizontal),
                            new_state.heat,
                        );
                        data.queue.push(new_state);
                    }
                }
            }
            current_state = new_state;
        } else {
            return;
        }
    }
}
