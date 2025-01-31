use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::{
    adv::Adv, bdv::Bdv, bst::Bst, bxc::Bxc, bxl::Bxl, cdv::Cdv, instruction::Instruction, jnz::Jnz,
    out::Out,
};

pub struct InstructionDecoder {}

impl InstructionDecoder {
    pub fn decode(opcode: &str) -> Result<Box<dyn Instruction>, PuzzleError> {
        match opcode {
            "0" => Ok(Box::new(Adv::new()) as Box<dyn Instruction>),
            "1" => Ok(Box::new(Bxl::new()) as Box<dyn Instruction>),
            "2" => Ok(Box::new(Bst::new()) as Box<dyn Instruction>),
            "3" => Ok(Box::new(Jnz::new()) as Box<dyn Instruction>),
            "4" => Ok(Box::new(Bxc::new()) as Box<dyn Instruction>),
            "5" => Ok(Box::new(Out::new()) as Box<dyn Instruction>),
            "6" => Ok(Box::new(Bdv::new()) as Box<dyn Instruction>),
            "7" => Ok(Box::new(Cdv::new()) as Box<dyn Instruction>),
            _ => Err(PuzzleError::InvalidContentError(format!(
                "Unsupported opcode: '{}'",
                opcode
            ))),
        }
    }
}
