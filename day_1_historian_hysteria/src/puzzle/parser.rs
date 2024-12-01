use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::locations::Locations;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Locations, PuzzleError> {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in lines {
            let (l, r) = Self::decode_line(line)?;

            left.push(l);
            right.push(r);
        }

        Ok(Locations::new(left, right))
    }

    fn decode_line(line: &str) -> Result<(usize, usize), PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"^(\w+)\s+(\w+)").expect("Failed to create 'Location' regex")
        });

        if let Some(captures) = RE.captures(line) {
            let left = captures[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse 'left' '{}' to usize with an error '{}'",
                    &captures[1], err
                ))
            })?;

            let right = captures[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse 'right' '{}' to usize with an error '{}'",
                    &captures[2], err
                ))
            })?;

            return Ok((left, right));
        }

        Err(PuzzleError::InvalidContentError(format!(
            "Failed to decode 'Location' from line '{}'",
            line
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let lines = ["1 2", "3 4", "5 6"];

        let result = Parser::parse_lines(&lines);

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.as_ref().unwrap().get_left(), &[1, 3, 5]);
        assert_eq!(result.as_ref().unwrap().get_right(), &[2, 4, 6]);
    }

    #[test]
    fn decode_line() {
        let result = Parser::decode_line("42 24");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.as_ref().unwrap(), &(42, 24));
    }
}
