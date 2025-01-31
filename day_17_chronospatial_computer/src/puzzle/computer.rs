use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::instructions::instruction_decoder::InstructionDecoder;

use super::registers::Registers;

pub type ComputerOutput = Vec<usize>;

#[derive(Debug, Default)]
pub struct Computer {
    registers: Registers,
    instruction_pointer: usize,
    output: ComputerOutput,
}

impl Computer {
    pub fn new(registers: Registers) -> Self {
        Self {
            registers,
            instruction_pointer: 0,
            output: ComputerOutput::with_capacity(1024),
        }
    }

    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }

    pub fn run_program(&mut self, program: &str) -> Result<String, PuzzleError> {
        // Split program to instructions so we can easily access them with instruction pointer
        let instructions = program.split_terminator(",").collect::<Vec<_>>();

        while self.instruction_pointer < instructions.len() {
            // Read two instruction - opcode + operand
            let opcode = instructions[self.instruction_pointer];
            let operand = instructions[self.instruction_pointer + 1];

            let mut instruction = InstructionDecoder::decode(opcode)?;
            instruction.execute(operand, &mut self.registers, &mut self.output)?;

            // Move instruction pointer based on the instruction itself
            self.instruction_pointer =
                instruction.get_instruction_pointer(self.instruction_pointer);
        }

        // Convert numeric results to output string - numbers separated by ,
        Ok(self
            .output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(","))
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::registers_builder::RegistersBuilder;

    use super::*;

    #[test]
    fn test_run_program_example_1() {
        let registers = RegistersBuilder::new().c(9).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("2,6");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "");
        assert_eq!(computer.registers.a, 0);
        assert_eq!(computer.registers.b, 1);
        assert_eq!(computer.registers.c, 9);
    }

    #[test]
    fn test_run_program_example_2() {
        let registers = RegistersBuilder::new().a(10).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("5,0,5,1,5,4");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "0,1,2");
        assert_eq!(computer.registers.a, 10);
        assert_eq!(computer.registers.b, 0);
        assert_eq!(computer.registers.c, 0);
    }

    #[test]
    fn test_run_program_example_3() {
        let registers = RegistersBuilder::new().a(2024).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("0,1,5,4,3,0");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.registers.a, 0);
        assert_eq!(computer.registers.b, 0);
        assert_eq!(computer.registers.c, 0);
    }

    #[test]
    fn test_run_program_example_4() {
        let registers = RegistersBuilder::new().b(29).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("1,7");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "");
        assert_eq!(computer.registers.a, 0);
        assert_eq!(computer.registers.b, 26);
        assert_eq!(computer.registers.c, 0);
    }

    #[test]
    fn test_run_program_example_5() {
        let registers = RegistersBuilder::new().b(2024).c(43690).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("4,0");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "");
        assert_eq!(computer.registers.a, 0);
        assert_eq!(computer.registers.b, 44354);
        assert_eq!(computer.registers.c, 43690);
    }

    #[test]
    fn test_run_program() {
        let registers = RegistersBuilder::new().a(729).build();
        let mut computer = Computer::new(registers);

        let result = computer.run_program("0,1,5,4,3,0");

        assert!(result.is_ok(), "result: {:?}", result);
        assert_eq!(result.unwrap(), "4,6,3,5,6,3,5,2,1,0");
    }
}
