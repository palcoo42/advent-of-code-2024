use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, topographic_map::TopographicMap};

pub struct Solver {
    topographic_map: TopographicMap,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            topographic_map: TopographicMap::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 10: Hoof It ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.topographic_map = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let count = self.topographic_map.count_trail_heads_score();
        Ok(count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let count = self.topographic_map.count_trail_heads_rating();
        Ok(count.to_string())
    }

    fn part_3(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
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

    /// Create a solver instance which can be used to validate parts algorithms.
    ///
    /// # Returns
    ///
    /// Mutex to solver because unit tests are executed in a different threads
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
                .expect("Failed to unwrap 'Solver Mutex'");

            result = solver.part_1();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), String::from("531"));
    }

    #[test]
    fn test_part_2() {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let solver = create_solver()
                .lock()
                .expect("Failed to unwrap 'Solver Mutex'");

            result = solver.part_2();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), String::from("1210"));
    }

    #[test]
    fn test_part_3() {
        let result;

        // Solve the puzzle inside a scope so that guard is released automatically avoiding a panic in the thread.
        {
            let solver = create_solver()
                .lock()
                .expect("Failed to unwrap 'Solver Mutex'");

            result = solver.part_3();
        }

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), String::from("Not solved"));
    }
}
