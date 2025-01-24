use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    // Number of points from left
    pub x: isize,
    // Number of points from top
    pub y: isize,
}

impl Point {
    pub fn neighbor(&self, direction: Direction) -> Point {
        match direction {
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
}
