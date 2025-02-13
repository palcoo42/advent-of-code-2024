use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{designer::Designer, parser::Parser};

pub struct Solver {
    designer: Designer,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            designer: Designer::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 19: Linen Layout ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.designer = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let solutions = self.designer.count_unique_solutions();
        Ok(solutions.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let solutions = self.designer.count_all_solutions();
        Ok(solutions.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "255";
    const SOLUTION_2: &str = "621820080273474";

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
