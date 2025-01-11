use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::topographic_map::TopographicMap;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<TopographicMap, PuzzleError> {
        let topography: Result<Vec<Vec<u8>>, PuzzleError> =
            lines.iter().map(|line| Parser::parse_line(line)).collect();

        Ok(TopographicMap::new(topography?))
    }

    fn parse_line(line: &str) -> Result<Vec<u8>, PuzzleError> {
        let mut digits = Vec::new();

        for c in line.chars() {
            match c.to_string().parse::<u8>() {
                Ok(number) => {
                    digits.push(number);
                }
                Err(err) => {
                    return Err(PuzzleError::InvalidContentError(format!(
                        "Failed to parse digit '{}' to u8 with an error '{}'",
                        c, err
                    )))
                }
            }
        }

        Ok(digits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = Parser::parse_line("0123456789");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_parse_line_error() {
        let result = Parser::parse_line("012345x6789");

        assert!(result.is_err(), "result: {:?}", result);
    }
}
