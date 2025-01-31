use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{
    computer::ComputerOutput, operands::literal_operand::LiteralOperand, registers::Registers,
};

use super::instruction::Instruction;

#[derive(Debug, Default)]
pub struct Bxl {}

impl Bxl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for Bxl {
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        _output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        let literal_operand = LiteralOperand::new(operand)?;
        registers.b ^= literal_operand.value;
        Ok(())
    }
}
