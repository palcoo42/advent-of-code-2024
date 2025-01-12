use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, print_queue::PrintQueue};

pub struct Solver {
    print_queue: PrintQueue,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            print_queue: PrintQueue::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 5: Print Queue ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.print_queue = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let sum_middles = self.print_queue.count_middle_pages_in_order();
        Ok(sum_middles.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let sum_middles_only_fixed = self.print_queue.count_middle_pages_in_only_fixed_order();
        Ok(sum_middles_only_fixed.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "6384";
    const SOLUTION_2: &str = "5353";

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
