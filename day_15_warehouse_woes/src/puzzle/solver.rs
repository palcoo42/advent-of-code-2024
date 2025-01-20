use std::cell::RefCell;

use advent_of_code::puzzles::{
    puzzle::{PuzzleResult, SolutionResult},
    puzzle_solver::PuzzleSolver,
};

use super::{movement::Movement, parser::Parser, warehouse::Warehouse};

pub struct Solver {
    warehouse: RefCell<Warehouse>,
    movements: Vec<Movement>,
}

impl PuzzleSolver for Solver {
    fn new() -> Self {
        Self {
            warehouse: RefCell::default(),
            movements: Vec::new(),
        }
    }

    fn get_description(&self) -> &str {
        "--- Day 15: Warehouse Woes ---"
    }

    fn parse_input_file(&mut self, lines: &[&str]) -> PuzzleResult {
        (*self.warehouse.borrow_mut(), self.movements) = Parser::parse_lines(lines)?;
        Ok(())
    }

    fn part_1(&self) -> SolutionResult {
        // Move stuff in a warehouse
        self.warehouse.borrow_mut().move_boxes(&self.movements);

        // Calculate GPS for all boxes
        let gps_coordinates = self.warehouse.borrow().gps_coordinates();

        Ok(gps_coordinates.to_string())
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

    const SOLUTION_1: &str = "1471826";
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
