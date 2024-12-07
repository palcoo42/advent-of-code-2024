use std::collections::HashMap;
use std::io::Write;
use std::{collections::HashSet, fs::File};

use super::position::Position;
use super::{direction::Direction, guard::Guard, maze_object::MazeObject};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Maze {
    maze: Vec<Vec<MazeObject>>,
    maze_rows: usize,
    maze_cols: usize,
    guard: Guard,
    new_obstacle_position: Option<Position>,
}

impl Default for Maze {
    fn default() -> Self {
        Self {
            maze: Vec::new(),
            maze_rows: 0,
            maze_cols: 0,
            guard: Guard::new(super::position::Position::new(0, 0), Direction::Down),
            new_obstacle_position: None,
        }
    }
}

impl Maze {
    pub fn new(maze: Vec<Vec<MazeObject>>, guard: Guard) -> Self {
        let maze_rows = maze.len();
        let maze_cols = maze[0].len();

        Self {
            maze,
            maze_rows,
            maze_cols,
            guard,
            new_obstacle_position: None,
        }
    }

    pub fn move_guard_distinct_position_count(&self) -> usize {
        // Track visited positions and directions
        let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();
        let dirs = visited
            .entry(self.guard.get_position().clone())
            .or_default();
        dirs.insert(self.guard.get_direction().clone());

        // Keep track of the new position
        let mut guard = Some(self.guard.clone());

        while let Some(new_guard) = self.move_guard(&guard) {
            let dirs = visited.entry(new_guard.get_position().clone()).or_default();
            dirs.insert(new_guard.get_direction().clone());

            guard = Some(new_guard);
        }

        const PRINT_TO_FILE: bool = false;
        if PRINT_TO_FILE {
            self.print_to_file(&visited);
        }

        // Count visited positions
        visited.len()
    }

    fn print_to_file(&self, visited: &HashMap<Position, HashSet<Direction>>) {
        // Print visited locations into the file
        let mut file = File::create("/tmp/log.txt").unwrap();

        for i in 0..self.maze_rows {
            for j in 0..self.maze_cols {
                let c = match self.maze[i][j] {
                    MazeObject::Empty => {
                        if self.guard.get_position().x == i && self.guard.get_position().y == j {
                            match self.guard.get_direction() {
                                Direction::Right => ">",
                                Direction::Down => "v",
                                Direction::Left => "<",
                                Direction::Up => "^",
                            }
                        } else if let Some(dirs) = visited.get(&Position::new(i, j)) {
                            let up_down =
                                dirs.contains(&Direction::Up) || dirs.contains(&Direction::Down);

                            let left_right =
                                dirs.contains(&Direction::Left) || dirs.contains(&Direction::Right);

                            match (up_down, left_right) {
                                (true, true) => "+",
                                (true, false) => "|",
                                (false, true) => "-",
                                (false, false) => "x",
                            }
                        } else {
                            "."
                        }
                    }
                    MazeObject::Obstruction => "#",
                    MazeObject::NewObstruction => "O",
                };

                write!(file, "{}", c).unwrap();
            }
            writeln!(file).unwrap();
        }
    }

    fn move_guard(&self, guard: &Option<Guard>) -> Option<Guard> {
        match guard {
            Some(g) => match g.get_direction() {
                Direction::Right => self.move_guard_right(g),
                Direction::Down => self.move_guard_down(g),
                Direction::Left => self.move_guard_left(g),
                Direction::Up => self.move_guard_up(g),
            },
            None => None,
        }
    }

    fn move_guard_up(&self, guard: &Guard) -> Option<Guard> {
        // If we are at the top of the maze we are done
        if guard.get_position().x == 0 {
            return None;
        }

        match self.maze[guard.get_position().x - 1][guard.get_position().y] {
            MazeObject::Empty => {
                // Move up
                let mut guard_up = guard.clone();
                guard_up.get_position_mut().x -= 1;

                Some(guard_up)
            }
            MazeObject::Obstruction | MazeObject::NewObstruction => {
                // Rotate right and move right (90 degrees right)
                let mut guard_right = guard.clone();
                *guard_right.get_direction_mut() = Direction::Right;

                self.move_guard_right(&guard_right)
            }
        }
    }

    fn move_guard_right(&self, guard: &Guard) -> Option<Guard> {
        // If we are at the right of the maze we are done
        if guard.get_position().y == self.maze_cols - 1 {
            return None;
        }

        match self.maze[guard.get_position().x][guard.get_position().y + 1] {
            MazeObject::Empty => {
                // Move right
                let mut guard_right = guard.clone();
                guard_right.get_position_mut().y += 1;

                Some(guard_right)
            }
            MazeObject::Obstruction | MazeObject::NewObstruction => {
                // Rotate down and move down (90 degrees right)
                let mut guard_down = guard.clone();
                *guard_down.get_direction_mut() = Direction::Down;

                self.move_guard_down(&guard_down)
            }
        }
    }

    fn move_guard_down(&self, guard: &Guard) -> Option<Guard> {
        // If we are at the bottom of the maze we are done
        if guard.get_position().x == self.maze_rows - 1 {
            return None;
        }

        match self.maze[guard.get_position().x + 1][guard.get_position().y] {
            MazeObject::Empty => {
                // Move right
                let mut guard_down = guard.clone();
                guard_down.get_position_mut().x += 1;

                Some(guard_down)
            }
            MazeObject::Obstruction | MazeObject::NewObstruction => {
                // Rotate left and move left (90 degrees right)
                let mut guard_left = guard.clone();
                *guard_left.get_direction_mut() = Direction::Left;

                self.move_guard_left(&guard_left)
            }
        }
    }

    fn move_guard_left(&self, guard: &Guard) -> Option<Guard> {
        // If we are at the left of the maze we are done
        if guard.get_position().y == 0 {
            return None;
        }

        match self.maze[guard.get_position().x][guard.get_position().y - 1] {
            MazeObject::Empty => {
                // Move left
                let mut guard_left = guard.clone();
                guard_left.get_position_mut().y -= 1;

                Some(guard_left)
            }
            MazeObject::Obstruction | MazeObject::NewObstruction => {
                // Rotate up and move up (90 degrees right)
                let mut guard_up = guard.clone();
                *guard_up.get_direction_mut() = Direction::Up;

                self.move_guard_up(&guard_up)
            }
        }
    }

    pub fn find_obstructions_count(&self) -> usize {
        // Spawn new puzzle with a new obstacle and investigate if it contains a loop
        let mut loops = 0;

        for i in 0..self.maze_rows {
            for j in 0..self.maze_cols {
                // Skip if position is occupied by Obstruction or Guard
                if self.maze[i][j] == MazeObject::Obstruction
                    || (i == self.guard.get_position().x && j == self.guard.get_position().y)
                {
                    continue;
                }

                let mut maze = self.clone();
                if maze.investigate_loop(Position::new(i, j)) {
                    loops += 1;
                }
            }
        }

        loops
    }

    fn investigate_loop(&mut self, new_obstruction: Position) -> bool {
        // And single extra obstacle
        self.insert_new_obstruction(new_obstruction);

        // Track visited positions and directions
        let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::new();
        let dirs = visited
            .entry(self.guard.get_position().clone())
            .or_default();
        dirs.insert(self.guard.get_direction().clone());

        // Keep track of the new position
        let mut guard = Some(self.guard.clone());

        while let Some(new_guard) = self.move_guard(&guard) {
            // Continue with movement
            let dirs = visited.entry(new_guard.get_position().clone()).or_default();

            // If we are moving in the same direction as previously we know we are in the loop
            if dirs.contains(new_guard.get_direction()) {
                return true;
            }

            // Update visited direction
            dirs.insert(new_guard.get_direction().clone());
            guard = Some(new_guard);
        }

        // No loop detected
        false
    }

    fn insert_new_obstruction(&mut self, pos: Position) {
        // Remove previous obstruction
        if let Some(current_pos) = &self.new_obstacle_position {
            self.maze[current_pos.x][current_pos.y] = MazeObject::Empty;
        }

        // Insert new obstruction
        self.new_obstacle_position = Some(pos.clone());
        self.maze[pos.x][pos.y] = MazeObject::NewObstruction;
    }
}

#[cfg(test)]
mod tests {

    use crate::puzzle::parser::Parser;

    use super::*;

    fn create_maze() -> Maze {
        Parser::parse_lines(&[
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ])
        .unwrap_or_else(|err| {
            panic!(
                "Failed to create maze from string data with an error '{}'",
                err
            )
        })
    }

    #[test]
    fn test_move_guard_distinct_position_count() {
        let maze = create_maze();

        assert_eq!(maze.move_guard_distinct_position_count(), 41);
    }

    #[test]
    fn test_investigate_loop() {
        let mut maze = create_maze();

        assert!(!maze.investigate_loop(Position::new(0, 0)));
        assert!(!maze.investigate_loop(Position::new(0, 9)));
        assert!(!maze.investigate_loop(Position::new(9, 0)));
        assert!(!maze.investigate_loop(Position::new(9, 9)));

        assert!(maze.investigate_loop(Position::new(6, 3)));
        assert!(maze.investigate_loop(Position::new(7, 6)));
        assert!(maze.investigate_loop(Position::new(7, 7)));
        assert!(maze.investigate_loop(Position::new(8, 1)));
        assert!(maze.investigate_loop(Position::new(8, 3)));
        assert!(maze.investigate_loop(Position::new(9, 7)));
    }

    #[test]
    fn test_find_obstructions_count() {
        let maze = create_maze();

        assert_eq!(maze.find_obstructions_count(), 6);
    }
}
