use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use super::{part::Part, position::Position};

pub struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    algorithm: RefCell<Part>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(Vec::new())
    }
}

impl Grid {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = if rows != 0 { grid[0].len() } else { 0 };

        Self {
            grid,
            rows,
            cols,
            algorithm: RefCell::new(Part::Part1),
        }
    }

    pub fn set_algorithm(&self, algorithm: Part) {
        *self.algorithm.borrow_mut() = algorithm;
    }

    pub fn collect_anti_nodes(&self) -> HashSet<Position> {
        let mut anti_nodes = HashSet::new();

        let antennas = self.collect_antennas();

        for (_antenna, positions) in antennas {
            // Skip antennas which do not have pairs
            if positions.len() == 1 {
                continue;
            }

            let combinations = positions.iter().combinations(2);

            for combination in combinations {
                let nodes = match *self.algorithm.borrow() {
                    Part::Part1 => self.compute_anti_nodes(combination[0], combination[1]),
                    Part::Part2 => {
                        self.compute_anti_nodes_recursive(combination[0], combination[1])
                    }
                };

                for node in nodes {
                    anti_nodes.insert(node);
                }
            }
        }

        anti_nodes
    }

    fn compute_anti_nodes(&self, a: &Position, b: &Position) -> Vec<Position> {
        // Return only valid anti node positions
        let x_diff = a.x.abs_diff(b.x);
        let y_diff = a.y.abs_diff(b.y);

        // Not correct alignment of antennas
        if x_diff == 0 || y_diff == 0 {
            return vec![];
        }

        // We have two options
        //
        // ..........      ..........
        // ...#......      ......#...
        // ..........      ..........
        // ....a.....      .....a....
        // ..........      ..........
        // .....a....      ....a.....
        // ..........      ..........
        // ......#...      ...#......
        // ..........      ..........
        // ..........      ..........
        //
        let mut anti_nodes = Vec::new();

        if a.x < b.x && a.y < b.y {
            // 1st picture from the above
            let pos_x = a.x as isize - x_diff as isize;
            let pos_y = a.y as isize - y_diff as isize;

            if self.is_position_valid(pos_x, pos_y) {
                anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
            }

            let pos_x = b.x as isize + x_diff as isize;
            let pos_y = b.y as isize + y_diff as isize;

            if self.is_position_valid(pos_x, pos_y) {
                anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
            }
        } else {
            // 2nd picture from the above
            let pos_x = a.x as isize - x_diff as isize;
            let pos_y = a.y as isize + y_diff as isize;

            if self.is_position_valid(pos_x, pos_y) {
                anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
            }

            let pos_x = b.x as isize + x_diff as isize;
            let pos_y = b.y as isize - y_diff as isize;

            if self.is_position_valid(pos_x, pos_y) {
                anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
            }
        }

        anti_nodes
    }

    fn compute_anti_nodes_recursive(&self, a: &Position, b: &Position) -> Vec<Position> {
        // Return only valid anti node positions
        let x_diff = a.x.abs_diff(b.x);
        let y_diff = a.y.abs_diff(b.y);

        // Not correct alignment of antennas
        if x_diff == 0 || y_diff == 0 {
            return vec![];
        }

        // We have two options
        //
        // ..........      ..........
        // ...#......      ......#...
        // ..........      ..........
        // ....a.....      .....a....
        // ..........      ..........
        // .....a....      ....a.....
        // ..........      ..........
        // ......#...      ...#......
        // ..........      ..........
        // .......#..      ..#.......
        //
        let mut anti_nodes = Vec::new();

        if a.x < b.x && a.y < b.y {
            // 1st picture from the above
            let mut count = 0_usize;

            loop {
                let pos_x = a.x as isize - (x_diff * count) as isize;
                let pos_y = a.y as isize - (y_diff * count) as isize;

                if self.is_position_valid(pos_x, pos_y) {
                    anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
                    count += 1;
                } else {
                    break;
                }
            }

            let mut count = 0_usize;

            loop {
                let pos_x = b.x as isize + (x_diff * count) as isize;
                let pos_y = b.y as isize + (y_diff * count) as isize;

                if self.is_position_valid(pos_x, pos_y) {
                    anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
                    count += 1;
                } else {
                    break;
                }
            }
        } else {
            // 2nd picture from the above
            let mut count = 0_usize;

            loop {
                let pos_x = a.x as isize - (x_diff * count) as isize;
                let pos_y = a.y as isize + (y_diff * count) as isize;

                if self.is_position_valid(pos_x, pos_y) {
                    anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
                    count += 1;
                } else {
                    break;
                }
            }

            let mut count = 0_usize;

            loop {
                let pos_x = b.x as isize + (x_diff * count) as isize;
                let pos_y = b.y as isize - (y_diff * count) as isize;

                if self.is_position_valid(pos_x, pos_y) {
                    anti_nodes.push(Position::new(pos_x as usize, pos_y as usize));
                    count += 1;
                } else {
                    break;
                }
            }
        }

        anti_nodes
    }

    fn is_position_valid(&self, pos_x: isize, pos_y: isize) -> bool {
        pos_x >= 0 && pos_x < self.rows as isize && pos_y >= 0 && pos_y < self.cols as isize
    }

    fn collect_antennas(&self) -> HashMap<char, Vec<Position>> {
        let mut antennas = HashMap::new();

        for i in 0..self.rows {
            for j in 0..self.cols {
                match self.grid[i][j] {
                    '.' => {}
                    c => {
                        let entry: &mut Vec<Position> = antennas.entry(c).or_default();
                        entry.push(Position::new(i, j));
                    }
                }
            }
        }

        antennas
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::parser::Parser;

    use super::*;

    fn create_grid_simple() -> Grid {
        let raw = vec![
            "..........",
            "..........",
            "..........",
            "....a.....",
            "..........",
            ".....a....",
            "..........",
            "..........",
            "..........",
            "..........",
        ];

        Parser::parse_lines(&raw).unwrap_or_else(|err| {
            panic!(
                "Failed to create grid with an error '{}', raw: '{:?}'",
                err, raw
            )
        })
    }

    fn create_grid_simple_reversed() -> Grid {
        let raw = vec![
            "..........",
            "..........",
            "..........",
            ".....a....",
            "..........",
            "....a.....",
            "..........",
            "..........",
            "..........",
            "..........",
        ];

        Parser::parse_lines(&raw).unwrap_or_else(|err| {
            panic!(
                "Failed to create grid with an error '{}', raw: '{:?}'",
                err, raw
            )
        })
    }

    fn create_grid_medium() -> Grid {
        let raw = vec![
            "..........",
            "..........",
            "..........",
            "....a.....",
            "........a.",
            ".....a....",
            "..........",
            "..........",
            "..........",
            "..........",
        ];

        Parser::parse_lines(&raw).unwrap_or_else(|err| {
            panic!(
                "Failed to create grid with an error '{}', raw: '{:?}'",
                err, raw
            )
        })
    }

    fn create_grid_complex() -> Grid {
        let raw = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ];

        Parser::parse_lines(&raw).unwrap_or_else(|err| {
            panic!(
                "Failed to create grid with an error '{}', raw: '{:?}'",
                err, raw
            )
        })
    }

    fn create_grid_t_complex() -> Grid {
        let raw = vec![
            "T.........",
            "...T......",
            ".T........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
            "..........",
        ];

        Parser::parse_lines(&raw).unwrap_or_else(|err| {
            panic!(
                "Failed to create grid with an error '{}', raw: '{:?}'",
                err, raw
            )
        })
    }

    #[test]
    fn test_collect_antennas_simple() {
        let grid = create_grid_simple();

        let positions = grid.collect_antennas();
        assert_eq!(positions.len(), 1);

        let a = positions.get(&'a').unwrap();
        assert_eq!(a.len(), 2);
        assert_eq!(a[0], Position::new(3, 4));
        assert_eq!(a[1], Position::new(5, 5));
    }

    #[test]
    fn test_collect_anti_nodes_simple() {
        let grid = create_grid_simple();

        let positions = grid.collect_anti_nodes();
        assert_eq!(
            positions,
            [Position::new(1, 3), Position::new(7, 6)]
                .into_iter()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_simple_reversed() {
        let grid = create_grid_simple_reversed();

        let positions = grid.collect_anti_nodes();
        assert_eq!(
            positions,
            [Position::new(1, 6), Position::new(7, 3)]
                .into_iter()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_medium() {
        let grid = create_grid_medium();

        let positions = grid.collect_anti_nodes();
        assert_eq!(
            positions,
            [
                Position::new(2, 0),
                Position::new(1, 3),
                Position::new(7, 6),
                Position::new(6, 2),
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_complex() {
        let grid = create_grid_complex();

        let positions = grid.collect_anti_nodes();

        assert_eq!(
            positions,
            [
                Position::new(0, 6),
                Position::new(0, 11),
                Position::new(1, 3),
                Position::new(2, 4),
                Position::new(2, 10),
                Position::new(3, 2),
                Position::new(4, 9),
                Position::new(5, 1),
                Position::new(5, 6),
                Position::new(6, 3),
                Position::new(7, 0),
                Position::new(7, 7),
                Position::new(10, 10),
                Position::new(11, 10)
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_simple_recursive() {
        let grid = create_grid_simple();
        grid.set_algorithm(Part::Part2);

        let positions = grid.collect_anti_nodes();
        assert_eq!(
            positions,
            [
                Position::new(1, 3),
                Position::new(3, 4),
                Position::new(5, 5),
                Position::new(7, 6),
                Position::new(9, 7)
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_simple_recursive_reversed() {
        let grid = create_grid_simple_reversed();
        grid.set_algorithm(Part::Part2);

        let positions = grid.collect_anti_nodes();
        assert_eq!(
            positions,
            [
                Position::new(1, 6),
                Position::new(3, 5),
                Position::new(5, 4),
                Position::new(7, 3),
                Position::new(9, 2)
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_complex_recursive() {
        let grid = create_grid_complex();
        grid.set_algorithm(Part::Part2);

        let positions = grid.collect_anti_nodes();

        assert_eq!(
            positions,
            [
                Position::new(0, 0),
                Position::new(0, 1),
                Position::new(0, 6),
                Position::new(0, 11),
                Position::new(1, 1),
                Position::new(1, 3),
                Position::new(1, 8),
                Position::new(2, 2),
                Position::new(2, 4),
                Position::new(2, 5),
                Position::new(2, 10),
                Position::new(3, 2),
                Position::new(3, 3),
                Position::new(3, 7),
                Position::new(4, 4),
                Position::new(4, 9),
                Position::new(5, 1),
                Position::new(5, 5),
                Position::new(5, 6),
                Position::new(5, 11),
                Position::new(6, 3),
                Position::new(6, 6),
                Position::new(7, 0),
                Position::new(7, 5),
                Position::new(7, 7),
                Position::new(8, 2),
                Position::new(8, 8),
                Position::new(9, 4),
                Position::new(9, 9),
                Position::new(10, 1),
                Position::new(10, 10),
                Position::new(11, 3),
                Position::new(11, 10),
                Position::new(11, 11),
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_collect_anti_nodes_t_complex_recursive() {
        let grid = create_grid_t_complex();
        grid.set_algorithm(Part::Part2);

        let positions = grid.collect_anti_nodes();

        assert_eq!(
            positions,
            [
                Position::new(0, 0),
                Position::new(0, 5),
                Position::new(1, 3),
                Position::new(2, 1),
                Position::new(2, 6),
                Position::new(3, 9),
                Position::new(4, 2),
                Position::new(6, 3),
                Position::new(8, 4),
            ]
            .into_iter()
            .collect::<HashSet<_>>()
        );
    }
}
