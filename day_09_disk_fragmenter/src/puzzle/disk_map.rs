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

    pub fn compact_per_block_get_checksum(&self) -> usize {
        // Expand raw format
        let mut expanded = Self::expand(&self.dense_format);

        // Compact file format
        Self::compact_per_block(&mut expanded);

        // Calculate checksum
        Self::calculate_checksum(&expanded)
    }

    pub fn compact_per_file_get_checksum(&self) -> usize {
        // Expand raw format
        let mut expanded = Self::expand(&self.dense_format);

        // Compact file format
        Self::compact_per_file(&mut expanded);

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

    fn compact_per_block(blocks: &mut [Block]) {
        let mut free = Self::find_free_from_left(blocks, 0);
        let mut file = Self::find_block_from_right(blocks, blocks.len() - 1);

        while let (Some(free_index), Some(file_index)) = (free, file) {
            // Check for the end condition
            if free_index > file_index {
                break;
            }

            // Swap elements
            blocks.swap(free_index, file_index);

            // Find next candidates
            free = Self::find_free_from_left(blocks, free_index + 1);
            file = Self::find_block_from_right(blocks, file_index - 1);
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

    fn find_block_from_right(blocks: &[Block], to: usize) -> Option<usize> {
        for index in (0..=to).rev() {
            if let Block::File { id: _ } = blocks[index] {
                return Some(index);
            }
        }

        None
    }

    fn compact_per_file(blocks: &mut [Block]) {
        let mut right_position = blocks.len() - 1;
        let mut file;
        let mut free;

        while right_position > 0 {
            file = Self::find_file_from_right(blocks, right_position);
            free = Self::find_free_from_left_from_file(blocks, 0, file);

            if let (
                Some((free_index_from, free_index_to)),
                Some((file_index_from, file_index_to)),
            ) = (free, file)
            {
                // Swap only if free is before file
                if free_index_from > file_index_from {
                    right_position = file_index_from - 1;
                    continue;
                }

                // Swap elements
                for (free_index, file_index) in
                    (free_index_from..=free_index_to).zip(file_index_from..=file_index_to)
                {
                    blocks.swap(free_index, file_index);
                }

                // Update position
                right_position = file_index_from - 1;
            } else if let Some((file_index_from, _file_index_to)) = file {
                // Update position
                right_position = file_index_from - 1;
            } else {
                right_position -= 1;
            }
        }
    }

    fn find_file_from_right(blocks: &[Block], to: usize) -> Option<(usize, usize)> {
        let block_index = Self::find_block_from_right(blocks, to)?;

        // Find all blocks
        if let Block::File { id } = blocks[block_index] {
            for index in (0..block_index).rev() {
                match blocks[index] {
                    Block::File { id: current_id } => {
                        if current_id != id {
                            return Some((index + 1, block_index));
                        }
                    }
                    Block::Free => {
                        return Some((index + 1, block_index));
                    }
                }
            }

            return Some((block_index, block_index));
        }

        panic!("{:?} expected to be File", blocks[block_index]);
    }

    fn find_free_from_left_from_file(
        blocks: &[Block],
        from: usize,
        file: Option<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        if let Some((file_index_from, file_index_to)) = file {
            return Self::find_free_from_left_len(
                blocks,
                from,
                file_index_to - file_index_from + 1,
            );
        }

        None
    }

    fn find_free_from_left_len(
        blocks: &[Block],
        from: usize,
        len: usize,
    ) -> Option<(usize, usize)> {
        // Find first free spot with given length
        let mut index = from;

        while index < blocks.len() {
            match Self::find_free_from_left(blocks, index) {
                Some(start_index) => {
                    // Check if we have sufficient free space
                    if blocks.iter().skip(start_index).take(len).all(|b| match b {
                        Block::File { id: _ } => false,
                        Block::Free => true,
                    }) {
                        return Some((start_index, start_index + len - 1));
                    } else {
                        // Find next free space
                        index = start_index + 1;
                    }
                }
                None => break,
            }
        }

        None
    }

    #[allow(dead_code)]
    fn blocks_to_string(blocks: &[Block]) -> String {
        blocks
            .iter()
            .map(|b| match b {
                Block::File { id } => id.to_string(),
                Block::Free => ".".to_string(),
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_per_block_get_checksum() {
        let disk_map = DiskMap::new("2333133121414131402");
        assert_eq!(disk_map.compact_per_block_get_checksum(), 1928);
    }

    #[test]
    fn test_compact_per_file_get_checksum() {
        let disk_map = DiskMap::new("2333133121414131402");
        assert_eq!(disk_map.compact_per_file_get_checksum(), 2858);
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
    fn test_compact_per_block() {
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

        DiskMap::compact_per_block(&mut blocks);

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
    fn test_find_block_from_right() {
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

        assert_eq!(DiskMap::find_block_from_right(&blocks, 14), Some(14));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 13), Some(13));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 12), Some(12));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 11), Some(11));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 10), Some(10));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 9), Some(5));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 8), Some(5));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 7), Some(5));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 6), Some(5));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 5), Some(5));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 4), Some(4));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 3), Some(3));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 2), Some(0));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 1), Some(0));
        assert_eq!(DiskMap::find_block_from_right(&blocks, 0), Some(0));
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

    #[test]
    fn test_compact_per_file() {
        let mut blocks = vec![
            Block::File { id: 0 },
            Block::File { id: 0 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::File { id: 1 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 2 },
            Block::Free,
            Block::Free,
            Block::Free,
            Block::File { id: 3 },
            Block::File { id: 3 },
            Block::File { id: 3 },
            Block::Free,
            Block::File { id: 4 },
            Block::File { id: 4 },
            Block::Free,
            Block::File { id: 5 },
            Block::File { id: 5 },
            Block::File { id: 5 },
            Block::File { id: 5 },
            Block::Free,
            Block::File { id: 6 },
            Block::File { id: 6 },
            Block::File { id: 6 },
            Block::File { id: 6 },
            Block::Free,
            Block::File { id: 7 },
            Block::File { id: 7 },
            Block::File { id: 7 },
            Block::Free,
            Block::File { id: 8 },
            Block::File { id: 8 },
            Block::File { id: 8 },
            Block::File { id: 8 },
            Block::File { id: 9 },
            Block::File { id: 9 },
        ];

        DiskMap::compact_per_file(&mut blocks);

        assert_eq!(
            blocks,
            vec![
                Block::File { id: 0 },
                Block::File { id: 0 },
                Block::File { id: 9 },
                Block::File { id: 9 },
                Block::File { id: 2 },
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::File { id: 1 },
                Block::File { id: 7 },
                Block::File { id: 7 },
                Block::File { id: 7 },
                Block::Free,
                Block::File { id: 4 },
                Block::File { id: 4 },
                Block::Free,
                Block::File { id: 3 },
                Block::File { id: 3 },
                Block::File { id: 3 },
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File { id: 5 },
                Block::File { id: 5 },
                Block::File { id: 5 },
                Block::File { id: 5 },
                Block::Free,
                Block::File { id: 6 },
                Block::File { id: 6 },
                Block::File { id: 6 },
                Block::File { id: 6 },
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::Free,
                Block::File { id: 8 },
                Block::File { id: 8 },
                Block::File { id: 8 },
                Block::File { id: 8 },
                Block::Free,
                Block::Free,
            ]
        );
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

        assert_eq!(DiskMap::find_file_from_right(&blocks, 14), Some((10, 14)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 13), Some((10, 13)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 12), Some((10, 12)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 11), Some((10, 11)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 10), Some((10, 10)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 9), Some((3, 5)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 8), Some((3, 5)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 7), Some((3, 5)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 6), Some((3, 5)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 5), Some((3, 5)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 4), Some((3, 4)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 3), Some((3, 3)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 2), Some((0, 0)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 1), Some((0, 0)));
        assert_eq!(DiskMap::find_file_from_right(&blocks, 0), Some((0, 0)));
    }

    #[test]
    fn test_find_free_from_left_len() {
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

        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 0, 1),
            Some((1, 1))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 0, 2),
            Some((1, 2))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 0, 3),
            Some((6, 8))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 0, 4),
            Some((6, 9))
        );
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 0, 5), None);

        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 1, 1),
            Some((1, 1))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 1, 2),
            Some((1, 2))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 1, 4),
            Some((6, 9))
        );
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 1, 5), None);

        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 2, 1),
            Some((2, 2))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 2, 2),
            Some((6, 7))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 2, 4),
            Some((6, 9))
        );
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 2, 5), None);

        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 3, 1),
            Some((6, 6))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 3, 2),
            Some((6, 7))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 3, 3),
            Some((6, 8))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 3, 4),
            Some((6, 9))
        );
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 3, 5), None);

        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 6, 1),
            Some((6, 6))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 6, 2),
            Some((6, 7))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 6, 3),
            Some((6, 8))
        );
        assert_eq!(
            DiskMap::find_free_from_left_len(&blocks, 6, 4),
            Some((6, 9))
        );
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 6, 5), None);

        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 10, 1), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 10, 2), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 10, 3), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 10, 4), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 10, 5), None);

        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 14, 1), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 14, 2), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 14, 3), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 14, 4), None);
        assert_eq!(DiskMap::find_free_from_left_len(&blocks, 14, 5), None);
    }
}
