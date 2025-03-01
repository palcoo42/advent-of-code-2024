use advent_of_code::grids::point::Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cheat {
    pub start: Point,
    pub end: Point,
}
