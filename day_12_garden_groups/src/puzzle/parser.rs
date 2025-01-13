use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::garden::Garden;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Garden, PuzzleError> {
        let grid: Result<Vec<Vec<char>>, PuzzleError> =
            lines.iter().map(|&line| Self::parse_line(line)).collect();

        Ok(Garden::new(grid?))
    }

    #[inline]
    fn parse_line(line: &str) -> Result<Vec<char>, PuzzleError> {
        Ok(line.chars().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = Parser::parse_line("ABBCCC");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), vec!['A', 'B', 'B', 'C', 'C', 'C']);
    }
}
