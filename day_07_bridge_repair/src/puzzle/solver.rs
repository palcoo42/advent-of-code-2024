use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{equation::Equation, parser::Parser};

pub struct Solver {
    equations: Vec<Equation>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            equations: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 7: Bridge Repair ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.equations = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let result = self
            .equations
            .iter()
            .filter_map(|equation| match equation.solve().is_empty() {
                true => None,
                false => Some(equation.get_calibration()),
            })
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }

    fn part_3(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}
