use super::claw_machine::ClawMachine;
use rayon::prelude::*;

#[derive(Default)]
pub struct Game {
    machines: Vec<ClawMachine>,
}

impl Game {
    pub fn new(machines: Vec<ClawMachine>) -> Self {
        Self { machines }
    }

    pub fn push(&mut self, machine: ClawMachine) {
        self.machines.push(machine);
    }

    pub fn count_fewest_tokens_to_win_all_prizes(&self) -> Option<usize> {
        // Find out fewest tokens for every machine. If machine does not have a solution
        // None is returned.
        let tokens = self
            .machines
            .par_iter()
            .filter_map(|machine| machine.find_fewest_tokens())
            .collect::<Vec<_>>();

        // There may be no solution -> in this case return None
        match tokens.is_empty() {
            true => None,
            false => Some(tokens.iter().sum::<usize>()),
        }
    }

    pub fn count_fewest_tokens_to_win_all_prizes_fast(&self) -> Option<usize> {
        // Find out fewest tokens for every machine. If machine does not have a solution
        // None is returned.
        let tokens = self
            .machines
            .par_iter()
            .filter_map(|machine| machine.calculate_fewest_tokens())
            .collect::<Vec<_>>();

        // There may be no solution -> in this case return None
        match tokens.is_empty() {
            true => None,
            false => Some(tokens.iter().sum::<usize>()),
        }
    }

    pub fn calculate_fewest_tokens_to_win_all_prizes(&self) -> Option<usize> {
        const APPEND_VALUE: usize = 10_000_000_000_000;

        // Find out fewest tokens for every machine. If machine does not have a solution
        // None is returned.
        let tokens = self
            .machines
            .par_iter()
            .filter_map(|machine| {
                // Note: We cannot update machines in the Game instance as it would affect
                // unit tests as all parts are run in parallel. Instead we will clone() machine
                // and update its content.
                let mut updated_machine = machine.clone();
                updated_machine.append_prizes(APPEND_VALUE);

                updated_machine.calculate_fewest_tokens()
            })
            .collect::<Vec<_>>();

        // There may be no solution -> in this case return None
        match tokens.is_empty() {
            true => None,
            false => Some(tokens.iter().sum::<usize>()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::{button::Button, prize::Prize};

    use super::*;

    fn create_game() -> Game {
        let mut game = Game::new(vec![]);

        game.push(ClawMachine::new(
            Button { x: 94, y: 34 },
            Button { x: 22, y: 67 },
            Prize { x: 8400, y: 5400 },
        ));
        game.push(ClawMachine::new(
            Button { x: 26, y: 66 },
            Button { x: 67, y: 21 },
            Prize { x: 12748, y: 12176 },
        ));
        game.push(ClawMachine::new(
            Button { x: 17, y: 86 },
            Button { x: 84, y: 37 },
            Prize { x: 7870, y: 6450 },
        ));
        game.push(ClawMachine::new(
            Button { x: 69, y: 23 },
            Button { x: 27, y: 71 },
            Prize { x: 18641, y: 10279 },
        ));

        game
    }

    #[test]
    fn test_count_fewest_tokens_to_win_all_prizes() {
        let game = create_game();
        assert_eq!(game.count_fewest_tokens_to_win_all_prizes(), Some(480));
    }

    #[test]
    fn test_calculate_fewest_tokens_to_win_all_prizes() {
        let game = create_game();
        assert_eq!(
            game.calculate_fewest_tokens_to_win_all_prizes(),
            Some(875318608908)
        );
    }
}
