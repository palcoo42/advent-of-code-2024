use super::corners::Corners;

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

    pub fn corners(&self, rows_count: usize, cols_count: usize) -> Corners {
        let mut corners = Corners::default();

        // Fill in mappings for all sides which we have: side, row, col
        let sides = [
            (&mut corners.north, self.row as isize - 1, self.col as isize),
            (
                &mut corners.north_east,
                self.row as isize - 1,
                self.col as isize + 1,
            ),
            (&mut corners.east, self.row as isize, self.col as isize + 1),
            (
                &mut corners.south_east,
                self.row as isize + 1,
                self.col as isize + 1,
            ),
            (&mut corners.south, self.row as isize + 1, self.col as isize),
            (
                &mut corners.south_west,
                self.row as isize + 1,
                self.col as isize - 1,
            ),
            (&mut corners.west, self.row as isize, self.col as isize - 1),
            (
                &mut corners.north_west,
                self.row as isize - 1,
                self.col as isize - 1,
            ),
        ];

        // Process all sides
        for (side, row, col) in sides {
            Self::assign_position_if_valid(row, col, rows_count, cols_count, side);
        }

        corners
    }

    fn assign_position_if_valid(
        row: isize,
        col: isize,
        rows_count: usize,
        cols_count: usize,
        side: &mut Option<Position>,
    ) {
        // Depending on the position validity assign it to the side od the corner
        *side = if row >= 0 && row < rows_count as isize && col >= 0 && col < cols_count as isize {
            Some(Position::new(row as usize, col as usize))
        } else {
            None
        };
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

    #[test]
    fn test_corners() {
        const ROWS: usize = 3;
        const COLS: usize = 5;

        assert_eq!(
            Position::new(0, 0).corners(ROWS, COLS),
            Corners {
                north: None,
                north_east: None,
                east: Some(Position::new(0, 1)),
                south_east: Some(Position::new(1, 1)),
                south: Some(Position::new(1, 0)),
                south_west: None,
                west: None,
                north_west: None,
            }
        );

        assert_eq!(
            Position::new(0, COLS - 1).corners(ROWS, COLS),
            Corners {
                north: None,
                north_east: None,
                east: None,
                south_east: None,
                south: Some(Position::new(1, COLS - 1)),
                south_west: Some(Position::new(1, COLS - 2)),
                west: Some(Position::new(0, COLS - 2)),
                north_west: None,
            }
        );

        assert_eq!(
            Position::new(1, 1).corners(ROWS, COLS),
            Corners {
                north: Some(Position::new(0, 1)),
                north_east: Some(Position::new(0, 2)),
                east: Some(Position::new(1, 2)),
                south_east: Some(Position::new(2, 2)),
                south: Some(Position::new(2, 1)),
                south_west: Some(Position::new(2, 0)),
                west: Some(Position::new(1, 0)),
                north_west: Some(Position::new(0, 0)),
            }
        );

        assert_eq!(
            Position::new(ROWS - 1, 0).corners(ROWS, COLS),
            Corners {
                north: Some(Position::new(ROWS - 2, 0)),
                north_east: Some(Position::new(ROWS - 2, 1)),
                east: Some(Position::new(ROWS - 1, 1)),
                south_east: None,
                south: None,
                south_west: None,
                west: None,
                north_west: None,
            }
        );

        assert_eq!(
            Position::new(ROWS - 1, COLS - 1).corners(ROWS, COLS),
            Corners {
                north: Some(Position::new(ROWS - 2, COLS - 1)),
                north_east: None,
                east: None,
                south_east: None,
                south: None,
                south_west: None,
                west: Some(Position::new(ROWS - 1, COLS - 2)),
                north_west: Some(Position::new(ROWS - 2, COLS - 2)),
            }
        );
    }
}
