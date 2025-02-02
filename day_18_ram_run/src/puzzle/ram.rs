use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use advent_of_code::{
    grids::{direction::Direction, grid::Grid, point::Point},
    puzzles::puzzle_error::PuzzleError,
};
use priority_queue::PriorityQueue;

#[derive(Default)]
pub struct Ram {
    grid: Grid,
}

impl Ram {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn count_minimum_steps(
        &self,
        corrupted: &[Point],
        corrupted_bytes_count: usize,
    ) -> Result<usize, PuzzleError> {
        // Prepare grid with corrupted bytes '#'
        let grid = self.corrupt_ram(&corrupted[0..corrupted_bytes_count])?;

        let minimum_path = Self::dijkstra_minimum_path(
            &grid,
            Point { x: 0, y: 0 },
            Point {
                x: grid.cols() as isize - 1,
                y: grid.rows() as isize - 1,
            },
        );

        match minimum_path {
            Some((shortest_steps, _path)) => {
                // grid.print_with_visited(&_path);
                Ok(shortest_steps)
            }

            None => Err(PuzzleError::GenericError(
                "Failed to find shortest path".to_string(),
            )),
        }
    }

    fn corrupt_ram(&self, corrupted: &[Point]) -> Result<Grid, PuzzleError> {
        // Prepare grid with corrupted bytes, take into account number of 'fallen_bytes'
        // Fallen point is marked with '#'
        let corrupted_bytes = corrupted
            .iter()
            .map(|point| (*point, '#'))
            .collect::<Vec<_>>();

        let mut grid = self.grid.clone();
        grid.fill(&corrupted_bytes)?;

        Ok(grid)
    }

    fn dijkstra_minimum_path(grid: &Grid, start: Point, end: Point) -> Option<(usize, Vec<Point>)> {
        // Prepare data structures
        let mut shortest_score = None;
        let mut path = Vec::new();
        let mut visited = HashSet::new();
        let mut nodes = HashMap::new();
        let mut queue = PriorityQueue::new();

        // Fill in all nodes as infinite
        grid.get_all_values('.').into_iter().for_each(|point| {
            nodes.insert(point, (usize::MAX, vec![]));
        });

        // Override start position
        nodes.insert(start, (0, vec![]));

        // Add start position to the queue for analysis
        queue.push(start, Reverse(0));

        while let Some((point, _)) = queue.pop() {
            // Skip already visited points
            if visited.contains(&point) {
                continue;
            }

            visited.insert(point);

            // Fetch current score and parents
            let (score, _) = nodes
                .get(&point)
                .cloned()
                .unwrap_or_else(|| panic!("Failed to find point '{:?}' in nodes", point));

            // Check for a solution
            if point == end {
                // We have a shortest path
                shortest_score = Some(score);

                // Backtrack path to the solution
                path = Self::backtrack_path(&end, &nodes);
            }

            // Analyze valid neighbors
            let neighbors = grid.neighbors_if(
                &point,
                &[
                    &Direction::North,
                    &Direction::East,
                    &Direction::South,
                    &Direction::West,
                ],
                |point, _| grid[*point] != '#',
            );

            for next_point in neighbors {
                // Fetch values for the next point
                let (current_score, current_parents) = nodes
                    .get_mut(&next_point)
                    .unwrap_or_else(|| panic!("Failed to find point '{:?}' in nodes", point));

                // Every step has the same score
                let next_score = score + 1;

                if !visited.contains(&next_point) && next_score < *current_score {
                    *current_score = next_score;
                    current_parents.push(point);
                    queue.push(next_point, Reverse(next_score));
                }
            }
        }

        // Return score together with path
        shortest_score.map(|score| (score, path))
    }

    fn backtrack_path(end: &Point, nodes: &HashMap<Point, (usize, Vec<Point>)>) -> Vec<Point> {
        let mut path = Vec::new();
        let mut next_point: Option<Point> = Some(*end);

        while let Some(point) = next_point {
            // Append to the path
            path.push(point);

            // Find next point
            let entry = nodes
                .get(&point)
                .unwrap_or_else(|| panic!("Failed to find point '{:?}' in nodes", point));

            next_point = match entry.1.is_empty() {
                true => None,
                false => Some(entry.1[0]),
            }
        }

        path
    }

    pub fn find_first_falling_byte(
        &self,
        corrupted: &[Point],
        corrupted_bytes: usize,
    ) -> Result<Point, PuzzleError> {
        // Use binary algorithm to speed up
        let mut falling_byte: Option<usize> = None;
        let mut left = corrupted_bytes; // Include already corrupted bytes
        let mut right = corrupted.len();

        while left < right {
            // Corrupt RAM
            let middle = left + (right - left) / 2;

            // Note: 0..=middle includes as corrupted also 'middle' point as we check middle now
            let grid = self.corrupt_ram(&corrupted[0..=middle])?;

            // Analyze result
            match Self::dijkstra_path_found(&grid) {
                true => {
                    // There is still a path
                    left = middle + 1;
                }
                false => {
                    // Path is blocked
                    right = middle;
                    falling_byte = Some(middle);
                }
            }
        }

        falling_byte
            .map(|index| corrupted[index])
            .ok_or_else(|| PuzzleError::GenericError("Solution not found".to_string()))
    }

    fn dijkstra_path_found(grid: &Grid) -> bool {
        // Create start and end points
        let start = Point { x: 0, y: 0 };
        let end = Point {
            x: grid.cols() as isize - 1,
            y: grid.rows() as isize - 1,
        };

        Self::dijkstra_minimum_path(grid, start, end).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_ram() -> (Ram, Vec<Point>) {
        let ram = Ram::new(Grid::new_with(7, 7, |_| '.').expect("Failed to create Grid"));
        let points = vec![
            Point { x: 5, y: 4 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 5 },
            Point { x: 3, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 6, y: 3 },
            Point { x: 2, y: 4 },
            Point { x: 1, y: 5 },
            Point { x: 0, y: 6 },
            Point { x: 3, y: 3 },
            Point { x: 2, y: 6 },
            Point { x: 5, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 5, y: 5 },
            Point { x: 2, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 1, y: 4 },
            Point { x: 0, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 1, y: 1 },
            Point { x: 6, y: 1 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 5 },
            Point { x: 1, y: 6 },
            Point { x: 2, y: 0 },
        ];

        (ram, points)
    }

    #[test]
    fn test_count_minimum_steps() {
        let (ram, corrupted) = build_ram();

        let result = ram.count_minimum_steps(&corrupted, 12);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), 22);
    }

    #[test]
    fn test_find_first_falling_byte() {
        let (ram, corrupted) = build_ram();

        let result = ram.find_first_falling_byte(&corrupted, 12);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Point { x: 6, y: 1 });
    }
}
