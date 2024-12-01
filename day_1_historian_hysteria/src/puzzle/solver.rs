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
        Ok(String::from("not solved"))
    }
}
