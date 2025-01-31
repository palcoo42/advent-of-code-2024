use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::{
    computer::ComputerOutput, operands::combo_operand::ComboOperand, registers::Registers,
};

use super::instruction::Instruction;

#[derive(Default)]
pub struct Bst {}

// The bst instruction (opcode 2) calculates the value of its combo operand modulo 8
// (thereby keeping only its lowest 3 bits), then writes that value to the B register.
impl Bst {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for Bst {
    fn execute(
        &mut self,
        operand: &str,
        registers: &mut Registers,
        _output: &mut ComputerOutput,
    ) -> Result<(), PuzzleError> {
        let combo_operand = ComboOperand::new(operand, registers)?;
        registers.b = combo_operand.value % 8;
        Ok(())
    }
}
