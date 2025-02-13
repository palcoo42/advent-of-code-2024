use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::designer::Designer;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Designer, PuzzleError> {
        // Check file length
        if lines.len() < 3 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Input file should contain at least 3 lines [{} found]",
                lines.len()
            )));
        }

        // First line contains towel patterns
        let towel_patterns = lines[0]
            .split(",")
            .map(|pat| pat.trim().to_string())
            .collect();

        // Lines 3 until the end of the file contains designs
        let designs = lines.iter().skip(2).map(|line| line.to_string()).collect();

        Ok(Designer::new(towel_patterns, designs))
    }
}
