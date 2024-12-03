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
        let mul = self
            .instructions
            .iter()
            .flat_map(|instr| instr.multiply())
            .sum::<usize>();

        Ok(mul.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("not solved"))
    }
}
