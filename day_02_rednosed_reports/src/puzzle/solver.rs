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
