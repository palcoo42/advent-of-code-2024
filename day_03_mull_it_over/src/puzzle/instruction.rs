use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::parser::Parser;

pub struct Instruction {
    memory: String,
}

impl Instruction {
    pub fn new(memory: &str) -> Self {
        Self {
            memory: String::from(memory),
        }
    }

    pub fn multiply(&self) -> Result<usize, PuzzleError> {
        let multiplies = Parser::decode_mul(&self.memory)?;
        Ok(multiplies.iter().map(|(a, b)| a * b).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let instruction = Instruction::new(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );

        let result = instruction.multiply();

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(instruction.multiply().unwrap(), 161);
    }
}
