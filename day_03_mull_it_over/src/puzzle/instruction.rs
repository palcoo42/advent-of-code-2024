#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Multiply(usize, usize),
    Do,
    DoNot,
}
