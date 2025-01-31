use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{computer::ComputerOutput, registers::Registers};

pub const DEFAULT_INSTRUCTION_POINTER_OFFSET: usize = 2;

pub trait Instruction {
    /// Execute instruction
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError>;

    /// Get offset of the instruction to apply
    ///
    /// Default value is + 2, i.e. next instruction, jump instructions may have different value
    fn get_instruction_pointer(&self, instruction_pointer: usize) -> usize {
        instruction_pointer + DEFAULT_INSTRUCTION_POINTER_OFFSET
    }
}
