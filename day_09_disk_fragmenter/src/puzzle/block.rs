#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    File { id: usize },
    Free,
}
