use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{computer::ComputerOutput, registers::Registers};

use super::instruction::Instruction;

#[derive(Debug, Default)]
pub struct Bxc {}

impl Bxc {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for Bxc {
    fn execute(
        &mut self,
        _operand: &str,
        registers: &mut Registers,
        _output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        registers.b ^= registers.c;
        Ok(())
    }
}
