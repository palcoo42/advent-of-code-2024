use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{computer::Computer, registers::Registers};

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<(Computer, String), PuzzleError> {
        // Check file size
        if lines.len() < 5 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Input file should contain at least 5 lines, but {} found",
                lines.len()
            )));
        }

        let register_a = Self::parse_register(lines[0], "Register A:")?;
        let register_b = Self::parse_register(lines[1], "Register B:")?;
        let register_c = Self::parse_register(lines[2], "Register C:")?;
        let program = Self::parse_program(lines[4])?;

        let computer = Computer::new(Registers {
            a: register_a,
            b: register_b,
            c: register_c,
        });
        Ok((computer, program))
    }

    fn parse_register(line: &str, prefix: &str) -> Result<usize, PuzzleError> {
        match line.find(prefix) {
            Some(index) => {
                let value = &line[index + prefix.len()..].trim();
                let number = value.parse::<usize>().map_err(|err| {
                    PuzzleError::InvalidContentError(format!(
                        "Failed to parse '{}' to usize [{}]",
                        value, err
                    ))
                })?;

                Ok(number)
            }
            None => Err(PuzzleError::InvalidContentError(format!(
                "Failed to find '{}' in '{}'",
                prefix, line
            ))),
        }
    }

    fn parse_program(line: &str) -> Result<String, PuzzleError> {
        const PROGRAM: &str = "Program:";

        match line.find(PROGRAM) {
            Some(index) => Ok(line[index + PROGRAM.len()..].trim().to_string()),
            None => Err(PuzzleError::InvalidContentError(format!(
                "Failed to find '{}' in '{}'",
                PROGRAM, line
            ))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_lines() {
        let lines = [
            "Register A: 729",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ];

        let result = Parser::parse_lines(&lines);
        assert!(result.is_ok(), "result: {:?}", result);

        let (computer, program) = result.unwrap();
        assert_eq!(computer.get_registers(), &Registers { a: 729, b: 0, c: 0 });
        assert_eq!(program, "0,1,5,4,3,0");
    }
}
