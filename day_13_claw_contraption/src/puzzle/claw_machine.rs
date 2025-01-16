use std::collections::{HashSet, VecDeque};

use super::{button::Button, claw_machine_state::ClawMachineState, prize::Prize};

pub const BUTTON_A_TOKENS: usize = 3;
pub const BUTTON_B_TOKENS: usize = 1;
pub const MAX_ATTEMPTS: usize = 100;

#[derive(Debug, Clone)]
pub struct ClawMachine {
    a: Button,
    b: Button,
    prize: Prize,
}

impl ClawMachine {
    pub fn new(a: Button, b: Button, prize: Prize) -> Self {
        Self { a, b, prize }
    }

    pub fn append_prizes(&mut self, value: usize) {
        self.prize.x += value;
        self.prize.y += value;
    }

    pub fn find_fewest_tokens(&self) -> Option<usize> {
        let mut fewest_tokens = usize::MAX;

        let mut remaining: VecDeque<_> = vec![ClawMachineState::default()].into_iter().collect();
        // Todo: Change to ClawMachineState
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

    pub fn calculate_fewest_tokens(&self) -> Option<usize> {
        // Little bit of math
        // Note: We have to use isize as equations could ne negative
        let divider = self.a.x as isize * self.b.y as isize - self.a.y as isize * self.b.x as isize;

        let a_count = (self.b.y as isize * self.prize.x as isize
            - self.b.x as isize * self.prize.y as isize)
            / divider;

        let b_count = (self.a.x as isize * self.prize.y as isize
            - self.a.y as isize * self.prize.x as isize)
            / divider;

        // Because we are using isize results could be truncated. Therefore we need to check
        // equations if found a_count and b_count are really correct.
        if a_count > 0
            && b_count > 0
            && a_count as usize * self.a.x + b_count as usize * self.b.x == self.prize.x
            && a_count as usize * self.a.y + b_count as usize * self.b.y == self.prize.y
        {
            let state = ClawMachineState {
                a_count: a_count as usize,
                b_count: b_count as usize,
            };

            Some(state.calc_tokens())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_input() -> Vec<(ClawMachine, Option<usize>)> {
        vec![
            (
                ClawMachine::new(
                    Button { x: 94, y: 34 },
                    Button { x: 22, y: 67 },
                    Prize { x: 8400, y: 5400 },
                ),
                Some(280),
            ),
            (
                ClawMachine::new(
                    Button { x: 26, y: 66 },
                    Button { x: 67, y: 21 },
                    Prize { x: 12748, y: 12176 },
                ),
                None,
            ),
            (
                ClawMachine::new(
                    Button { x: 17, y: 86 },
                    Button { x: 84, y: 37 },
                    Prize { x: 7870, y: 6450 },
                ),
                Some(200),
            ),
            (
                ClawMachine::new(
                    Button { x: 69, y: 23 },
                    Button { x: 27, y: 71 },
                    Prize { x: 18641, y: 10279 },
                ),
                None,
            ),
        ]
    }

    fn create_input_appended() -> Vec<(ClawMachine, Option<usize>)> {
        vec![
            (
                ClawMachine::new(
                    Button { x: 94, y: 34 },
                    Button { x: 22, y: 67 },
                    Prize {
                        x: 10000000008400,
                        y: 10000000005400,
                    },
                ),
                None,
            ),
            (
                ClawMachine::new(
                    Button { x: 26, y: 66 },
                    Button { x: 67, y: 21 },
                    Prize {
                        x: 10000000012748,
                        y: 10000000012176,
                    },
                ),
                Some(118679050709 * 3 + 103199174542),
            ),
            (
                ClawMachine::new(
                    Button { x: 17, y: 86 },
                    Button { x: 84, y: 37 },
                    Prize {
                        x: 10000000007870,
                        y: 10000000006450,
                    },
                ),
                None,
            ),
            (
                ClawMachine::new(
                    Button { x: 69, y: 23 },
                    Button { x: 27, y: 71 },
                    Prize {
                        x: 10000000018641,
                        y: 10000000010279,
                    },
                ),
                Some(102851800151 * 3 + 107526881786),
            ),
        ]
    }

    #[test]
    fn test_find_fewest_tokens() {
        let input = create_input();

        for (machine, expected_tokens) in input {
            let tokens = machine.find_fewest_tokens();
            assert_eq!(tokens, expected_tokens, "machine: {:?}", machine);
        }
    }

    #[test]
    fn test_calculate_fewest_tokens_appended() {
        let input = create_input_appended();

        for (machine, expected_tokens) in input {
            let tokens = machine.calculate_fewest_tokens();
            assert_eq!(tokens, expected_tokens, "machine: {:?}", machine);
        }
    }
}
