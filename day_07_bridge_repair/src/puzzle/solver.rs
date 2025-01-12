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
            .filter_map(
                |equation| match equation.solve_without_concatenation().is_empty() {
                    true => None,
                    false => Some(equation.get_calibration()),
                },
            )
            .sum::<usize>();

        Ok(result.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let result = self
            .equations
            .iter()
            .filter_map(
                |equation| match equation.solve_with_concatenation().is_empty() {
                    true => None,
                    false => Some(equation.get_calibration()),
                },
            )
            .sum::<usize>();

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "1611660863222";
    const SOLUTION_2: &str = "945341732469724";

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
