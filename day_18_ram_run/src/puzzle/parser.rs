use advent_of_code::{grids::point::Point, puzzles::puzzle_error::PuzzleError};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Point>, PuzzleError> {
        lines.iter().map(|line| Parser::parse_point(line)).collect()
    }

    fn parse_point(line: &str) -> Result<Point, PuzzleError> {
        let splitted = line.trim().split(",").collect::<Vec<_>>();

        if splitted.len() != 2 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Invalid format of corrupted RAM point '{}'",
                line
            )));
        }

        let x = splitted[0].trim().parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to parse position x '{}' to usize [{:?}]",
                splitted[0], err
            ))
        })?;

        let y = splitted[1].trim().parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to parse position y '{}' to usize [{:?}]",
                splitted[1], err
            ))
        })?;

        Ok(Point {
            x: x as isize,
            y: y as isize,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let result = Parser::parse_point("1,42");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), Point { x: 1, y: 42 });
    }

    #[test]
    fn test_parse_point_no_delimiter() {
        let result = Parser::parse_point("142");

        assert!(result.is_err(), "result: {:?}", result);
    }

    #[test]
    fn test_parse_point_non_number() {
        let result = Parser::parse_point("1,4a");

        assert!(result.is_err(), "result: {:?}", result);
    }
}
