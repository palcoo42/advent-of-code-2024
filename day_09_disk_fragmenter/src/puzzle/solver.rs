use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{disk_map::DiskMap, parser::Parser};

pub struct Solver {
    disk_map: DiskMap,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            disk_map: Default::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 9: Disk Fragmenter ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.disk_map = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let checksum = self.disk_map.compact_per_block_get_checksum();
        Ok(checksum.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let checksum = self.disk_map.compact_per_file_get_checksum();
        Ok(checksum.to_string())
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
        assert_eq!(result.unwrap(), String::from("6390180901651"));
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
        assert_eq!(result.unwrap(), String::from("6412390114238"));
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
