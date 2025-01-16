use std::collections::{HashSet, VecDeque};

use super::{button::Button, claw_machine_state::ClawMachineState, prize::Prize};

pub const BUTTON_A_TOKENS: usize = 3;
pub const BUTTON_B_TOKENS: usize = 1;
pub const MAX_ATTEMPTS: usize = 100;

#[derive(Debug)]
pub struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl ClawMachine {
    pub fn new(a: Button, b: Button, prize: Prize) -> Self {
        Self { a, b, prize }
    }

    pub fn find_fewest_tokens(&self) -> Option<usize> {
        let mut fewest_tokens = usize::MAX;

        let mut remaining: VecDeque<_> = vec![ClawMachineState::default()].into_iter().collect();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        // DFS - Repeat until we have states to check
        while let Some(current_state) = remaining.pop_front() {
            if visited.contains(&(current_state.a_count, current_state.b_count)) {
                continue;
            }

            visited.insert((current_state.a_count, current_state.b_count));

            // Check for too many attempts
            if current_state.too_many_attempts() {
                continue;
            }

            // If we already have more token than minimum we can stop
            if current_state.calc_tokens() >= fewest_tokens {
                continue;
            }

            // If we have a solution update fewest tokens of applicable
            if current_state.is_solution(&self.a, &self.b, &self.prize) {
                let tokens = current_state.calc_tokens();
                if tokens < fewest_tokens {
                    fewest_tokens = tokens;
                }
                continue;
            }

            // Otherwise continue with investigation -> append next states
            let states = current_state.create_states();
            for state in states {
                remaining.push_back(state);
            }
        }

        // Return result
        match fewest_tokens {
            usize::MAX => None,
            tokens => Some(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_fewest_tokens() {
        let machine = ClawMachine::new(
            Button { x: 94, y: 34 },
            Button { x: 22, y: 67 },
            Prize { x: 8400, y: 5400 },
        );
        assert_eq!(machine.find_fewest_tokens(), Some(280));

        let machine = ClawMachine::new(
            Button { x: 26, y: 66 },
            Button { x: 67, y: 21 },
            Prize { x: 12748, y: 12176 },
        );
        assert_eq!(machine.find_fewest_tokens(), None);

        let machine = ClawMachine::new(
            Button { x: 17, y: 86 },
            Button { x: 84, y: 37 },
            Prize { x: 7870, y: 6450 },
        );
        assert_eq!(machine.find_fewest_tokens(), Some(200));

        let machine = ClawMachine::new(
            Button { x: 69, y: 23 },
            Button { x: 27, y: 71 },
            Prize { x: 18641, y: 10279 },
        );
        assert_eq!(machine.find_fewest_tokens(), None);
    }
}
