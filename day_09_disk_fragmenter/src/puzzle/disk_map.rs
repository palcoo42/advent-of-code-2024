use super::block::Block;

#[derive(Default)]
pub struct DiskMap {
    dense_format: String,
}

impl DiskMap {
    pub fn new(dense_format: &str) -> Self {
        Self {
            dense_format: String::from(dense_format),
        }
    }

    pub fn compact_get_checksum(&self) -> usize {
        // Expand raw format
        let mut expanded = Self::expand(&self.dense_format);

        // Compact file format
        Self::compact(&mut expanded);

        // Calculate checksum
        Self::calculate_checksum(&expanded)
    }

    fn calculate_checksum(blocks: &[Block]) -> usize {
        blocks
            .iter()
            .enumerate()
            .filter_map(|(index, block)| match block {
                Block::File { id } => Some(index * id),
                Block::Free => None,
            })
            .sum()
    }

    fn expand(dense_format: &str) -> Vec<Block> {
        // NOTE: Warning - magic number 10
        let mut blocks = Vec::with_capacity(dense_format.len() * 10);

        for (id, c) in dense_format.chars().enumerate() {
            let number = c.to_digit(10).unwrap_or_else(|| {
                panic!("Invalid non-numeric digit '{}' detected", c);
            }) as usize;

            let mut data = match id % 2 == 0 {
                // Block
                true => {
                    let block_id = id / 2; // enumerate() counts also free space
                    vec![Block::File { id: block_id }; number]
                }
                // Free space
                false => {
                    vec![Block::Free; number]
                }
            };

            blocks.append(&mut data);
        }

        blocks
    }

    fn compact(blocks: &mut [Block]) {
        let mut free = Self::find_free_from_left(blocks, 0);
        let mut file = Self::find_file_from_right(blocks, blocks.len() - 1);

        while let (Some(free_index), Some(file_index)) = (free, file) {
            // Check for the end condition
            if free_index > file_index {
                break;
            }

            // Swap elements
            blocks.swap(free_index, file_index);

            // Find next candidates
            free = Self::find_free_from_left(blocks, free_index + 1);
            file = Self::find_file_from_right(blocks, file_index - 1);
        }
    }

    fn find_free_from_left(blocks: &[Block], from: usize) -> Option<usize> {
        blocks
            .iter()
            .enumerate()
            .skip(from)
            .find_map(|(index, block)| match block {
                Block::File { id: _ } => None,
                Block::Free => Some(index),
            })
    }

    fn find_file_from_right(blocks: &[Block], to: usize) -> Option<usize> {
        for index in (0..=to).rev() {
            if let Block::File { id: _ } = blocks[index] {
                return Some(index);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_get_checksum() {
        let disk_map = DiskMap::new("2333133121414131402");
        assert_eq!(disk_map.compact_get_checksum(), 1928);
    }

    #[test]
    fn test_expand() {
        assert_eq!(
            DiskMap::expand("12345"),
            vec![
                Block::File { id: 0 },
                Block::Free,
                Block::Free,
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::File { id: 2 }
            ]
        );
    }

    #[test]
    fn test_compact() {
        let mut blocks = vec![
            Block::File { id: 0 },
            Block::Free,
            Block::Free,
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
        ];

        DiskMap::compact(&mut blocks);

        assert_eq!(
            blocks,
            vec![
                Block::File { id: 0 },
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::File { id: 2 },
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
            ]
        );
    }

    #[test]
    fn test_find_free_from_left() {
        let blocks = vec![
            Block::File { id: 0 },
            Block::Free,
            Block::Free,
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
        ];

        assert_eq!(DiskMap::find_free_from_left(&blocks, 0), Some(1));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 1), Some(1));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 2), Some(2));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 3), Some(6));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 4), Some(6));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 5), Some(6));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 6), Some(6));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 7), Some(7));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 8), Some(8));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 9), Some(9));
        assert_eq!(DiskMap::find_free_from_left(&blocks, 10), None);
        assert_eq!(DiskMap::find_free_from_left(&blocks, 11), None);
        assert_eq!(DiskMap::find_free_from_left(&blocks, 12), None);
        assert_eq!(DiskMap::find_free_from_left(&blocks, 13), None);
        assert_eq!(DiskMap::find_free_from_left(&blocks, 14), None);
    }

    #[test]
    fn test_find_file_from_right() {
        let blocks = vec![
            Block::File { id: 0 },
            Block::Free,
            Block::Free,
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
        ];

        assert_eq!(DiskMap::find_file_from_right(&blocks, 14), Some(14));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 13), Some(13));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 12), Some(12));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 11), Some(11));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 10), Some(10));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 9), Some(5));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 8), Some(5));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 7), Some(5));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 6), Some(5));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 5), Some(5));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 4), Some(4));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 3), Some(3));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 2), Some(0));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 1), Some(0));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 0), Some(0));
    }

    #[test]
    fn test_calculate_checksum() {
        let blocks = vec![
            Block::File { id: 0 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::File { id: 2 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
        ];

        assert_eq!(DiskMap::calculate_checksum(&blocks), 60);
    }
}
