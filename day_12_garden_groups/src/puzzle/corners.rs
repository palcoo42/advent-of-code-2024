use super::position::Position;

#[derive(Debug, Default, PartialEq)]
pub struct Corners {
    pub north: Option<Position>,
    pub north_west: Option<Position>,
    pub west: Option<Position>,
    pub south_west: Option<Position>,
    pub south: Option<Position>,
    pub south_east: Option<Position>,
    pub east: Option<Position>,
    pub north_east: Option<Position>,
}
