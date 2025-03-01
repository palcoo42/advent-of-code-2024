use advent_of_code::grids::point::Point;

#[derive(Debug, Default)]
pub struct Path {
    internal: Vec<(Point, usize)>,
}

impl Path {
    pub fn new() -> Self {
        Self { internal: vec![] }
    }

    pub fn push(&mut self, point: Point, distance_to_finish: usize) {
        self.internal.push((point, distance_to_finish));
    }

    pub fn get(&self, point: &Point) -> Option<usize> {
        self.internal
            .iter()
            .find_map(|(p, distance)| if p == point { Some(*distance) } else { None })
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Point, usize)> {
        self.internal.iter()
    }

    pub fn len(&self) -> usize {
        self.internal.len()
    }

    pub fn is_empty(&self) -> bool {
        self.internal.is_empty()
    }
}
