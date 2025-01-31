use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{
    computer::ComputerOutput, operands::combo_operand::ComboOperand, registers::Registers,
};

use super::instruction::Instruction;

#[derive(Default)]
pub struct Out {}

impl Out {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for Out {
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        let combo_operand = ComboOperand::new(operand, registers)?;
        output.push(combo_operand.value % 8);
        Ok(())
    }
}
