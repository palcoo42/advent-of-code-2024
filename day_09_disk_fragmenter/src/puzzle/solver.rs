use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{disk_map::DiskMap, parser::Parser};

pub struct Solver {
    disk_map: DiskMap,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            disk_map: Default::default(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 9: Disk Fragmenter ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        self.disk_map = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        let checksum = self.disk_map.compact_per_block_get_checksum();
        Ok(checksum.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let checksum = self.disk_map.compact_per_file_get_checksum();
        Ok(checksum.to_string())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::LazyLock;

    use advent_of_code::puzzles::puzzle_tester::PuzzleTester;

    use super::*;

    const SOLUTION_1: &str = "6390180901651";
    const SOLUTION_2: &str = "6412390114238";

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
