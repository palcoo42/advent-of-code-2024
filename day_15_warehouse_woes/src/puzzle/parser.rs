use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{movement::Movement, tile::Tile, warehouse::Warehouse};

enum ParserState {
    Warehouse,
    Movements,
}

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<(Warehouse, Vec<Movement>), PuzzleError> {
        // Note: In input file are two parts separated by an empty line:
        // - Warehouse layout
        //   ...
        // - One empty line
        // - Movements
        //   ...

        let mut state = ParserState::Warehouse;
        let mut warehouse_lines = Vec::with_capacity(lines.len());
        let mut movements_lines = Vec::with_capacity(lines.len());

        for &line in lines {
            match state {
                ParserState::Warehouse => warehouse_lines.push(line),
                ParserState::Movements => movements_lines.push(line),
            }

            // Detect an empty line to change parser state
            if line.is_empty() {
                state = ParserState::Movements;
            }
        }

        let warehouse = Self::parse_warehouse(&warehouse_lines)?;
        let movements = Self::parse_movements(&movements_lines)?;

        Ok((warehouse, movements))
    }

    pub fn parse_warehouse(lines: &[&str]) -> Result<Warehouse, PuzzleError> {
        // Check for input validity
        if lines.is_empty() {
            return Err(PuzzleError::InvalidContentError(
                "Failed to create warehouse, lines are empty".to_string(),
            ));
        }

        let rows = lines.len();
        let cols = lines[0].len();
        let mut tiles = Vec::with_capacity(rows * cols);

        // Build vector with values
        for line in lines {
            for c in line.chars() {
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    other => {
                        return Err(PuzzleError::InvalidContentError(format!(
                            "Failed to create warehouse, invalid character '{}'",
                            other
                        )))
                    }
                };

                tiles.push(tile);
            }
        }

        Ok(Warehouse::new(rows, cols, tiles))
    }

    pub fn parse_movements(lines: &[&str]) -> Result<Vec<Movement>, PuzzleError> {
        let mut movements = Vec::with_capacity(lines.len() * 100);

        for line in lines {
            for c in line.chars() {
                let movement = match c {
                    '>' => Movement::Right,
                    'v' => Movement::Down,
                    '<' => Movement::Left,
                    '^' => Movement::Up,
                    other => {
                        return Err(PuzzleError::InvalidContentError(format!(
                            "Failed to create movements, invalid character '{}'",
                            other
                        )))
                    }
                };

                movements.push(movement);
            }
        }

        Ok(movements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_warehouse() {
        let lines = [
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ];

        let expected_warehouse = Warehouse::new(
            8,
            8,
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Robot,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Wall,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Box,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Empty,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
            ],
        );

        let result = Parser::parse_warehouse(&lines);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), expected_warehouse);
    }

    #[test]
    fn test_parse_movements() {
        let lines = [">v<^", "^<v>"];

        let expected_movements = vec![
            Movement::Right,
            Movement::Down,
            Movement::Left,
            Movement::Up,
            Movement::Up,
            Movement::Left,
            Movement::Down,
            Movement::Right,
        ];
        let result = Parser::parse_movements(&lines);

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), expected_movements);
    }
}
