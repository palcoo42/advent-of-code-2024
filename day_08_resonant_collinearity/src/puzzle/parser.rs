use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::grid::Grid;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Grid, PuzzleError> {
        let grid = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Grid::new(grid))
    }
}
