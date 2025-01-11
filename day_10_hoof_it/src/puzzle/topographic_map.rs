use std::collections::{HashSet, VecDeque};

use super::{position::Position, topographic_state::TopographicState};

const TRAIL_HEAD_START: u8 = 0;
const TRAIL_HEAD_END: u8 = 9;

#[derive(Debug, Clone, Default)]
pub struct TopographicMap {
    rows: usize,
    cols: usize,
    internal: Vec<Vec<u8>>,
}

impl TopographicMap {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        if grid.is_empty() {
            panic!("Empty grid");
        }

        Self {
            rows: grid.len(),
            cols: grid[0].len(),
            internal: grid,
        }
    }

    pub fn rows_len(&self) -> usize {
        self.rows
    }

    pub fn cols_len(&self) -> usize {
        self.cols
    }

    pub fn count_trail_heads_score(&self) -> usize {
        let trail_heads = self.find_trail_heads();

        trail_heads
            .into_iter()
            .map(|trail_head| TopographicMap::count_trail_head_score(self, &trail_head))
            .sum()
    }

    fn find_trail_heads(&self) -> Vec<Position> {
        let mut trail_heads = Vec::new();

        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.internal[row][col] == TRAIL_HEAD_START {
                    // Start of the trail head is marked with 0
                    trail_heads.push(Position { row, col });
                }
            }
        }

        trail_heads
    }

    fn count_trail_head_score(topographic_map: &TopographicMap, trail_head: &Position) -> usize {
        // Starting position is 0
        if topographic_map.get_value(trail_head) != TRAIL_HEAD_START {
            return 0;
        }

        // Collection of found trails (end positions for given trail starting ad trail_head)
        let mut hiking_trails: HashSet<Position> = HashSet::new();

        // Remaining states to check
        let state = TopographicState::new(topographic_map.clone(), trail_head);
        let mut remaining_states: VecDeque<_> = vec![state].into();

        // Run DFS to find solutions
        while let Some(state) = remaining_states.pop_front() {
            // If we have reached end position we have a solution so we can stop here
            if state.get_value() == TRAIL_HEAD_END {
                hiking_trails.insert(state.position.clone());
                continue;
            }

            // Find next possible states to analyze
            let next_states = state.find_next_states();
            for next_state in next_states {
                remaining_states.push_front(next_state);
            }
        }

        hiking_trails.len()
    }

    pub fn get_value(&self, pos: &Position) -> u8 {
        self.internal[pos.row][pos.col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_map_simple() -> TopographicMap {
        TopographicMap::new(vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ])
    }

    fn create_map_complex() -> TopographicMap {
        TopographicMap::new(vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ])
    }

    #[test]
    fn test_find_trail_heads() {
        let topographic_map = create_map_simple();
        let trail_heads = topographic_map.find_trail_heads();

        assert_eq!(trail_heads, vec![Position { row: 0, col: 0 }]);
    }

    #[test]
    fn test_count_trail_head_score_simple() {
        let topographic_map = create_map_simple();

        assert_eq!(
            TopographicMap::count_trail_head_score(&topographic_map, &Position { row: 0, col: 0 }),
            1
        );
        assert_eq!(
            TopographicMap::count_trail_head_score(&topographic_map, &Position { row: 0, col: 3 }),
            0
        );
    }

    #[test]
    fn test_count_trail_head_score_complex() {
        const TRAIL_SCORES: [(Position, usize); 9] = [
            (Position { row: 0, col: 2 }, 5),
            (Position { row: 0, col: 4 }, 6),
            (Position { row: 2, col: 4 }, 5),
            (Position { row: 4, col: 6 }, 3),
            (Position { row: 5, col: 2 }, 1),
            (Position { row: 5, col: 5 }, 3),
            (Position { row: 6, col: 0 }, 5),
            (Position { row: 6, col: 6 }, 3),
            (Position { row: 7, col: 1 }, 5),
        ];

        let topographic_map = create_map_complex();

        for trail_score in TRAIL_SCORES {
            let score = TopographicMap::count_trail_head_score(&topographic_map, &trail_score.0);

            assert_eq!(
                score, trail_score.1,
                "Wrong trail head count for position '{:?}', found: {}, expected: {}",
                trail_score.0, score, trail_score.1
            );
        }
    }

    #[test]
    fn test_count_trail_heads_score_simple() {
        let topographic_map = create_map_simple();
        assert_eq!(topographic_map.count_trail_heads_score(), 1);
    }

    #[test]
    fn test_count_trail_heads_score_complex() {
        let topographic_map = create_map_complex();
        assert_eq!(topographic_map.count_trail_heads_score(), 36);
    }
}
