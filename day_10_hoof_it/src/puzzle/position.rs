use super::grid::Grid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn get_neighbors(&self, grid: &impl Grid) -> Vec<Position> {
        let mut neighbors = Vec::new();

        // left
        if self.col > 0 {
            neighbors.push(Position {
                row: self.row,
                col: self.col - 1,
            });
        }

        // right
        if self.col < grid.cols_len() - 1 {
            neighbors.push(Position {
                row: self.row,
                col: self.col + 1,
            });
        }

        // up
        if self.row > 0 {
            neighbors.push(Position {
                row: self.row - 1,
                col: self.col,
            });
        }

        // down
        if self.row < grid.rows_len() - 1 {
            neighbors.push(Position {
                row: self.row + 1,
                col: self.col,
            });
        }

        neighbors
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestGrid {
        pub rows: usize,
        pub cols: usize,
    }

    impl Grid for TestGrid {
        fn rows_len(&self) -> usize {
            self.rows
        }

        fn cols_len(&self) -> usize {
            self.cols
        }
    }

    #[test]
    fn test_get_neighbors() {
        const GRID: TestGrid = TestGrid { rows: 3, cols: 5 };

        assert_eq!(
            Position { row: 0, col: 0 }.get_neighbors(&GRID),
            vec![Position { row: 0, col: 1 }, Position { row: 1, col: 0 }]
        );

        assert_eq!(
            Position { row: 0, col: 4 }.get_neighbors(&GRID),
            vec![Position { row: 0, col: 3 }, Position { row: 1, col: 4 }]
        );

        assert_eq!(
            Position { row: 1, col: 1 }.get_neighbors(&GRID),
            vec![
                Position { row: 1, col: 0 },
                Position { row: 1, col: 2 },
                Position { row: 0, col: 1 },
                Position { row: 2, col: 1 }
            ]
        );

        assert_eq!(
            Position { row: 2, col: 0 }.get_neighbors(&GRID),
            vec![Position { row: 2, col: 1 }, Position { row: 1, col: 0 }]
        );

        assert_eq!(
            Position { row: 2, col: 4 }.get_neighbors(&GRID),
            vec![Position { row: 2, col: 3 }, Position { row: 1, col: 4 }]
        );
    }
}
