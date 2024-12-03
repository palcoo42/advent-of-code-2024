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
    use super::*;

    fn create_solver() -> Solver {
        let mut solver = Solver::new();

        solver.instructions = vec![
            Instruction::Multiply(2, 4),
            Instruction::DoNot,
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Do,
            Instruction::Multiply(8, 5),
        ];

        solver
    }

    #[test]
    fn test_part_1() {
        let solver = create_solver();

        let result = solver.part_1();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(solver.part_1().unwrap(), "161");
    }

    #[test]
    fn test_part_2() {
        let solver = create_solver();

        let result = solver.part_2();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(solver.part_2().unwrap(), "48");
    }
}
