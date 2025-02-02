use advent_of_code::{
    grids::{grid::Grid, point::Point},
    puzzles::{
        puzzle::{PuzzleResult, SolutionResult},
        puzzle_solver::PuzzleSolver,
    },
};

use super::{parser::Parser, ram::Ram};

const CORRUPTED_BYTES: usize = 1024;
const RAM_ROWS: usize = 71;
const RAM_COLS: usize = 71;

pub struct Solver {
    ram: Ram,
    corrupted: Vec<Point>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        // Create grid with all bytes allowed
        let grid = Grid::new_with(RAM_ROWS, RAM_COLS, |_| '.')
            .unwrap_or_else(|err| panic!("Failed to create grid [{:?}]", err));

        Self {
            ram: Ram::new(grid),
            corrupted: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 18: RAM Run ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.corrupted = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let minimum_steps = self
            .ram
            .count_minimum_steps(&self.corrupted, CORRUPTED_BYTES)?;

        Ok(minimum_steps.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let falling_byte = self
            .ram
            .find_first_falling_byte(&self.corrupted, CORRUPTED_BYTES)?;

        Ok(format!("{},{}", falling_byte.x, falling_byte.y))
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "330";
    const SOLUTION_2: &str = "10,38";

    fn get_tester() -> &'static PuzzleTester<Solver> {
        static TESTER: LazyLock<PuzzleTester<Solver>> =
            LazyLock::new(|| PuzzleTester::new(SOLUTION_1, SOLUTION_2));

        &TESTER
    }

    #[test]
    fn test_part_1() {
        get_tester().test_part_1();
    }

    #[test]
    fn test_part_2() {
        get_tester().test_part_2();
    }
}
