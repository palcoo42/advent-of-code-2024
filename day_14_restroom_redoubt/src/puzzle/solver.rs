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
        "--- Day 14: Restroom Redoubt ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.grid = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let safety_factor = self.grid.safety_factor(100);
        Ok(safety_factor.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "229980828";
    const SOLUTION_2: &str = "Not solved";

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
