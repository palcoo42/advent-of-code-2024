use advent_of_code::puzzles::puzzle_error::PuzzleError;

use crate::puzzle::registers::Registers;

#[derive(Debug)]
pub struct ComboOperand {
    pub value: usize,
}

impl ComboOperand {
    pub fn new(operand: &str, registers: &Registers) -> Result<Self, PuzzleError> {
        let three_bit_number = operand.parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to parse combo operand '{}' to usize [{:?}]",
                operand, err
            ))
        })?;

        // Check number bounds -> 7 is not allowed
        if three_bit_number > 6 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Combo operand '{}' is out of bounds <0; 6>",
                operand
            )));
        }

        // Map value for combo operand
        let value = match three_bit_number {
            0..=3 => three_bit_number,
            4 => registers.a,
            5 => registers.b,
            6 => registers.c,
            _ => {
                return Err(PuzzleError::GenericError(format!(
                    "Unexpected three bit number '{}'",
                    three_bit_number,
                )))
            }
        };

        Ok(Self { value })
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::registers_builder::RegistersBuilder;

    use super::*;

    #[test]
    fn test_new() {
        const REGISTER_A: usize = 44;
        const REGISTER_B: usize = 55;
        const REGISTER_C: usize = 66;

        let registers = RegistersBuilder::new()
            .a(REGISTER_A)
            .b(REGISTER_B)
            .c(REGISTER_C)
            .build();

        let inputs = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", REGISTER_A),
            ("5", REGISTER_B),
            ("6", REGISTER_C),
        ];

        for (operand, expected) in inputs {
            let result = ComboOperand::new(operand, &registers);

            assert!(result.is_ok(), "operand: {} result: {:?}", operand, result);
            assert_eq!(result.unwrap().value, expected, "operand: {}", operand);
        }
    }

    #[test]
    fn test_new_invalid_number() {
        let registers = Registers::default();

        let result = ComboOperand::new("7", &registers);
        assert!(result.is_err(), "result: {:?}", result);

        let result = ComboOperand::new("42", &registers);
        assert!(result.is_err(), "result: {:?}", result);

        let result = ComboOperand::new("-1", &registers);
        assert!(result.is_err(), "result: {:?}", result);

        let result = ComboOperand::new("-42", &registers);
        assert!(result.is_err(), "result: {:?}", result);
    }

    #[test]
    fn test_new_non_number() {
        let registers = Registers::default();

        let result = ComboOperand::new("a", &registers);
        assert!(result.is_err(), "result: {:?}", result);

        let result = ComboOperand::new("uuups", &registers);
        assert!(result.is_err(), "result: {:?}", result);
    }
}
