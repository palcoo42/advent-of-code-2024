use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{grid::Grid, parser::Parser, part::Part};

pub struct Solver {
    grid: Grid,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            grid: Grid::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 8: Resonant Collinearity ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.grid = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        self.grid.set_algorithm(Part::Part1);
        let anti_nodes_len = self.grid.collect_anti_nodes().len();
        Ok(anti_nodes_len.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        self.grid.set_algorithm(Part::Part2);
        let anti_nodes_len = self.grid.collect_anti_nodes().len();
        Ok(anti_nodes_len.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "220";
    const SOLUTION_2: &str = "813";

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
