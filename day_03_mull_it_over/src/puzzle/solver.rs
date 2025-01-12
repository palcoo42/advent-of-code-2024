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

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "187194524";
    const SOLUTION_2: &str = "127092535";

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
