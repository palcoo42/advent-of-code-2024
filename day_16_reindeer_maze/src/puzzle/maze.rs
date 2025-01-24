use core::panic;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
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
        // self.print(&_path);
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
    fn print(&self, steps: &[(Point, Direction)]) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                // In case of empty tile show direction if tile was visited
                let character = match self.maze[x][y] {
                    Tile::Empty => match steps
                        .iter()
                        .find(|(point, dir)| point.x == x as isize && point.y == y as isize)
                    {
                        None => '.',
                        Some((step, dir)) => match dir {
                            Direction::East => '>',
                            Direction::South => 'v',
                            Direction::West => '<',
                            Direction::North => '^',
                        },
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
}
