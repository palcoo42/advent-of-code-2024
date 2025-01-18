use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{position::Position, quadrant::Quadrant, robot::Robot};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Grid {
    rows: usize,
    cols: usize,
    robots: RefCell<Vec<Robot>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize, robots: Vec<Robot>) -> Self {
        // In the puzzle description is expected that number of rows and columns is odd
        assert!(rows % 2 == 1, "Number of rows has to be odd [{}]", rows);
        assert!(cols % 2 == 1, "Number of columns has to be odd [{}]", cols);

        Self {
            rows,
            cols,
            robots: RefCell::new(robots),
        }
    }

    pub fn safety_factor(&self, steps: usize) -> usize {
        // Move all robots
        self.move_robots(steps);

        // Calculate safety factor
        self.calculate_safety_factor()
    }

    fn move_robots(&self, steps: usize) {
        // Move all robots 'step' times
        self.robots
            .borrow_mut()
            .iter_mut()
            .for_each(|r| r.move_robot(self.rows, self.cols, steps));
    }

    fn calculate_safety_factor(&self) -> usize {
        let quadrants = self.split_to_quadrants();

        quadrants
            .iter()
            .fold(1, |value, quadrant| value * self.count_robots(quadrant))
    }

    // Split grid into four quadrants. We expect that number of rows and columns is odd (check in ctor).
    // We are deliberately skipping middle row and column.
    fn split_to_quadrants(&self) -> Vec<Quadrant> {
        let rows_split = self.rows / 2;
        let cols_split = self.cols / 2;

        vec![
            Quadrant {
                x: 0..rows_split,
                y: 0..cols_split,
            },
            Quadrant {
                x: 0..rows_split,
                y: cols_split + 1..(self.cols),
            },
            Quadrant {
                x: rows_split + 1..(self.rows),
                y: 0..cols_split,
            },
            Quadrant {
                x: rows_split + 1..(self.rows),
                y: cols_split + 1..(self.cols),
            },
        ]
    }

    // Note: rows and cols ranges are exclusive, i.e. end is not included
    fn count_robots(&self, quadrant: &Quadrant) -> usize {
        self.robots
            .borrow()
            .iter()
            .filter(|&r| {
                let pos = r.get_position();
                quadrant.x.contains(&pos.y) && quadrant.y.contains(&pos.x)
            })
            .count()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for r in 0..self.rows {
            for c in 0..self.cols {
                match self.count(r, c) {
                    0 => print!("."),
                    n => print!("{}", n),
                }
            }
            println!();
        }
    }

    fn count(&self, row: usize, col: usize) -> usize {
        self.robots
            .borrow()
            .iter()
            .filter(|&r| r.get_position() == &Position { x: col, y: row })
            .count()
    }

    pub fn print_to_file(&self, path: &Path) -> Result<(), PuzzleError> {
        // Create file for writing
        let mut file = File::create(path).map_err(|err| {
            PuzzleError::GenericError(format!(
                "Failed to create a file '{:?}' with and error '{}'",
                path, err
            ))
        })?;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let character = match self.count(r, c) {
                    0 => '.',
                    _ => '#',
                };

                write!(file, "{}", character).map_err(|err| {
                    PuzzleError::GenericError(format!(
                        "Failed to write to file '{:?}' with an error '{}'",
                        path, err,
                    ))
                })?;
            }

            writeln!(file).map_err(|err| {
                PuzzleError::GenericError(format!(
                    "Failed to write to file '{:?}' with an error '{}'",
                    path, err,
                ))
            })?;
        }

        Ok(())
    }

    pub fn find_possible_christmas_trees(
        &self,
        path: &Path,
        max: usize,
    ) -> Result<(), PuzzleError> {
        // Create directory structure if it does not exist yet
        fs::create_dir_all(path).map_err(|err| {
            PuzzleError::GenericError(format!(
                "Failed to create directory structure '{:?}' with an error '{}'",
                path, err
            ))
        })?;

        // Create text files with images
        for seconds in 1..max {
            // Move always only by a single step
            self.move_robots(1);

            // Print to a file
            if self.is_possible_christmas_tree() {
                let file = path.join(format!("{:04}.txt", seconds));
                self.print_to_file(&file)?;
            }
        }

        Ok(())
    }

    fn is_possible_christmas_tree(&self) -> bool {
        // If we have more than cca 33% of robots in a row this is interesting grid to check
        let limit: usize = self.cols / 3;

        let mut counts = HashMap::new();

        for robot in self.robots.borrow().iter() {
            let position = robot.get_position();

            let count = counts.entry(position.x).or_insert(0);
            *count += 1;
        }

        counts.iter().any(|(_, count)| count >= &limit)
    }
}

#[cfg(test)]
mod tests {

    use crate::puzzle::{position::Position, velocity::Velocity};

    use super::*;

    fn create_grid() -> Grid {
        Grid::new(
            7,
            11,
            vec![
                Robot::new(Position { x: 0, y: 4 }, Velocity { x: 3, y: -3 }),
                Robot::new(Position { x: 6, y: 3 }, Velocity { x: -1, y: -3 }),
                Robot::new(Position { x: 10, y: 3 }, Velocity { x: -1, y: 2 }),
                Robot::new(Position { x: 2, y: 0 }, Velocity { x: 2, y: -1 }),
                Robot::new(Position { x: 0, y: 0 }, Velocity { x: 1, y: 3 }),
                Robot::new(Position { x: 3, y: 0 }, Velocity { x: -2, y: -2 }),
                Robot::new(Position { x: 7, y: 6 }, Velocity { x: -1, y: -3 }),
                Robot::new(Position { x: 3, y: 0 }, Velocity { x: -1, y: -2 }),
                Robot::new(Position { x: 9, y: 3 }, Velocity { x: 2, y: 3 }),
                Robot::new(Position { x: 7, y: 3 }, Velocity { x: -1, y: 2 }),
                Robot::new(Position { x: 2, y: 4 }, Velocity { x: 2, y: -3 }),
                Robot::new(Position { x: 9, y: 5 }, Velocity { x: -3, y: -3 }),
            ],
        )
    }

    #[test]
    fn test_split_to_quadrants() {
        let grid = create_grid();

        assert_eq!(
            grid.split_to_quadrants(),
            vec![
                Quadrant { x: 0..3, y: 0..5 },
                Quadrant { x: 0..3, y: 6..11 },
                Quadrant { x: 4..7, y: 0..5 },
                Quadrant { x: 4..7, y: 6..11 }
            ]
        );
    }

    #[test]
    fn test_safety_factor() {
        let grid = create_grid();

        assert_eq!(grid.safety_factor(100), 12);
    }
}
