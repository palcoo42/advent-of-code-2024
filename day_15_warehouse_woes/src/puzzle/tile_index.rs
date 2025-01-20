use super::tile::Tile;

#[derive(Debug, Clone, PartialEq)]
pub struct TileIndex {
    pub tile: Tile,
    pub index: usize,
}

impl TileIndex {
    pub fn new(tile: Tile, index: usize) -> Self {
        Self { tile, index }
    }
}
