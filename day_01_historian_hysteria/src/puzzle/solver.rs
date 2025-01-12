use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{locations::Locations, parser::Parser};

pub struct Solver {
    locations: Locations,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            locations: Default::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 1: Historian Hysteria ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.locations = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        Ok(self.locations.get_total_distance().to_string())
    }

    fn part_2(&self) -> SolutionResult {
        Ok(self.locations.get_similarity_score().to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "2378066";
    const SOLUTION_2: &str = "18934359";

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
