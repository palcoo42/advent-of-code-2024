use std::collections::VecDeque;

use super::operation::Operation;

#[derive(Debug, Clone, PartialEq)]
pub struct EquationState {
    pub value: usize,
    pub operations: Vec<Operation>,
    pub numbers: VecDeque<usize>,
}
