use std::{cell::RefCell, path::Path};

use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{grid::Grid, parser::Parser};

pub struct Solver {
    grid: RefCell<Grid>,
    original: Grid,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            grid: RefCell::new(Grid::default()),
            original: Grid::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 14: Restroom Redoubt ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.original = Parser::parse_lines(lines)?;
        self.grid = RefCell::new(self.original.clone());
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let safety_factor = self.grid.borrow().safety_factor(100);
        Ok(safety_factor.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        // Restore grid because part 1 changed it
        *self.grid.borrow_mut() = self.original.clone();

        self.grid
            .borrow()
            .find_possible_christmas_trees(Path::new("/tmp/aoc-2024/day-14"), 10000)?;

        Ok(
            "Look for candidates under the directory '/tmp/aoc-2024/day-14' and find a solution"
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "229980828";
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

    // Note: Commented out because solution has to be found manually from generated grids
    // #[test]
    // fn test_part_2() {
    //     get_tester().test_part_2();
    // }
}
