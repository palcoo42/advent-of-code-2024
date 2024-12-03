use std::sync::LazyLock;

use advent_of_code::puzzles::puzzle_error::PuzzleError;
use regex::Regex;

use super::instruction::Instruction;

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<Vec<Instruction>, PuzzleError> {
        let mut instructions = Vec::new();

        for line in lines {
            let mut line_instructions = Self::decode_instructions(line)?;
            instructions.append(&mut line_instructions);
        }

        Ok(instructions)
    }

    pub fn decode_instructions(line: &str) -> Result<Vec<Instruction>, PuzzleError> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"(?P<mul>mul\((?P<first>\d+),(?P<second>\d+)\))|(?P<do_not>don't\(\))|(?P<do>do\(\))")
                .expect("Failed to create regex 'mul'")
        });

        let instructions = RE
            .captures_iter(line)
            .filter_map(|capture| {
                if capture.name("mul").is_some() {
                    let first = capture["first"]
                        .parse::<usize>()
                        .map_err(|err| {
                            PuzzleError::InvalidContentError(format!(
                                "Failed to parse 1st number '{}' to usize with an error '{}'",
                                &capture[1], err
                            ))
                        })
                        .ok()?;

                    let second = capture["second"]
                        .parse::<usize>()
                        .map_err(|err| {
                            PuzzleError::InvalidContentError(format!(
                                "Failed to parse 2nd number '{}' to usize with an error '{}'",
                                &capture[2], err
                            ))
                        })
                        .ok()?;

                    Some(Instruction::Multiply(first, second))
                } else if capture.name("do_not").is_some() {
                    Some(Instruction::DoNot)
                } else if capture.name("do").is_some() {
                    Some(Instruction::Do)
                } else {
                    None
                }
            })
            .collect();

        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_instructions() {
        let result = Parser::decode_instructions(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(
            result.unwrap(),
            vec![
                Instruction::Multiply(2, 4),
                Instruction::DoNot,
                Instruction::Multiply(5, 5),
                Instruction::Multiply(11, 8),
                Instruction::Do,
                Instruction::Multiply(8, 5)
            ]
        );
    }
}
