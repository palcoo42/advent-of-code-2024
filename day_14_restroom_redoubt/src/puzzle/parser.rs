use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use crate::puzzle::{position::Position, velocity::Velocity};

use super::{grid::Grid, robot::Robot};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Grid, PuzzleError> {
        let robots: Result<Vec<Robot>, PuzzleError> =
            lines.iter().map(|line| Self::parse_line(line)).collect();

        Ok(Grid::new(HEIGHT, WIDTH, robots?))
    }

    fn parse_line(line: &str) -> Result<Robot, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^p=(\d+),(\d+)\s+v=(-?\d+),(-?\d+)"#)
                .expect("Failed to create 'Robot' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let pos_x = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse position x '{}' to usize with an error '{}'",
                    &captures[1], err
                ))
            })?;

            let pos_y = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse position y '{}' to usize with an error '{}'",
                    &captures[2], err
                ))
            })?;

            let vel_x = captures[3].parse::<isize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse velocity x '{}' to isize with an error '{}'",
                    &captures[3], err
                ))
            })?;

            let vel_y = captures[4].parse::<isize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse velocity y '{}' to isize with an error '{}'",
                    &captures[4], err
                ))
            })?;

            return Ok(Robot::new(
                Position { x: pos_x, y: pos_y },
                Velocity { x: vel_x, y: vel_y },
            ));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to parse robot from the line '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let result = Parser::parse_lines(&[
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ]);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            Grid::new(
                HEIGHT,
                WIDTH,
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
                ]
            )
        )
    }
}
