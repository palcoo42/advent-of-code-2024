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

    use std::{
        fs::File,
        io::{BufRead, BufReader},
        sync::{LazyLock, Mutex},
    };

    use advent_of_code::env::project::Project;

    use super::*;

    fn create_solver() -> &'static Mutex<Solver> {
        static SOLVER: LazyLock<Mutex<Solver>> = LazyLock::new(|| {
            // Read input file
            let input_file = Project::new().resource_file("input.txt");
            let file = File::open(&input_file)
                .unwrap_or_else(|err| panic!("Failed to open file with an error '{}'", err));
            let reader = BufReader::new(file);
            let lines = reader
                .lines()
                .collect::<Result<Vec<String>, _>>()
                .unwrap_or_else(|err| panic!("Failed to unwrap lines with an error '{}'", err));

            let lines: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();

            // Prepare solver
            let mut solver = Solver::new();

            // Parse input file
            solver
                .parse_input_file(&lines)
                .unwrap_or_else(|err| panic!("Failed to parse input file with error '{}'", err));

            Mutex::new(solver)
        });

        &SOLVER
    }

    #[test]
    fn test_part_1() {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let solver = create_solver()
                .lock()
                .expect("Failed to unwrap Solver Mutex");

            result = solver.part_1();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), String::from("407"));
    }

    #[test]
    fn test_part_2() {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let solver = create_solver()
                .lock()
                .expect("Failed to unwrap Solver Mutex");

            result = solver.part_2();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), String::from("459"));
    }
}
