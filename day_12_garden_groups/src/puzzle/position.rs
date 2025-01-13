#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn neighbors(&self, rows_count: usize, cols_count: usize) -> Vec<Position> {
        let mut neighbors = Vec::new();

        // Left
        if self.col > 0 {
            neighbors.push(Position::new(self.row, self.col - 1));
        }

        // Right
        if self.col < cols_count - 1 {
            neighbors.push(Position::new(self.row, self.col + 1));
        }

        // Up
        if self.row > 0 {
            neighbors.push(Position::new(self.row - 1, self.col));
        }

        // Down
        if self.row < rows_count - 1 {
            neighbors.push(Position::new(self.row + 1, self.col));
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors() {
        const ROWS: usize = 3;
        const COLS: usize = 5;

        assert_eq!(
            Position::new(0, 0).neighbors(ROWS, COLS),
            vec![Position::new(0, 1), Position::new(1, 0)]
        );

        assert_eq!(
            Position::new(0, COLS - 1).neighbors(ROWS, COLS),
            vec![Position::new(0, COLS - 2), Position::new(1, COLS - 1)]
        );

        assert_eq!(
            Position::new(1, 1).neighbors(ROWS, COLS),
            vec![
                Position::new(1, 0),
                Position::new(1, 2),
                Position::new(0, 1),
                Position::new(2, 1)
            ]
        );

        assert_eq!(
            Position::new(ROWS - 1, 0).neighbors(ROWS, COLS),
            vec![Position::new(ROWS - 1, 1), Position::new(ROWS - 2, 0)]
        );

        assert_eq!(
            Position::new(ROWS - 1, COLS - 1).neighbors(ROWS, COLS),
            vec![
                Position::new(ROWS - 1, COLS - 2),
                Position::new(ROWS - 2, COLS - 1)
            ]
        );
    }
}
