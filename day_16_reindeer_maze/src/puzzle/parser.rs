use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{maze::Maze, tile::Tile};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Maze, PuzzleError> {
        if lines.is_empty() {
            return Err(PuzzleError::InvalidContentError(
                "Lines are empty".to_string(),
            ));
        }

        let height = lines.len();
        let width = lines[0].len();

        let maze: Result<Vec<Vec<Tile>>, PuzzleError> =
            lines.iter().map(|line| Self::parse_line(line)).collect();

        Ok(Maze::new(height, width, maze?))
    }

    fn parse_line(line: &str) -> Result<Vec<Tile>, PuzzleError> {
        line.chars()
            .map(|c| match c {
                '.' => Ok(Tile::Empty),
                '#' => Ok(Tile::Wall),
                'S' => Ok(Tile::Start),
                'E' => Ok(Tile::End),
                _ => Err(PuzzleError::InvalidContentError(format!(
                    "Invalid character '{}'",
                    c
                ))),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = Parser::parse_line(".#SE");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            vec![Tile::Empty, Tile::Wall, Tile::Start, Tile::End]
        )
    }
}
