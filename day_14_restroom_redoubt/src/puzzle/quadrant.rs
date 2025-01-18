use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct Quadrant {
    pub x: Range<usize>,
    pub y: Range<usize>,
}
