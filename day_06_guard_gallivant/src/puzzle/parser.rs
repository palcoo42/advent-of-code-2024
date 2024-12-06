use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{
    direction::Direction, guard::Guard, maze::Maze, maze_object::MazeObject, position::Position,
};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Maze, PuzzleError> {
        let mut rows = Vec::with_capacity(lines.len());

        for line in lines {
            rows.push(Self::decode_row(line)?);
        }

        let guard = Self::find_guard(lines)?;

        Ok(Maze::new(rows, guard))
    }

    fn decode_row(line: &str) -> Result<Vec<MazeObject>, PuzzleError> {
        let mut row = Vec::with_capacity(line.chars().count());

        for c in line.chars() {
            match c {
                '.' | '<' | '>' | '^' | 'v' => row.push(MazeObject::Empty),
                '#' => row.push(MazeObject::Obstruction),
                c => {
                    return Err(PuzzleError::InvalidContentError(format!(
                        "Invalid character '{}' in maze input",
                        c
                    )))
                }
            }
        }

        Ok(row)
    }

    fn find_guard(lines: &[&str]) -> Result<Guard, PuzzleError> {
        for (row_idx, line) in lines.iter().enumerate() {
            if let Some(col_idx) = line.find(">") {
                return Ok(Guard::new(
                    Position::new(row_idx, col_idx),
                    Direction::Right,
                ));
            }

            if let Some(col_idx) = line.find("v") {
                return Ok(Guard::new(Position::new(row_idx, col_idx), Direction::Down));
            }

            if let Some(col_idx) = line.find("<") {
                return Ok(Guard::new(Position::new(row_idx, col_idx), Direction::Left));
            }

            if let Some(col_idx) = line.find("^") {
                return Ok(Guard::new(Position::new(row_idx, col_idx), Direction::Up));
            }
        }

        Err(PuzzleError::InvalidContentError(String::from(
            "Guard not found in the maze",
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_row() {
        let result = Parser::decode_row("..#.#");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            vec![
                MazeObject::Empty,
                MazeObject::Empty,
                MazeObject::Obstruction,
                MazeObject::Empty,
                MazeObject::Obstruction
            ]
        )
    }

    #[test]
    fn test_find_guard() {
        let result = Parser::find_guard(&["..>.."]);
        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            Guard::new(Position::new(0, 2), Direction::Right)
        );

        let result = Parser::find_guard(&["v...."]);
        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            Guard::new(Position::new(0, 0), Direction::Down)
        );

        let result = Parser::find_guard(&["....<"]);
        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            Guard::new(Position::new(0, 4), Direction::Left)
        );

        let result = Parser::find_guard(&[".^..."]);
        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            Guard::new(Position::new(0, 1), Direction::Up)
        );
    }
}
