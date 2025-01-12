use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, pebbles::Pebbles};

pub struct Solver {
    pebbles: Pebbles,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            pebbles: Pebbles::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 11: Plutonian Pebbles ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.pebbles = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let stones_count = self.pebbles.blink_stones_count(25);
        Ok(stones_count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let stones_count = self.pebbles.blink_stones_count(75);
        Ok(stones_count.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "229043";
    const SOLUTION_2: &str = "272673043446478";

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
