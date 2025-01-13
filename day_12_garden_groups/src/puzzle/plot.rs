use super::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum Plot {
    Border,
    Different,
    Same(Position),
}
