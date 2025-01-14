use std::{cell::RefCell, collections::VecDeque};

use super::{corners::Corners, plot::Plot, position::Position};

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

            // Get all corners around current position
            let neighbors_corners = next.corners(self.rows, self.cols);

            // Add positions which needs to be analyzed next
            let mut neighbor_positions = self.get_next_positions(&next, &neighbors_corners);

            // Update perimeter - in positions are only Same positions which are not counted.
            // Because we can have up to 4 neighbors perimeter value is simply 4 - neighbor_positions.len()
            perimeter += 4 - neighbor_positions.len();

            remaining.append(&mut neighbor_positions);
        }

        area * perimeter
    }

    pub fn fence_price_discount(&self) -> usize {
        self.reset();

        let mut price = 0;

        while let Some(next) = self.find_next_position() {
            price += self.area_price_discount(&next);
        }

        price
    }

    fn area_price_discount(&self, pos: &Position) -> usize {
        let mut area = 0;
        // Note: Number of corner == number of sides, so we can compute number of corners instead
        let mut corners = 0;

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

            // Get all corners around current position
            let neighbors_corners = next.corners(self.rows, self.cols);

            // Calculate corners
            let corners_value = self.get_corners_value(&next, &neighbors_corners);

            // Update corners
            corners += corners_value;

            // Add positions which needs to be analyzed next
            let mut neighbor_positions = self.get_next_positions(&next, &neighbors_corners);
            remaining.append(&mut neighbor_positions);
        }

        area * corners
    }

    fn get_next_positions(&self, current: &Position, corners: &Corners) -> VecDeque<Position> {
        // Area contains plots with the same name, so get current name
        let area_name = self.grid[current.row][current.col];

        // We can move only horizontally and vertically, not diagonally
        let neighbor_positions = [&corners.north, &corners.east, &corners.south, &corners.west];

        // Filter out None elements and elements which are not in the same area
        neighbor_positions
            .iter()
            .filter_map(|&pos| match pos {
                Some(pos) => match self.grid[pos.row][pos.col] == area_name {
                    true => Some(pos.clone()),
                    false => None,
                },
                None => None,
            })
            .collect()
    }

    fn get_corners_value(&self, current: &Position, corners: &Corners) -> usize {
        // Area contains plots with the same name, so get current name
        let area_name = self.grid[current.row][current.col];

        let plot_north = self.get_plot_for_side(&corners.north, area_name);
        let plot_north_east = self.get_plot_for_side(&corners.north_east, area_name);
        let plot_east = self.get_plot_for_side(&corners.east, area_name);
        let plot_south_east = self.get_plot_for_side(&corners.south_east, area_name);
        let plot_south = self.get_plot_for_side(&corners.south, area_name);
        let plot_south_west = self.get_plot_for_side(&corners.south_west, area_name);
        let plot_west = self.get_plot_for_side(&corners.west, area_name);
        let plot_north_west = self.get_plot_for_side(&corners.north_west, area_name);

        // We can have up to four corners
        let top_left = (Self::is_plot_different(&plot_west)
            // && Self::is_plot_different(&plot_north_west)
            && Self::is_plot_different(&plot_north))
            || (Self::is_plot_same(&plot_west)
                && Self::is_plot_different(&plot_north_west)
                && Self::is_plot_same(&plot_north));

        let top_right = (Self::is_plot_different(&plot_north)
            // && Self::is_plot_different(&plot_north_east)
            && Self::is_plot_different(&plot_east))
            || (Self::is_plot_same(&plot_north)
                && Self::is_plot_different(&plot_north_east)
                && Self::is_plot_same(&plot_east));

        let bottom_right = (Self::is_plot_different(&plot_east)
            // && Self::is_plot_different(&plot_south_east)
            && Self::is_plot_different(&plot_south))
            || (Self::is_plot_same(&plot_east)
                && Self::is_plot_different(&plot_south_east)
                && Self::is_plot_same(&plot_south));

        let bottom_left = (Self::is_plot_different(&plot_south)
            // && Self::is_plot_different(&plot_south_west)
            && Self::is_plot_different(&plot_west))
            || (Self::is_plot_same(&plot_south)
                && Self::is_plot_different(&plot_south_west)
                && Self::is_plot_same(&plot_west));

        top_left as usize + top_right as usize + bottom_right as usize + bottom_left as usize
    }

    fn is_plot_different(plot: &Plot) -> bool {
        match plot {
            Plot::Border | Plot::Different => true,
            Plot::Same(_) => false,
        }
    }

    fn is_plot_same(plot: &Plot) -> bool {
        !Self::is_plot_different(plot)
    }

    fn get_plot_for_side(&self, side: &Option<Position>, area_name: char) -> Plot {
        match side {
            Some(pos) => match self.grid[pos.row][pos.col] == area_name {
                true => Plot::Same(pos.clone()),
                false => Plot::Different,
            },
            None => Plot::Border,
        }
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

    fn create_garden_e_shaped() -> Garden {
        Garden::new(vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'X', 'X', 'X', 'X'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'X', 'X', 'X', 'X'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ])
    }

    fn create_garden_abab() -> Garden {
        Garden::new(vec![
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'B', 'B', 'A'],
            vec!['A', 'A', 'A', 'B', 'B', 'A'],
            vec!['A', 'B', 'B', 'A', 'A', 'A'],
            vec!['A', 'B', 'B', 'A', 'A', 'A'],
            vec!['A', 'A', 'A', 'A', 'A', 'A'],
        ])
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

    #[test]
    fn test_area_price_discount() {
        let input = [
            // Region A
            ((Position::new(0, 0)), 16),
            ((Position::new(0, 1)), 16),
            ((Position::new(0, 2)), 16),
            ((Position::new(0, 3)), 16),
            // Region B
            ((Position::new(1, 0)), 16),
            ((Position::new(1, 1)), 16),
            ((Position::new(2, 0)), 16),
            ((Position::new(2, 1)), 16),
            // Region C
            ((Position::new(1, 2)), 32),
            ((Position::new(2, 2)), 32),
            ((Position::new(2, 3)), 32),
            ((Position::new(3, 3)), 32),
            // Region D
            ((Position::new(1, 3)), 4),
            // Region E
            ((Position::new(3, 0)), 12),
            ((Position::new(3, 1)), 12),
            ((Position::new(3, 2)), 12),
        ];

        for (pos, price) in input {
            let garden = create_garden_simple();

            assert_eq!(
                garden.area_price_discount(&pos),
                price,
                "Invalid price for position '{:?}'",
                pos
            );
        }
    }

    #[test]
    fn test_fence_price_discount_simple() {
        let garden = create_garden_simple();
        assert_eq!(garden.fence_price_discount(), 80);
    }

    #[test]
    fn test_fence_price_discount_complex() {
        let garden = create_garden_complex();
        assert_eq!(garden.fence_price_discount(), 1206);
    }

    #[test]
    fn test_fence_price_discount_e_shaped() {
        let garden = create_garden_e_shaped();
        assert_eq!(garden.fence_price_discount(), 236);
    }

    #[test]
    fn test_fence_price_discount_abab() {
        let garden = create_garden_abab();
        assert_eq!(garden.fence_price_discount(), 368);
    }
}
