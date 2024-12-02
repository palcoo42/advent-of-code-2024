use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::report::Report;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Report>, PuzzleError> {
        lines
            .iter()
            .map(|&line| Self::decode_report(line))
            .collect()
    }

    fn decode_report(line: &str) -> Result<Report, PuzzleError> {
        let splits = line.split_ascii_whitespace();

        let mut numbers = Vec::new();

        for split in splits {
            let number = split.parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with an error '{}', line: {}",
                    split, err, line
                ))
            })?;

            numbers.push(number);
        }

        Ok(Report::new(numbers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_report() {
        let result = Parser::decode_report("1 2 3 4 5");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), Report::new(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_decode_report_invalid() {
        let result = Parser::decode_report("1 2 3x 4 5");

        assert!(result.is_err(), "Result: {:?}", result);
    }
}
