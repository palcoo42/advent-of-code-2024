use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::disk_map::DiskMap;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<DiskMap, PuzzleError> {
        // We expect only a single line in the input file
        if lines.len() != 1 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Input file should contain only 1 line but {} found",
                lines.len()
            )));
        }

        Ok(DiskMap::new(lines[0]))
    }
}
