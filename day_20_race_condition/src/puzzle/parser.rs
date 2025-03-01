use advent_of_code::{grids::grid::Grid, puzzles::puzzle_error::PuzzleError};

use super::race::Race;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Race, PuzzleError> {
        let grid = Grid::new_from_lines(lines)?;
        Ok(Race::new(grid))
    }
}
