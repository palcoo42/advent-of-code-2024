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
        let sum_middles = self.print_queue.count_middle_pages_in_correct_order();
        Ok(sum_middles.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }

    fn part_3(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}
