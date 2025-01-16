use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::{button::Button, claw_machine::ClawMachine, game::Game, prize::Prize};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Game, PuzzleError> {
        let mut game = Game::new();

        // Every claw machine is separated by an empty line and contains exactly 3 items
        let mut claw_machine_raw = Vec::with_capacity(3);

        for &line in lines {
            match line.is_empty() {
                false => claw_machine_raw.push(line),
                true => {
                    // Parse machine
                    let machine = Self::parse_claw_machine(&claw_machine_raw)?;
                    game.push(machine);

                    // Clear content for next machine
                    claw_machine_raw.clear();
                }
            }
        }

        Ok(game)
    }

    fn parse_claw_machine(lines: &[&str]) -> Result<ClawMachine, PuzzleError> {
        // Double check correct number of lines
        if lines.len() != 3 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Claw machine should contain exactly 3 lines, but {} found",
                lines.len()
            )));
        }

        // Example:
        // Button A: X+77, Y+52
        // Button B: X+14, Y+32
        // Prize: X=5233, Y=14652
        let button_a = Self::parse_button(lines[0])?;
        let button_b = Self::parse_button(lines[1])?;
        let prize = Self::parse_prize(lines[2])?;

        Ok(ClawMachine::new(button_a, button_b, prize))
    }

    fn parse_button(line: &str) -> Result<Button, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^Button [A,B]: X\+(\d+), Y\+(\d+)"#)
                .expect("Failed to create 'Button' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let x = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with and error '{}'",
                    &captures[1], err
                ))
            })?;

            let y = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with and error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Button { x, y });
        }

        Err(PuzzleError::GenericError(format!(
            "Failed to parse button, line: {}",
            line
        )))
    }

    fn parse_prize(line: &str) -> Result<Prize, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"^Prize: X=(\d+), Y=(\d+)"#).expect("Failed to create 'Prize' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let x = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with an error '{}'",
                    &captures[1], err
                ))
            })?;

            let y = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with an error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok(Prize { x, y });
        }

        Err(PuzzleError::GenericError(format!(
            "Failed to parse prize, line: {}",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_button_a() {
        let result = Parser::parse_button("Button A: X+77, Y+52");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Button { x: 77, y: 52 });
    }

    #[test]
    fn test_parse_button_b() {
        let result = Parser::parse_button("Button B: X+14, Y+32");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Button { x: 14, y: 32 });
    }

    #[test]
    fn test_parse_prize() {
        let result = Parser::parse_prize("Prize: X=5233, Y=14652");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Prize { x: 5233, y: 14652 });
    }
}
