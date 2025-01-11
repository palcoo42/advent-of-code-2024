use std::collections::HashSet;

use super::{grid::Grid, position::Position, topographic_map::TopographicMap};

#[derive(Debug, Clone)]
pub struct TopographicState {
    pub topography: TopographicMap,
    pub position: Position,
    pub visited: HashSet<Position>,
}

impl Grid for TopographicState {
    fn rows_len(&self) -> usize {
        self.topography.rows_len()
    }

    fn cols_len(&self) -> usize {
        self.topography.cols_len()
    }
}

impl TopographicState {
    pub fn new(topography: TopographicMap, position: &Position) -> Self {
        Self {
            topography,
            position: position.clone(),
            visited: vec![position.clone()].into_iter().collect(),
        }
    }

    pub fn get_value(&self) -> u8 {
        self.topography.get_value(&self.position)
    }

    pub fn find_next_states(&self) -> Vec<TopographicState> {
        // Next state should have value incremented by 1
        let next_value = self.get_value() + 1;

        // Filter neighbors who has expected next value
        let neighbors = self
            .position
            .get_neighbors(self)
            .into_iter()
            .filter(|pos| {
                // Note: Check only not yet visited nodes
                if self.visited.contains(pos) {
                    false
                } else {
                    self.topography.get_value(pos) == next_value
                }
            })
            .collect::<Vec<_>>();

        // Build up new states
        neighbors
            .into_iter()
            .map(|pos| {
                let mut state = self.clone();
                state.position = pos.clone();
                state.visited.insert(pos.clone());
                state
            })
            .collect()
    }
}
