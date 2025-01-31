use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{
    computer::ComputerOutput, operands::literal_operand::LiteralOperand, registers::Registers,
};

use super::instruction::{Instruction, DEFAULT_INSTRUCTION_POINTER_OFFSET};

#[derive(Debug, Default)]
pub struct Jnz {
    jump_offset: Option<usize>,
}

impl Jnz {
    pub fn new() -> Self {
        Self { jump_offset: None }
    }
}

impl Instruction for Jnz {
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        _output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        match registers.a {
            0 => {
                self.jump_offset = None;
            }
            _ => {
                let literal_operand = LiteralOperand::new(operand)?;
                self.jump_offset = Some(literal_operand.value);
            }
        }
        Ok(())
    }

    fn get_instruction_pointer(&self, instruction_pointer: usize) -> usize {
        match self.jump_offset {
            Some(offset) => offset,
            None => instruction_pointer + DEFAULT_INSTRUCTION_POINTER_OFFSET,
        }
    }
}
