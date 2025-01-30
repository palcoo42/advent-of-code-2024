use core::panic;
use std::{
    cmp::{Ordering, Reverse},
    collections::{HashMap, HashSet, VecDeque},
};

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use priority_queue::PriorityQueue;

use super::{direction::Direction, point::Point, tile::Tile};

const FORWARD_SCORE: usize = 1;
const ROTATE_SCORE: usize = 1000;

#[derive(Debug, Default)]
pub struct Maze {
    rows: usize,
    cols: usize,
    maze: Vec<Vec<Tile>>,
}

impl Maze {
    pub fn new(rows: usize, cols: usize, maze: Vec<Vec<Tile>>) -> Self {
        Self { rows, cols, maze }
    }

    pub fn find_lowest_score(&self) -> Result<usize, PuzzleError> {
        let (lowest_score, _path) = self.dijkstra_lowest_score()?;
        // self.print(&_path, None);
        Ok(lowest_score)
    }

    fn dijkstra_lowest_score(&self) -> Result<(usize, Vec<(Point, Direction)>), PuzzleError> {
        // Queue with next states to analyze
        let mut queue = PriorityQueue::new();

        // Track already visited nodes
        let mut visited = HashSet::new();

        // Current scores for all nodes
        let mut scores = HashMap::new();

        // Track previous nodes
        let mut previous = HashMap::new();

        // Initialize with the start position
        let (start, end) = self.get_start_and_end()?;

        queue.push((start, Direction::East), Reverse(0));
        scores.insert(start, 0);
        previous.insert(start, None);

        while let Some(((point, direction), priority)) = queue.pop() {
            // Skip already visited states
            if visited.contains(&(point, direction)) {
                continue;
            }

            // Fetch score for current node
            let score = priority.0;

            visited.insert((point, direction));

            // Check for a solution
            if point == end {
                // We have found a shortest path

                // Backtrack path to the solution
                let mut path = Vec::new();

                let mut previous_node: &Option<(Point, Direction)> = previous
                    .get(&end)
                    .unwrap_or_else(|| panic!("Failed to find end point '{:?}' in previous", end));

                while let Some((prev_point, prev_dir)) = previous_node {
                    path.push((*prev_point, *prev_dir));

                    previous_node = previous.get(prev_point).unwrap_or_else(|| {
                        panic!("Failed to find end point '{:?}' in previous", end)
                    });
                }

                path.reverse();

                // Stop here and return results
                return Ok((score, path));
            }

            // Spawn next possible states:
            // - in the neighbor cell in the same direction with the cost 1
            // - in the same cell rotated in clock direction with the cost 1000
            // - in the same cell rotated in clockwise direction with the cost 1000
            let next_states = [
                (point.neighbor(direction), direction, score + FORWARD_SCORE),
                (point, direction.get_left(), score + ROTATE_SCORE),
                (point, direction.get_right(), score + ROTATE_SCORE),
            ];

            // Update states which are valid:
            // - point is within maze borders
            // - next_score < current score
            for (next_point, next_direction, next_score) in next_states {
                // Add next state for further analysis only if next point/direction are valid
                if !visited.contains(&(next_point, next_direction))
                    && self.is_point_within_maze(&point)
                    && self.get_tile(&next_point) != &Tile::Wall
                {
                    queue.push((next_point, next_direction), Reverse(next_score));
                }

                // If point/direction is not yet visited use as a current_score 'INFINITE'
                let current_score = scores.entry(next_point).or_insert(usize::MAX);

                // Update current score if next score is smaller for given point
                if next_score < *current_score {
                    // Update score
                    *current_score = next_score;

                    // Update priority of node in the queue
                    queue.change_priority(&(next_point, next_direction), Reverse(next_score));

                    // Update previous node to point to the correct node
                    previous.insert(next_point, Some((point, direction)));
                }
            }
        }

        Err(PuzzleError::GenericError(
            "Failed to find the shortest path".to_string(),
        ))
    }

    #[inline]
    fn is_point_within_maze(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.cols as isize && point.y >= 0 && point.y < self.rows as isize
    }

    #[inline]
    fn get_tile(&self, point: &Point) -> &Tile {
        &self.maze[point.y as usize][point.x as usize]
    }

    fn get_start_and_end(&self) -> Result<(Point, Point), PuzzleError> {
        let mut start = None;
        let mut end = None;

        for i in 0..self.rows {
            for j in 0..self.cols {
                match self.maze[i][j] {
                    Tile::Start => {
                        start = Some(Point {
                            x: j as isize,
                            y: i as isize,
                        });
                    }
                    Tile::End => {
                        end = Some(Point {
                            x: j as isize,
                            y: i as isize,
                        });
                    }
                    _ => {}
                }
            }
        }

        match (start, end) {
            (Some(s), Some(e)) => Ok((s, e)),
            _ => Err(PuzzleError::GenericError(format!(
                "Failed to find Start '{:?}' and/or End '{:?}'",
                start, end
            ))),
        }
    }

    #[allow(unused)]
    fn print(&self, steps: &[Point]) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                // In case of empty tile show direction if tile was visited
                let character = match self.maze[y][x] {
                    Tile::Empty => match steps
                        .iter()
                        .find(|point| point.x == x as isize && point.y == y as isize)
                    {
                        None => '.',
                        Some(_) => 'O',
                    },
                    Tile::Wall => '#',
                    Tile::Start => 'S',
                    Tile::End => 'E',
                };

                print!("{}", character);
            }
            println!();
        }
    }

    pub fn find_all_paths(&self) -> Result<usize, PuzzleError> {
        let solution = self.dijkstra_all_paths()?;
        // self.print(&solution);
        // Note: Returned vector already contains unique items
        Ok(solution.len())
    }

    // Run Modified Dijkstra algorithm to find all paths from Start to End
    fn dijkstra_all_paths(&self) -> Result<Vec<Point>, PuzzleError> {
        let mut lowest_score = usize::MAX;
        let (start, end) = self.get_start_and_end()?;

        let mut nodes = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = PriorityQueue::new();

        nodes.insert((start, Direction::East), (0, vec![]));
        queue.push((start, Direction::East, 0), Reverse(0));

        while let Some(((point, direction, score), _)) = queue.pop() {
            if visited.contains(&(point, direction)) {
                continue;
            }

            visited.insert((point, direction));

            // Check for a solution
            if point == end {
                match score.cmp(&lowest_score) {
                    Ordering::Less => {
                        // New shortest path
                        lowest_score = score;
                    }
                    Ordering::Equal | Ordering::Greater => {
                        // Nothing to do
                    }
                }

                // Continue to find all paths
                continue;
            }

            // Analyze neighbors
            let next_states = [
                (point.neighbor(direction), direction, score + FORWARD_SCORE),
                (point, direction.get_left(), score + ROTATE_SCORE),
                (point, direction.get_right(), score + ROTATE_SCORE),
            ];

            // Update states which are valid:
            // - point is within maze borders
            // - next_score < current score
            for (next_point, next_direction, next_score) in next_states {
                // Add next state for further analysis only if next point/direction are valid
                if !visited.contains(&(next_point, next_direction))
                    && self.is_point_within_maze(&point)
                    && self.get_tile(&next_point) != &Tile::Wall
                {
                    queue.push(
                        (next_point, next_direction, next_score),
                        Reverse(next_score),
                    );
                }

                // If point/direction is not yet visited use as a current_score 'INFINITE'
                let current_score = nodes
                    .entry((next_point, next_direction))
                    .or_insert((usize::MAX, vec![]));

                // Update current score if next score is smaller for given point
                match next_score.cmp(&current_score.0) {
                    Ordering::Less => {
                        // Update score
                        current_score.0 = next_score;
                        current_score.1 = vec![(point, direction)];

                        // Update priority of node in the queue
                        queue.change_priority(
                            &(next_point, next_direction, next_score),
                            Reverse(next_score),
                        );
                    }
                    Ordering::Equal => {
                        // Update score
                        current_score.1.push((point, direction));

                        // Update priority of node in the queue
                        queue.change_priority(
                            &(next_point, next_direction, next_score),
                            Reverse(next_score),
                        );
                    }
                    Ordering::Greater => {}
                }
            }
        }

        // Backtrack - Go from End node and collect back all parent nodes
        let minimal_costs = nodes
            .iter()
            .filter_map(|((point, direction), (cost, parents))| {
                if point == &end && cost == &lowest_score {
                    Some((*point, *direction, parents.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        for (point, _, parents) in minimal_costs {
            for (_, parent_direction) in parents {
                queue.push_back((point, parent_direction));
            }
        }

        while let Some((point, direction)) = queue.pop_front() {
            if seen.contains(&(point, direction)) {
                continue;
            }

            seen.insert((point, direction));

            // Fetch node parents
            let (_, parents) = nodes
                .get(&(point, direction))
                .unwrap_or_else(|| panic!("Failed to find '{:?}:{:?}' in nodes", point, direction));

            for (parent_point, parent_direction) in parents {
                queue.push_back((*parent_point, *parent_direction))
            }
        }

        // Collect only unique point, i.e. ignore direction
        let unique_points = seen
            .into_iter()
            .map(|(point, _)| point)
            .collect::<HashSet<_>>();

        Ok(unique_points.into_iter().collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::parser::Parser;

    use super::*;

    fn build_small_maze() -> Maze {
        let maze = [
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ];

        Parser::parse_lines(&maze).expect("Failed to build Maze")
    }

    fn build_large_maze() -> Maze {
        let maze = [
            "#################",
            "#...#...#...#..E#",
            "#.#.#.#.#.#.#.#.#",
            "#.#.#.#...#...#.#",
            "#.#.#.#.###.#.#.#",
            "#...#.#.#.....#.#",
            "#.#.#.#.#.#####.#",
            "#.#...#.#.#.....#",
            "#.#.#####.#.###.#",
            "#.#.#.......#...#",
            "#.#.###.#####.###",
            "#.#.#...#.....#.#",
            "#.#.#.#####.###.#",
            "#.#.#.........#.#",
            "#.#.#.#########.#",
            "#S#.............#",
            "#################",
        ];

        Parser::parse_lines(&maze).expect("Failed to build Maze")
    }

    #[test]
    fn test_maze_build() {
        let maze = build_small_maze();

        assert_eq!(maze.rows, 15);
        assert_eq!(maze.cols, 15);
        assert_eq!(maze.maze[13][1], Tile::Start);
        assert_eq!(maze.maze[1][13], Tile::End);
    }

    #[test]
    fn test_find_lowest_score_small_maze() {
        let maze = build_small_maze();
        let result = maze.find_lowest_score();

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), 7036);
    }

    #[test]
    fn test_find_lowest_score_large_maze() {
        let maze = build_large_maze();
        let result = maze.find_lowest_score();

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), 11048);
    }

    #[test]
    fn test_find_all_paths_small_maze() {
        let maze = build_small_maze();
        let result = maze.find_all_paths();

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), 45);
    }

    #[test]
    fn test_find_all_paths_large_maze() {
        let maze = build_large_maze();
        let result = maze.find_all_paths();

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), 64);
    }
}
