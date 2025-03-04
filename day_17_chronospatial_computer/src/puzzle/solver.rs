use std::cell::RefCell;

use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{computer::Computer, parser::Parser};

pub struct Solver {
    computer: RefCell<Computer>,
    program: String,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            computer: RefCell::new(Computer::default()),
            program: String::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 17: Chronospatial Computer ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        (*self.computer.borrow_mut(), self.program) = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let program_output = self.computer.borrow_mut().run_program(&self.program)?;
        Ok(program_output)
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "2,7,2,5,1,2,7,3,7";
    const SOLUTION_2: &str = "Not solved";

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
