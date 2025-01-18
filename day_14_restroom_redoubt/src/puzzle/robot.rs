use super::{position::Position, velocity::Velocity};

#[derive(Debug, Clone, PartialEq)]
pub struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    pub fn new(position: Position, velocity: Velocity) -> Self {
        Self { position, velocity }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn move_robot(&mut self, rows: usize, cols: usize, steps: usize) {
        let mut new_x = self.position.x as isize + self.velocity.x * steps as isize;
        let mut new_y = self.position.y as isize + self.velocity.y * steps as isize;

        // If we are out of bounds wrap around
        new_x = if new_x >= 0 {
            new_x % cols as isize
        } else {
            // Note: new_x < 0 so + will change to -
            match new_x % cols as isize {
                0 => 0,
                diff => cols as isize + diff,
            }
        };

        new_y = if new_y >= 0 {
            new_y % rows as isize
        } else {
            // Note: new_x < 0 so + will change to -
            match new_y % rows as isize {
                0 => 0,
                diff => rows as isize + diff,
            }
        };

        self.position = Position {
            x: new_x as usize,
            y: new_y as usize,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_robot_single_step() {
        const ROWS: usize = 7;
        const COLS: usize = 11;

        let robot = Robot::new(Position { x: 2, y: 4 }, Velocity { x: 2, y: -3 });

        let input = [
            (0, Position { x: 2, y: 4 }),
            (1, Position { x: 4, y: 1 }),
            (2, Position { x: 6, y: 5 }),
            (3, Position { x: 8, y: 2 }),
            (4, Position { x: 10, y: 6 }),
            (5, Position { x: 1, y: 3 }),
        ];

        for (steps, ref expected) in input {
            let mut test_robot = robot.clone();

            // Step 0 is initial state without movement
            if steps > 0 {
                test_robot.move_robot(ROWS, COLS, steps);
            }

            assert_eq!(
                expected, &test_robot.position,
                "Failed at '{}' steps",
                steps
            );
        }
    }

    #[test]
    fn test_move_robot_compare_single_vs_multiple_steps() {
        // Note: These two method has to have the same results
        const ROWS: usize = 7;
        const COLS: usize = 11;

        let robot = Robot::new(Position { x: 7, y: 6 }, Velocity { x: -1, y: -3 });

        for steps in 0..100 {
            let mut robot_1 = robot.clone();
            let mut robot_2 = robot.clone();

            // First method
            (0..steps).for_each(|_| robot_1.move_robot(ROWS, COLS, 1));

            // Second method
            robot_2.move_robot(ROWS, COLS, steps);

            assert_eq!(
                robot_1.get_position(),
                robot_2.get_position(),
                "Positions differs at {}",
                steps
            );
        }
    }
}
