use std::{cell::RefCell, collections::VecDeque};

use super::{plot::Plot, position::Position};

#[derive(Default)]
pub struct Garden {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>,
    visited: RefCell<Vec<Vec<bool>>>,
}

impl Garden {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        assert!(!grid.is_empty());

        let rows = grid.len();
        let cols = grid[0].len();

        // Fill in visited to false
        let mut visited = Vec::with_capacity(rows);
        for _ in 0..rows {
            visited.push(vec![false; cols]);
        }

        Self {
            rows,
            cols,
            grid,
            visited: RefCell::new(visited),
        }
    }

    fn reset(&self) {
        let mut visited = self.visited.borrow_mut();

        for row in 0..self.rows {
            for col in 0..self.cols {
                visited[row][col] = false;
            }
        }
    }

    pub fn fence_price(&self) -> usize {
        self.reset();

        let mut price = 0;

        while let Some(next) = self.find_next_position() {
            price += self.area_price(&next);
        }

        price
    }

    fn find_next_position(&self) -> Option<Position> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if !self.visited.borrow()[row][col] {
                    return Some(Position::new(row, col));
                }
            }
        }

        None
    }

    #[inline]
    fn is_visited(&self, pos: &Position) -> bool {
        self.visited.borrow()[pos.row][pos.col]
    }

    #[inline]
    fn set_visited(&self, pos: &Position) {
        self.visited.borrow_mut()[pos.row][pos.col] = true;
    }

    fn area_price(&self, pos: &Position) -> usize {
        let mut area = 0;
        let mut perimeter = 0;

        // Collection of remaining positions to check
        let mut remaining: VecDeque<_> = vec![pos.clone()].into();

        while let Some(next) = remaining.pop_front() {
            // Note: We could insert the same position in the 'remaining' queue multiple times.
            // To solve this issue check also here if position was already analyzed;
            if self.is_visited(&next) {
                continue;
            }

            // We have new area
            area += 1;

            // Mark current position as already analyzed
            self.set_visited(&next);

            // Get surrounding plots
            let plots = self.get_plots(&next);

            // Update perimeter
            for plot in plots {
                let perimeter_value = match plot {
                    Plot::Border | Plot::Different => 1,
                    Plot::Same(neighbor_pos) => {
                        // New position to continue with calculation
                        if !self.is_visited(&neighbor_pos) {
                            remaining.push_back(neighbor_pos);
                        }
                        0
                    }
                };

                perimeter += perimeter_value;
            }
        }

        area * perimeter
    }

    fn get_plots(&self, current: &Position) -> Vec<Plot> {
        // Area contains plots with the same name, so get current name
        let area_name = self.grid[current.row][current.col];

        // Note: This method returns only valid positions, i.e. position which are within grid
        // marked by rows and cols. However we need four plots which are surrounding current
        // position. These missing positions are marked as Border.
        let neighbors = current.neighbors(self.rows, self.cols);
        let border_plots = 4 - neighbors.len();

        let mut plots = neighbors
            .into_iter()
            .map(|pos| match self.grid[pos.row][pos.col] == area_name {
                true => Plot::Same(pos),
                false => Plot::Different,
            })
            .collect::<Vec<_>>();

        // Append Border plots (if present)
        plots.append(&mut vec![Plot::Border; border_plots]);

        plots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_garden_simple() -> Garden {
        Garden::new(vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ])
    }

    fn create_garden_medium() -> Garden {
        Garden::new(vec![
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
        ])
    }

    fn create_garden_complex() -> Garden {
        Garden::new(vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ])
    }

    #[test]
    fn test_get_plots() {
        let garden = create_garden_simple();

        assert_eq!(
            garden.get_plots(&Position::new(0, 0)),
            vec![
                Plot::Same(Position::new(0, 1)),
                Plot::Different,
                Plot::Border,
                Plot::Border
            ]
        );

        assert_eq!(
            garden.get_plots(&Position::new(0, 1)),
            vec![
                Plot::Same(Position::new(0, 0)),
                Plot::Same(Position::new(0, 2)),
                Plot::Different,
                Plot::Border
            ]
        );

        assert_eq!(
            garden.get_plots(&Position::new(0, 3)),
            vec![
                Plot::Same(Position::new(0, 2)),
                Plot::Different,
                Plot::Border,
                Plot::Border
            ]
        );

        assert_eq!(
            garden.get_plots(&Position::new(2, 2)),
            vec![
                Plot::Different,
                Plot::Same(Position::new(2, 3)),
                Plot::Same(Position::new(1, 2)),
                Plot::Different,
            ]
        );

        assert_eq!(
            garden.get_plots(&Position::new(3, 0)),
            vec![
                Plot::Same(Position::new(3, 1)),
                Plot::Different,
                Plot::Border,
                Plot::Border
            ]
        );

        assert_eq!(
            garden.get_plots(&Position::new(3, 3)),
            vec![
                Plot::Different,
                Plot::Same(Position::new(2, 3)),
                Plot::Border,
                Plot::Border
            ]
        );
    }

    #[test]
    fn test_area_price() {
        let input = [
            // Region A
            ((Position::new(0, 0)), 40),
            ((Position::new(0, 1)), 40),
            ((Position::new(0, 2)), 40),
            ((Position::new(0, 3)), 40),
            // Region B
            ((Position::new(1, 0)), 32),
            ((Position::new(1, 1)), 32),
            ((Position::new(2, 0)), 32),
            ((Position::new(2, 1)), 32),
            // Region C
            ((Position::new(1, 2)), 40),
            ((Position::new(2, 2)), 40),
            ((Position::new(2, 3)), 40),
            ((Position::new(3, 3)), 40),
            // Region D
            ((Position::new(1, 3)), 4),
            // Region E
            ((Position::new(3, 0)), 24),
            ((Position::new(3, 1)), 24),
            ((Position::new(3, 2)), 24),
        ];

        for (pos, price) in input {
            let garden = create_garden_simple();

            assert_eq!(
                garden.area_price(&pos),
                price,
                "Invalid price for position '{:?}'",
                pos
            );
        }
    }

    #[test]
    fn test_fence_price_simple() {
        let garden = create_garden_simple();
        assert_eq!(garden.fence_price(), 140);
    }

    #[test]
    fn test_fence_price_medium() {
        let garden = create_garden_medium();
        assert_eq!(garden.fence_price(), 772);
    }

    #[test]
    fn test_fence_price_complex() {
        let garden = create_garden_complex();
        assert_eq!(garden.fence_price(), 1930);
    }
}
