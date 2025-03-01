use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{parser::Parser, race::Race};

pub struct Solver {
    race: Race,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            race: Race::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 20: Race Condition ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.race = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let count = self.race.count_cheats(100, 2)?;
        Ok(count.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let count = self.race.count_cheats(100, 20)?;
        Ok(count.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "1497";
    const SOLUTION_2: &str = "1030809";

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
