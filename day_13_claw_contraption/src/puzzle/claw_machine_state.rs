use super::{
    button::Button,
    claw_machine::{BUTTON_A_TOKENS, BUTTON_B_TOKENS, MAX_ATTEMPTS},
    prize::Prize,
};

#[derive(Debug, Clone, Default)]
pub struct ClawMachineState {
    pub a_count: usize,
    pub b_count: usize,
}

impl ClawMachineState {
    pub fn calc_tokens(&self) -> usize {
        self.a_count * BUTTON_A_TOKENS + self.b_count * BUTTON_B_TOKENS
    }

    pub fn too_many_attempts(&self) -> bool {
        self.a_count > MAX_ATTEMPTS || self.b_count > MAX_ATTEMPTS
    }

    pub fn is_solution(&self, a: &Button, b: &Button, prize: &Prize) -> bool {
        prize.x == self.a_count * a.x + self.b_count * b.x
            && prize.y == self.a_count * a.y + self.b_count * b.y
    }

    pub fn create_states(&self) -> Vec<ClawMachineState> {
        vec![
            ClawMachineState {
                a_count: self.a_count + 1,
                b_count: self.b_count,
            },
            ClawMachineState {
                a_count: self.a_count,
                b_count: self.b_count + 1,
            },
        ]
    }
}
