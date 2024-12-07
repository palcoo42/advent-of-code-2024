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
        let visited_locations = self.maze.move_guard_distinct_position_count();
        Ok(visited_locations.to_string())
    }

    fn part_2(&self) -> SolutionResult {
        let obstructions_count = self.maze.find_obstructions_count();
        Ok(obstructions_count.to_string())
    }

    fn part_3(&self) -> SolutionResult {
        Ok(String::from("Not solved"))
    }
}
