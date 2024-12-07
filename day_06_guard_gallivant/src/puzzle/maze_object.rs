#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MazeObject {
    Empty,
    Obstruction,
    NewObstruction,
}
