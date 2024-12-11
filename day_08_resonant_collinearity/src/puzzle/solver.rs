use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{grid::Grid, parser::Parser};

pub struct Solver {
    grid: Grid,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            grid: Grid::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 8: Resonant Collinearity ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.grid = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let anti_nodes_len = self.grid.collect_anti_nodes().len();
        Ok(anti_nodes_len.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }

    fn part_3(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}
