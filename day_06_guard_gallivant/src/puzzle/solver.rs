use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{maze::Maze, parser::Parser};

pub struct Solver {
    maze: Maze,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            maze: Maze::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 6: Guard Gallivant ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.maze = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let count = self.maze.collect_guard_moves().len();
        Ok(count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let count = self.maze.find_obstructions_count();
        Ok(count.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "5212";
    const SOLUTION_2: &str = "1767";

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
