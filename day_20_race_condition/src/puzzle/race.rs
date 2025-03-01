use std::collections::{HashMap, HashSet};

use advent_of_code::{
    grids::{direction::Direction, grid::Grid, point::Point},
    puzzles::puzzle_error::PuzzleError,
};

use super::{cheat::Cheat, path::Path};

#[derive(Default)]
pub struct Race {
    grid: Grid,
}

impl Race {
    pub fn new(grid: Grid) -> Self {
        Self { grid }
    }

    pub fn count_cheats(&self, picoseconds: usize) -> Result<usize, PuzzleError> {
        // Build path to the destination
        let (start, end) = self.get_start_end()?;
        let path = self.get_path(&start, &end)?;

        let cheats = self.collect_cheats(&path)?;

        // Count number of cheats whose saved more than "picoseconds"
        let count = cheats
            .iter()
            .filter_map(|(saved, cheats)| {
                if *saved >= picoseconds as isize {
                    Some(cheats.len())
                } else {
                    None
                }
            })
            .sum();

        Ok(count)
    }

    fn get_path(&self, start: &Point, end: &Point) -> Result<Path, PuzzleError> {
        let mut path = Path::new();
        let mut visited = HashSet::new(); // Do not return back in the path
        let mut current = *start;
        let mut distance = 0;

        while current != *end {
            visited.insert(current);
            path.push(current, distance);

            let neighbors = self
                .grid
                .neighbors_if(&current, &Direction::CARDINAL, |p, _| {
                    !visited.contains(p) && self.grid[*p] != '#'
                });

            // There is exactly one path
            if neighbors.len() != 1 {
                return Err(PuzzleError::InvalidContentError(format!(
                    "Multiple neighbors for path found [{:?}]",
                    current
                )));
            }

            distance += 1;
            current = neighbors[0].0;
        }

        // Add end position
        path.push(current, distance);

        Ok(path)
    }

    fn get_start_end(&self) -> Result<(Point, Point), PuzzleError> {
        let start = self.grid.get_value('S');
        if start.len() != 1 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Exactly one 'S' tile is expected [{}]",
                start.len()
            )));
        }

        let end = self.grid.get_value('E');
        if end.len() != 1 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Exactly one 'E' tile is expected [{}]",
                end.len()
            )));
        }

        Ok((start[0], end[0]))
    }

    fn collect_cheats(&self, path: &Path) -> Result<HashMap<isize, Vec<Cheat>>, PuzzleError> {
        let mut cheats = HashMap::new();

        // Investigate only points which are in the path
        for (current_point, current_distance) in path.iter() {
            // Fetch cheats for current point
            let valid_cheats = self.get_valid_cheats(current_point, path);

            for cheat in valid_cheats {
                if let Some(end_distance) = path.get(&cheat.end) {
                    let saved = end_distance as isize - *current_distance as isize - 2;

                    // Append cheat to correct "saved" slot
                    let saved_cheats = cheats.entry(saved).or_insert(vec![]);
                    saved_cheats.push(cheat);
                }
            }
        }

        Ok(cheats)
    }

    fn get_valid_cheats(&self, point: &Point, path: &Path) -> Vec<Cheat> {
        const STEPS: isize = 2;

        let cheats = vec![
            Cheat {
                start: *point,
                end: Point {
                    x: point.x + STEPS,
                    y: point.y,
                },
            },
            Cheat {
                start: *point,
                end: Point {
                    x: point.x - STEPS,
                    y: point.y,
                },
            },
            Cheat {
                start: *point,
                end: Point {
                    x: point.x,
                    y: point.y + STEPS,
                },
            },
            Cheat {
                start: *point,
                end: Point {
                    x: point.x,
                    y: point.y - STEPS,
                },
            },
        ];

        // Valid cheat is when:
        // - End point is in a grid
        // - End point is in a path
        cheats
            .into_iter()
            .filter(|cheat| {
                self.grid.is_point_in_grid(&cheat.end) && path.get(&cheat.end).is_some()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_race() -> Race {
        let lines = [
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ];

        Race::new(
            Grid::new_from_lines(&lines)
                .unwrap_or_else(|err| panic!("Failed to create Grid from lines [{}]", err)),
        )
    }

    fn build_path_from_race(race: &Race) -> (Point, Point, Path) {
        let (start, end) = race.get_start_end().expect("Failed to get Start / End");
        let path = race.get_path(&start, &end).expect("Failed to get path");
        (start, end, path)
    }

    #[test]
    fn test_get_path() {
        let race = build_race();
        let (start, end) = race.get_start_end().unwrap();
        let result = race.get_path(&start, &end);

        assert!(result.is_ok(), "result: {:?}", result);

        let path = result.unwrap();
        let mut path_iter = path.iter();

        assert_eq!(path_iter.next(), Some(&(Point { x: 1, y: 3 }, 0)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 1, y: 2 }, 1)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 1, y: 1 }, 2)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 2, y: 1 }, 3)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 3, y: 1 }, 4)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 3, y: 2 }, 5)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 3, y: 3 }, 6)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 4, y: 3 }, 7)));
        assert_eq!(path_iter.next(), Some(&(Point { x: 5, y: 3 }, 8)));

        assert_eq!(path.get(&start), Some(0));
        assert_eq!(path.get(&end), Some(84));
        assert_eq!(path.len(), 85);
    }

    #[test]
    fn test_collect_cheats() {
        let race = build_race();
        let (_, _, path) = build_path_from_race(&race);

        let result = race.collect_cheats(&path);
        assert!(result.is_ok(), "result: {:?}", result);

        let cheats = result.unwrap();

        assert_eq!(cheats.get(&2).unwrap().len(), 14);
        assert_eq!(cheats.get(&4).unwrap().len(), 14);
        assert_eq!(cheats.get(&6).unwrap().len(), 2);
        assert_eq!(cheats.get(&8).unwrap().len(), 4);
        assert_eq!(cheats.get(&10).unwrap().len(), 2);
        assert_eq!(cheats.get(&12).unwrap().len(), 3);
        assert_eq!(cheats.get(&20).unwrap().len(), 1);
        assert_eq!(cheats.get(&36).unwrap().len(), 1);
        assert_eq!(cheats.get(&38).unwrap().len(), 1);
        assert_eq!(cheats.get(&40).unwrap().len(), 1);
        assert_eq!(cheats.get(&64).unwrap().len(), 1);
    }
}
