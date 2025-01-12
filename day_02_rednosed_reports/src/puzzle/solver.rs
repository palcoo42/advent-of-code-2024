use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, report::Report};

pub struct Solver {
    reports: Vec<Report>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 2: Red-Nosed Reports ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.reports = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let safe_reports_count = self.reports.iter().filter(|&r| r.is_safe()).count();

        Ok(safe_reports_count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let safe_reports_count = self
            .reports
            .iter()
            .filter(|&r| r.is_safe_problem_dampener())
            .count();

        Ok(safe_reports_count.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "407";
    const SOLUTION_2: &str = "459";

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
