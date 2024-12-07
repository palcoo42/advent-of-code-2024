use std::collections::VecDeque;

use super::{equation_state::EquationState, operation::Operation};

#[derive(Debug, PartialEq)]
pub struct Equation {
    calibration: usize,
    numbers: Vec<usize>,
}

impl Equation {
    pub fn new(calibration: usize, numbers: Vec<usize>) -> Self {
        Self {
            calibration,
            numbers,
        }
    }

    pub fn get_calibration(&self) -> usize {
        self.calibration
    }

    pub fn solve(&self) -> Vec<Vec<Operation>> {
        let mut solutions = Vec::new();

        // Make a copy of numbers which we can push to the next states
        let mut numbers = self.numbers.iter().copied().collect::<VecDeque<_>>();

        let mut next_states = VecDeque::new();
        next_states.push_back(EquationState {
            value: numbers.pop_front().expect("Failed to pop front element"),
            operations: vec![],
            numbers,
        });

        while let Some(equation_state) = next_states.pop_front() {
            // Check for the end condition
            if equation_state.numbers.is_empty() {
                // Check for a solution
                if equation_state.value == self.calibration {
                    solutions.push(equation_state.operations.clone());
                }
                continue;
            }

            // Spawn add operation
            let mut add = equation_state.clone();
            add.value += add
                .numbers
                .pop_front()
                .expect("Failed to pop front element");
            add.operations.push(Operation::Add);

            next_states.push_back(add);

            // Spawn multiply operation
            let mut multiply = equation_state.clone();
            multiply.value *= multiply
                .numbers
                .pop_front()
                .expect("Failed to pop front element");
            multiply.operations.push(Operation::Multiply);

            next_states.push_back(multiply);
        }

        solutions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            Equation::new(190, vec![10, 19]).solve(),
            vec![vec![Operation::Multiply]]
        );

        assert_eq!(
            Equation::new(3267, vec![81, 40, 27]).solve(),
            vec![
                vec![Operation::Add, Operation::Multiply],
                vec![Operation::Multiply, Operation::Add]
            ]
        );

        assert_eq!(
            Equation::new(292, vec![11, 6, 16, 20]).solve(),
            vec![vec![Operation::Add, Operation::Multiply, Operation::Add]]
        );
    }
}
