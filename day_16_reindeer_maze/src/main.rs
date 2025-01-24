use advent_of_code::{
    env::project::Project,
    puzzles::puzzle::{Puzzle, PuzzleResult},
};
use puzzle::solver::Solver;

pub mod puzzle;

fn main() -> PuzzleResult {
    let input_file = Project::new().resource_file("input.txt");
    let mut puzzle = Puzzle::<Solver>::new_with_reader(&input_file);

    puzzle.solve()
}
