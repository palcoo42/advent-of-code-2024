use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::equation::Equation;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Equation>, PuzzleError> {
        lines.iter().map(|line| Self::decode_line(line)).collect()
    }

    fn decode_line(line: &str) -> Result<Equation, PuzzleError> {
        let split = line.split(":").collect::<Vec<_>>();

        if split.len() != 2 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Invalid number of elements [{}] after split with :",
                split.len()
            )));
        }

        // Extract calibration
        let calibration = split[0].parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert 'calibration' '{}' to usize with an error '{}'",
                split[0], err
            ))
        })?;

        // Extract numbers
        let raw_numbers = split[1].split_ascii_whitespace().collect::<Vec<_>>();

        if raw_numbers.is_empty() {
            return Err(PuzzleError::InvalidContentError(format!(
                "Invalid number of elements [{}] after split with <whitespace>",
                raw_numbers.len()
            )));
        }

        let mut numbers = Vec::new();

        for raw_number in raw_numbers {
            // Convert to usize
            let number = raw_number.parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert 'number' '{}' to usize with an error '{}'",
                    raw_number, err
                ))
            })?;

            // Add to numbers
            numbers.push(number);
        }

        Ok(Equation::new(calibration, numbers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_line() {
        let result = Parser::decode_line("161011: 16 10 13");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), Equation::new(161011, vec![16, 10, 13]));
    }
}
