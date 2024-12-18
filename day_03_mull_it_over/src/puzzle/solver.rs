use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{instruction::Instruction, parser::Parser};

pub struct Solver {
    instructions: Vec<Instruction>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 3: Mull It Over ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.instructions = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        // Ignore Do and DoNot
        let mul = self
            .instructions
            .iter()
            .filter_map(|instr| match instr {
                Instruction::Multiply(first, second) => Some(first * second),
                Instruction::Do | Instruction::DoNot => None,
            })
            .sum::<usize>();

        Ok(mul.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        // Take into account Do and DoNot
        let mut action = Instruction::Do;

        let mul = self
            .instructions
            .iter()
            .filter_map(|instr| match instr {
                Instruction::Multiply(first, second) => match action {
                    Instruction::Do => Some(first * second),
                    Instruction::DoNot => None,
                    Instruction::Multiply(_, _) => panic!("Not allowed action"),
                },
                Instruction::Do | Instruction::DoNot => {
                    action = instr.clone();
                    None
                }
            })
            .sum::<usize>();

        Ok(mul.to_string())
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
        assert_eq!(result.unwrap(), String::from("187194524"));
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
        assert_eq!(result.unwrap(), String::from("127092535"));
    }
}
