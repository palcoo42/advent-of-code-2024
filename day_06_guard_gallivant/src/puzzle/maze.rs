use std::io::Write;
use std::{collections::HashSet, fs::File};

use super::position::Position;
use super::{direction::Direction, guard::Guard, maze_object::MazeObject};

#[derive(Debug)]
pub struct Maze {
    maze: Vec<Vec<MazeObject>>,
    maze_rows: usize,
    maze_cols: usize,
    guard: Guard,
}

impl Default for Maze {
    fn default() -> Self {
        Self {
            maze: Vec::new(),
            maze_rows: 0,
            maze_cols: 0,
            guard: Guard::new(super::position::Position::new(0, 0), Direction::Down),
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
        }
    }

    pub fn move_guard_distinct_position_count(&self) -> usize {
        // Track visited positions
        let mut visited = HashSet::new();
        visited.insert(self.guard.get_position().clone());

        // Keep track of the new position
        let mut guard = Some(self.guard.clone());

        while let Some(new_guard) = self.move_guard(&guard) {
            visited.insert(new_guard.get_position().clone());
            guard = Some(new_guard);
        }

        const PRINT_TO_FILE: bool = false;
        if PRINT_TO_FILE {
            self.print_to_file(&visited);
        }

        // Count visited positions
        visited.len()
    }

    fn print_to_file(&self, visited: &HashSet<Position>) {
        // Print visited locations into the file
        let mut file = File::create("/tmp/log.txt").unwrap();

        for i in 0..self.maze_rows {
            for j in 0..self.maze_cols {
                match self.maze[i][j] {
                    MazeObject::Empty => {
                        let c = if self.guard.get_position().x == i
                            && self.guard.get_position().y == j
                        {
                            match self.guard.get_direction() {
                                Direction::Right => ">",
                                Direction::Down => "v",
                                Direction::Left => "<",
                                Direction::Up => "^",
                            }
                        } else if visited.contains(&Position::new(i, j)) {
                            "o"
                        } else {
                            "."
                        };

                        write!(file, "{}", c).unwrap()
                    }
                    MazeObject::Obstruction => write!(file, "#").unwrap(),
                }
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
            MazeObject::Obstruction => {
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
            MazeObject::Obstruction => {
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
            MazeObject::Obstruction => {
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
            MazeObject::Obstruction => {
                // Rotate up and move up (90 degrees right)
                let mut guard_up = guard.clone();
                *guard_up.get_direction_mut() = Direction::Up;

                self.move_guard_up(&guard_up)
            }
        }
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
}
