use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{
    computer::ComputerOutput, operands::combo_operand::ComboOperand, registers::Registers,
};

use super::instruction::Instruction;

#[derive(Debug, Default)]
pub struct Cdv {}

impl Cdv {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for Cdv {
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        _output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        let numerator = registers.a;
        let combo_operand = ComboOperand::new(operand, registers)?;
        let denominator = 2_usize.pow(combo_operand.value as u32);

        registers.c = numerator / denominator;

        Ok(())
    }
}
