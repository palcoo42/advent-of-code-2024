use std::cell::RefCell;

use super::{position::Position, quadrant::Quadrant, robot::Robot};

#[derive(Debug, Default, PartialEq)]
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
    fn print(&self) {
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
