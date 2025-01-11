use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::pebbles::Pebbles;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Pebbles, PuzzleError> {
        if lines.len() != 1 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Input file should contain only single line but '{}'",
                lines.len()
            )));
        };

        let numbers: Result<Vec<usize>, PuzzleError> = lines[0]
            .split_ascii_whitespace()
            .map(|value| {
                value.parse::<usize>().map_err(|err| {
                    PuzzleError::InvalidContentError(format!(
                        "Failed to convert '{}' to usize with an error '{}'",
                        value, err
                    ))
                })
            })
            .collect();

        Ok(Pebbles::new(numbers?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let result = Parser::parse_lines(&["1 2 3"]);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Pebbles::new(vec![1, 2, 3]));
    }
}
