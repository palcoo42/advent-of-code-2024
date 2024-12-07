use super::{direction::Direction, position::Position};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    pub fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn get_direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }
}
