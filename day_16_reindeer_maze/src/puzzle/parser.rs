use advent_of_code::{grids::grid::Grid, puzzles::puzzle_error::PuzzleError};

use super::maze::Maze;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Maze, PuzzleError> {
        Ok(Maze::new(Grid::new_from_lines(lines)?))
    }
}
