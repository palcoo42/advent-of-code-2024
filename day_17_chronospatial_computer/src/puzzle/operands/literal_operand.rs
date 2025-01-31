use advent_of_code::puzzles::puzzle_error::PuzzleError;

#[derive(Debug)]
pub struct LiteralOperand {
    pub value: usize,
}

impl LiteralOperand {
    pub fn new(operand: &str) -> Result<Self, PuzzleError> {
        let three_bit_number = operand.parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to parse literal operand '{}' to usize [{:?}]",
                operand, err
            ))
        })?;

        // Check number bounds
        if three_bit_number > 7 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Literal operand '{}' is out of bounds <0; 6>",
                operand
            )));
        }

        Ok(Self {
            value: three_bit_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let inputs = [
            ("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
        ];

        for (operand, expected) in inputs {
            let result = LiteralOperand::new(operand);

            assert!(result.is_ok(), "operand: {} result: {:?}", operand, result);
            assert_eq!(result.unwrap().value, expected, "operand: {}", operand);
        }
    }

    #[test]
    fn test_new_invalid_number() {
        let result = LiteralOperand::new("42");
        assert!(result.is_err(), "result: {:?}", result);

        let result = LiteralOperand::new("-1");
        assert!(result.is_err(), "result: {:?}", result);

        let result = LiteralOperand::new("-42");
        assert!(result.is_err(), "result: {:?}", result);
    }

    #[test]
    fn test_new_non_number() {
        let result = LiteralOperand::new("a");
        assert!(result.is_err(), "result: {:?}", result);

        let result = LiteralOperand::new("uuups");
        assert!(result.is_err(), "result: {:?}", result);
    }
}
