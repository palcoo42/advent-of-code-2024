use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::instruction::Instruction;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Instruction>, PuzzleError> {
        Ok(lines.iter().map(|&line| Instruction::new(line)).collect())
    }

    pub fn decode_mul(memory: &str) -> Result<Vec<(usize, usize)>, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to create regex 'mul'")
        });

        let mut multiplies = Vec::new();

        for capture in RE.captures_iter(memory) {
            let first = capture[1].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse 1st number '{}' to usize with an error '{}'",
                    &capture[1], err
                ))
            })?;

            let second = capture[2].parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to parse 2nd number '{}' to usize with an error '{}'",
                    &capture[2], err
                ))
            })?;

            multiplies.push((first, second));
        }

        match multiplies.is_empty() {
            false => Ok(multiplies),
            true => Err(PuzzleError::InvalidContentError(String::from(
                "Failed to decode instruction to mul",
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_mul() {
        let result = Parser::decode_mul(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }
}
