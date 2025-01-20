use super::{movement::Movement, tile::Tile, tile_index::TileIndex};

#[derive(Debug, Default, PartialEq)]
pub struct Warehouse {
    rows: usize,
    cols: usize,
    tiles: Vec<Tile>,
}

impl Warehouse {
    pub fn new(rows: usize, cols: usize, tiles: Vec<Tile>) -> Self {
        Self { rows, cols, tiles }
    }

    pub fn move_boxes(&mut self, movements: &[Movement]) {
        // Todo: To speed up keep track of robot's current position
        let mut robot = self.get_robot_index();

        for movement in movements {
            let analyzed_tiles = match movement {
                Movement::Right => self.collect_tiles_right(robot),
                Movement::Down => self.collect_tiles_down(robot),
                Movement::Left => self.collect_tiles_left(robot),
                Movement::Up => self.collect_tiles_up(robot),
            };

            // Move analyzed items
            let tiles = analyzed_tiles
                .iter()
                .map(|t| t.tile.clone())
                .collect::<Vec<_>>();

            let indexes = analyzed_tiles.iter().map(|t| t.index).collect::<Vec<_>>();

            let moved_tiles = Self::analyze_movement(&tiles);
            self.move_tiles(indexes, moved_tiles, &mut robot);
        }
    }

    fn collect_tiles_right(&self, robot: usize) -> Vec<TileIndex> {
        // Collect tiles which we need to analyze
        let mut tiles = Vec::with_capacity(self.cols);

        // Append robot's position
        tiles.push(TileIndex::new(Tile::Robot, robot));

        // Get all tiles until first Wall to speed up
        let row_max = (robot / self.cols + 1) * self.cols;
        let mut next_index = robot + 1;

        while next_index < row_max {
            let tile = &self.tiles[next_index];

            tiles.push(TileIndex::new(tile.clone(), next_index));

            // Stop on first Wall
            if *tile == Tile::Wall {
                break;
            }

            next_index += 1;
        }

        tiles
    }

    fn collect_tiles_down(&self, robot: usize) -> Vec<TileIndex> {
        // Collect tiles which we need to analyze
        let mut tiles = Vec::with_capacity(self.rows);

        // Append robot's position
        tiles.push(TileIndex::new(Tile::Robot, robot));

        // Get all tiles until first Wall to speed up
        let col_max = self.rows * self.cols - 1;
        let mut next_index = robot + self.cols;

        while next_index < col_max {
            let tile = &self.tiles[next_index];

            tiles.push(TileIndex::new(tile.clone(), next_index));

            // Stop on first Wall
            if *tile == Tile::Wall {
                break;
            }

            next_index += self.cols;
        }

        tiles
    }

    fn collect_tiles_left(&self, robot: usize) -> Vec<TileIndex> {
        // Collect tiles which we need to analyze
        let mut tiles = Vec::with_capacity(self.cols);

        // Append robot's position
        tiles.push(TileIndex::new(Tile::Robot, robot));

        // Get all tiles until first Wall to speed up
        let row_min = ((robot / self.cols) * self.cols) as isize;
        let mut next_index = robot as isize - 1;

        while next_index > row_min {
            let tile = &self.tiles[next_index as usize];

            tiles.push(TileIndex::new(tile.clone(), next_index as usize));

            // Stop on first Wall
            if *tile == Tile::Wall {
                break;
            }

            next_index -= 1;
        }

        tiles
    }

    fn collect_tiles_up(&self, robot: usize) -> Vec<TileIndex> {
        // Collect tiles which we need to analyze
        let mut tiles = Vec::with_capacity(self.rows);

        // Append robot's position
        tiles.push(TileIndex::new(Tile::Robot, robot));

        // Get all tiles until first Wall to speed up
        let mut next_index = robot as isize - self.cols as isize;

        while next_index > 0 {
            let tile = &self.tiles[next_index as usize];

            tiles.push(TileIndex::new(tile.clone(), next_index as usize));

            // Stop on first Wall
            if *tile == Tile::Wall {
                break;
            }

            next_index -= self.cols as isize;
        }

        tiles
    }

    pub fn gps_coordinates(&self) -> usize {
        // Collect indexes for all boxes and calculate GPS values
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| match tile {
                Tile::Box => Some(self.calculate_gps_coordinates(idx)),
                _ => None,
            })
            .sum()
    }

    fn calculate_gps_coordinates(&self, index: usize) -> usize {
        let x = index % self.cols;
        let y = index / self.cols;
        x + y * 100
    }

    fn get_robot_index(&self) -> usize {
        match self
            .tiles
            .iter()
            .enumerate()
            .find(|(_, t)| **t == Tile::Robot)
        {
            Some((idx, _)) => idx,
            None => panic!("Invalid workhouse, robot not found"),
        }
    }

    fn analyze_movement(tiles: &[Tile]) -> Vec<Tile> {
        // Find first empty tile
        let empty_tile = match tiles
            .iter()
            .enumerate()
            .find(|(_, tile)| **tile == Tile::Empty)
        {
            Some((idx, _)) => idx,
            None => return tiles.to_vec(),
        };

        // To be able to move to this tile there cannot be any Wall in between
        if tiles
            .iter()
            .skip(1)
            .take(empty_tile - 1)
            .any(|tile| *tile == Tile::Wall)
        {
            return tiles.to_vec();
        }

        // Now we can move everything until empty file
        let mut moved = Vec::with_capacity(tiles.len());
        moved.push(Tile::Empty);
        moved.push(Tile::Robot);

        // Copy moved values -> skip 1st item because it is robot which moves to the 2nd item
        let mut copied = tiles
            .iter()
            .skip(1)
            .take(empty_tile - 1)
            .cloned()
            .collect::<Vec<_>>();

        moved.append(&mut copied);

        // Append remaining tiles after empty_tile as these are not changed but are preserved
        let mut remaining = tiles
            .iter()
            .skip(empty_tile + 1)
            .cloned()
            .collect::<Vec<_>>();

        moved.append(&mut remaining);

        // Return new order of tiles. The size is the same as input collection
        moved
    }

    fn move_tiles(&mut self, indexes: Vec<usize>, moved_tiles: Vec<Tile>, robot: &mut usize) {
        // Sanity check
        assert_eq!(
            indexes.len(),
            moved_tiles.len(),
            "Length of 'indexes' and 'moved' tiles does not match"
        );

        // Move tiles
        for (idx, tile) in indexes.iter().zip(moved_tiles) {
            if tile == Tile::Robot {
                *robot = *idx;
            }
            self.tiles[*idx] = tile;
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let tile = match self.tiles[row * self.cols + col] {
                    Tile::Empty => '.',
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                    Tile::Robot => '@',
                };

                print!("{}", tile);
            }

            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::puzzle::parser::Parser;

    use super::*;

    fn build_small_warehouse() -> (Warehouse, Vec<Movement>, Warehouse) {
        let initial_warehouse = Parser::parse_warehouse(&[
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ])
        .expect("Failed to create initial warehouse");

        let movements =
            Parser::parse_movements(&["<^^>>>vv<v>>v<<"]).expect("Failed to create movements");

        let final_warehouse = Parser::parse_warehouse(&[
            "########", "#....OO#", "##.....#", "#.....O#", "#.#O@..#", "#...O..#", "#...O..#",
            "########",
        ])
        .expect("Failed to create final warehouse");

        (initial_warehouse, movements, final_warehouse)
    }

    fn build_large_warehouse() -> (Warehouse, Vec<Movement>, Warehouse) {
        let initial_warehouse = Parser::parse_warehouse(&[
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#..O@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ])
        .expect("Failed to create initial warehouse");

        let movements = Parser::parse_movements(&[
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        ])
        .expect("Failed to create movements");

        let final_warehouse = Parser::parse_warehouse(&[
            "##########",
            "#.O.O.OOO#",
            "#........#",
            "#OO......#",
            "#OO@.....#",
            "#O#.....O#",
            "#O.....OO#",
            "#O.....OO#",
            "#OO....OO#",
            "##########",
        ])
        .expect("Failed to create final warehouse");

        (initial_warehouse, movements, final_warehouse)
    }

    #[test]
    fn test_analyze_movement_wall() {
        let tiles = vec![Tile::Robot, Tile::Wall, Tile::Empty, Tile::Box, Tile::Wall];
        let expected = vec![Tile::Robot, Tile::Wall, Tile::Empty, Tile::Box, Tile::Wall];

        let moved = Warehouse::analyze_movement(&tiles);
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_analyze_movement_wall_box_box_wall() {
        let tiles = vec![Tile::Robot, Tile::Wall, Tile::Box, Tile::Box, Tile::Wall];
        let expected = vec![Tile::Robot, Tile::Wall, Tile::Box, Tile::Box, Tile::Wall];

        let moved = Warehouse::analyze_movement(&tiles);
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_analyze_movement_box() {
        let tiles = vec![Tile::Robot, Tile::Box, Tile::Empty, Tile::Box, Tile::Wall];
        let expected = vec![Tile::Empty, Tile::Robot, Tile::Box, Tile::Box, Tile::Wall];

        let moved = Warehouse::analyze_movement(&tiles);
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_analyze_movement_box_box() {
        let tiles = vec![Tile::Robot, Tile::Box, Tile::Box, Tile::Empty, Tile::Wall];
        let expected = vec![Tile::Empty, Tile::Robot, Tile::Box, Tile::Box, Tile::Wall];

        let moved = Warehouse::analyze_movement(&tiles);
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_analyze_movement_box_box_box_wall() {
        let tiles = vec![Tile::Robot, Tile::Box, Tile::Box, Tile::Box, Tile::Wall];
        let expected = vec![Tile::Robot, Tile::Box, Tile::Box, Tile::Box, Tile::Wall];

        let moved = Warehouse::analyze_movement(&tiles);
        assert_eq!(moved, expected);
    }

    #[test]
    fn test_move_boxes_small_steps() {
        let (mut warehouse, _, _) = build_small_warehouse();

        let input = [
            (
                Movement::Left,
                Parser::parse_warehouse(&[
                    "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Up,
                Parser::parse_warehouse(&[
                    "########", "#.@O.O.#", "##..O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Up,
                Parser::parse_warehouse(&[
                    "########", "#.@O.O.#", "##..O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Right,
                Parser::parse_warehouse(&[
                    "########", "#..@OO.#", "##..O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Right,
                Parser::parse_warehouse(&[
                    "########", "#...@OO#", "##..O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Right,
                Parser::parse_warehouse(&[
                    "########", "#...@OO#", "##..O..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#......#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Down,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##..@..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Down,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##..@..#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Left,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.@...#", "#...O..#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Down,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#..@O..#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Right,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#...@O.#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Right,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#....@O#", "#.#.O..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Down,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#.....O#", "#.#.O@.#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Left,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#.....O#", "#.#O@..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
            (
                Movement::Left,
                Parser::parse_warehouse(&[
                    "########", "#....OO#", "##.....#", "#.....O#", "#.#O@..#", "#...O..#",
                    "#...O..#", "########",
                ])
                .unwrap(),
            ),
        ];

        // Compare step by step progress of the movements
        for (step, (movement, final_warehouse)) in input.iter().enumerate() {
            // Move warehouse items by a single movement at a time
            warehouse.move_boxes(&[movement.clone()]);

            assert_eq!(
                warehouse, *final_warehouse,
                "Error at step: '{}' movement: {:?}",
                step, movement
            );

            // warehouse.print();
        }
    }

    #[test]
    fn test_move_boxes_small() {
        let (mut warehouse, movements, final_warehouse) = build_small_warehouse();

        warehouse.move_boxes(&movements);

        assert_eq!(warehouse, final_warehouse);
        assert_eq!(warehouse.gps_coordinates(), 2028);
    }

    #[test]
    fn test_move_boxes_large() {
        let (mut warehouse, movements, final_warehouse) = build_large_warehouse();

        warehouse.move_boxes(&movements);

        assert_eq!(warehouse, final_warehouse);
        assert_eq!(warehouse.gps_coordinates(), 10092);
    }
}
