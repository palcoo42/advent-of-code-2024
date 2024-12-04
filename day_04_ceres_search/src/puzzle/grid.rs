use std::fmt::Display;

pub struct Grid {
    internal: Vec<Vec<char>>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            internal: Default::default(),
        }
    }
}

impl Grid {
    pub fn new(lines: Vec<String>) -> Self {
        let internal = lines.iter().map(|line| line.chars().collect()).collect();

        Self { internal }
    }

    pub fn find_word_count(&self, word: &str) -> usize {
        let mut count = 0;

        for (row_idx, row) in self.internal.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                // Collect words for given position
                let words = self.spawn_possible_texts(row_idx, col_idx, word.len());

                // Update word count
                count += words.iter().filter(|&w| w == word).count();
            }
        }

        count
    }

    fn spawn_possible_texts(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Vec<String> {
        // There are up to 8 possible ways how to create a text
        let texts = vec![
            self.get_text_right(row_idx, col_idx, word_len),
            self.get_text_right_down(row_idx, col_idx, word_len),
            self.get_text_down(row_idx, col_idx, word_len),
            self.get_text_left_down(row_idx, col_idx, word_len),
            self.get_text_left(row_idx, col_idx, word_len),
            self.get_text_left_up(row_idx, col_idx, word_len),
            self.get_text_up(row_idx, col_idx, word_len),
            self.get_text_right_up(row_idx, col_idx, word_len),
        ];

        // Remove None elements
        texts.into_iter().flatten().collect()
    }

    fn get_text_right(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, 0, col_idx, 1, word_len)
    }

    fn get_text_right_down(
        &self,
        row_idx: usize,
        col_idx: usize,
        word_len: usize,
    ) -> Option<String> {
        self.get_text(row_idx, 1, col_idx, 1, word_len)
    }

    fn get_text_down(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, 1, col_idx, 0, word_len)
    }

    fn get_text_left_down(
        &self,
        row_idx: usize,
        col_idx: usize,
        word_len: usize,
    ) -> Option<String> {
        self.get_text(row_idx, 1, col_idx, -1, word_len)
    }

    fn get_text_left(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, 0, col_idx, -1, word_len)
    }

    fn get_text_left_up(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, -1, col_idx, -1, word_len)
    }

    fn get_text_up(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, -1, col_idx, 0, word_len)
    }

    fn get_text_right_up(&self, row_idx: usize, col_idx: usize, word_len: usize) -> Option<String> {
        self.get_text(row_idx, -1, col_idx, 1, word_len)
    }

    fn get_text(
        &self,
        row_idx: usize,
        row_diff: isize,
        col_idx: usize,
        col_diff: isize,
        word_len: usize,
    ) -> Option<String> {
        let row_length = self.internal.len() as isize;
        let column_length = self.internal[row_idx].len() as isize;

        let mut row_index = row_idx as isize;
        let mut column_index = col_idx as isize;

        let mut text = String::with_capacity(word_len);

        while row_index >= 0
            && column_index >= 0
            && row_index < row_length
            && column_index < column_length
            && text.len() != word_len
        {
            text.push(self.internal[row_index as usize][column_index as usize]);

            row_index = row_index as isize + row_diff;
            column_index = column_index as isize + col_diff;
        }

        match text.len() == word_len {
            true => Some(text),
            false => None,
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.internal {
            for col in row {
                let _ = write!(f, "{}", col);
            }
            let _ = writeln!(f);
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_grid() -> Grid {
        Grid::new(vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ])
    }

    #[test]
    fn test_get_text_right() {
        let grid = create_grid();
        assert_eq!(grid.get_text_right(0, 0, 4), Some("MMMS".to_string()));
        assert_eq!(grid.get_text_right(0, 1, 4), Some("MMSX".to_string()));
        assert_eq!(grid.get_text_right(0, 6, 4), Some("MASM".to_string()));
        assert_eq!(grid.get_text_right(0, 7, 4), None);
        assert_eq!(grid.get_text_right(0, 8, 4), None);
        assert_eq!(grid.get_text_right(0, 9, 4), None);
    }

    #[test]
    fn test_get_text_right_down() {
        let grid = create_grid();
        assert_eq!(grid.get_text_right_down(0, 0, 4), Some("MSXM".to_string()));
        assert_eq!(grid.get_text_right_down(0, 1, 4), Some("MASA".to_string()));
        assert_eq!(grid.get_text_right_down(0, 6, 4), Some("MMMX".to_string()));
        assert_eq!(grid.get_text_right_down(0, 7, 4), None);
        assert_eq!(grid.get_text_right_down(0, 8, 4), None);
        assert_eq!(grid.get_text_right_down(0, 9, 4), None);
        assert_eq!(grid.get_text_right_down(6, 0, 4), Some("SAMX".to_string()));
        assert_eq!(grid.get_text_right_down(6, 6, 4), Some("SAMX".to_string()));
        assert_eq!(grid.get_text_right_down(7, 6, 4), None);
        assert_eq!(grid.get_text_right_down(8, 0, 4), None);
        assert_eq!(grid.get_text_right_down(9, 0, 4), None);
    }

    #[test]
    fn test_get_text_down() {
        let grid = create_grid();
        assert_eq!(grid.get_text_down(0, 0, 4), Some("MMAM".to_string()));
        assert_eq!(grid.get_text_down(1, 0, 4), Some("MAMX".to_string()));
        assert_eq!(grid.get_text_down(2, 0, 4), Some("AMXX".to_string()));
        assert_eq!(grid.get_text_down(6, 0, 4), Some("SSMM".to_string()));
        assert_eq!(grid.get_text_down(7, 0, 4), None);
        assert_eq!(grid.get_text_down(8, 0, 4), None);
        assert_eq!(grid.get_text_down(9, 0, 4), None);
    }

    #[test]
    fn test_get_text_left_down() {
        let grid = create_grid();
        assert_eq!(grid.get_text_left_down(0, 0, 4), None);
        assert_eq!(grid.get_text_left_down(0, 1, 4), None);
        assert_eq!(grid.get_text_left_down(0, 2, 4), None);
        assert_eq!(grid.get_text_left_down(0, 3, 4), Some("SAMM".to_string()));
        assert_eq!(grid.get_text_left_down(0, 8, 4), Some("SMAS".to_string()));
        assert_eq!(grid.get_text_left_down(0, 9, 4), Some("MSAM".to_string()));
        assert_eq!(grid.get_text_left_down(6, 9, 4), Some("SAMM".to_string()));
        assert_eq!(grid.get_text_left_down(7, 9, 4), None);
        assert_eq!(grid.get_text_left_down(8, 9, 4), None);
        assert_eq!(grid.get_text_left_down(9, 9, 4), None);
        assert_eq!(grid.get_text_left_down(6, 3, 4), Some("MXAM".to_string()));
        assert_eq!(grid.get_text_left_down(6, 2, 4), None);
        assert_eq!(grid.get_text_left_down(6, 1, 4), None);
        assert_eq!(grid.get_text_left_down(6, 0, 4), None);
        assert_eq!(grid.get_text_left_down(9, 0, 4), None);
    }

    #[test]
    fn test_get_text_left() {
        let grid = create_grid();
        assert_eq!(grid.get_text_left(0, 0, 4), None);
        assert_eq!(grid.get_text_left(0, 1, 4), None);
        assert_eq!(grid.get_text_left(0, 2, 4), None);
        assert_eq!(grid.get_text_left(0, 3, 4), Some("SMMM".to_string()));
        assert_eq!(grid.get_text_left(0, 4, 4), Some("XSMM".to_string()));
        assert_eq!(grid.get_text_left(0, 8, 4), Some("SAMX".to_string()));
        assert_eq!(grid.get_text_left(0, 9, 4), Some("MSAM".to_string()));
    }

    #[test]
    fn test_get_text_left_up() {
        let grid = create_grid();
        assert_eq!(grid.get_text_left_up(0, 0, 4), None);
        assert_eq!(grid.get_text_left_up(1, 1, 4), None);
        assert_eq!(grid.get_text_left_up(2, 2, 4), None);
        assert_eq!(grid.get_text_left_up(3, 3, 4), Some("MXSM".to_string()));
        assert_eq!(grid.get_text_left_up(3, 9, 4), Some("XMMM".to_string()));
        assert_eq!(grid.get_text_left_up(2, 9, 4), None);
        assert_eq!(grid.get_text_left_up(9, 0, 4), None);
        assert_eq!(grid.get_text_left_up(9, 1, 4), None);
        assert_eq!(grid.get_text_left_up(9, 2, 4), None);
        assert_eq!(grid.get_text_left_up(9, 3, 4), Some("XMAS".to_string()));
        assert_eq!(grid.get_text_left_up(9, 9, 4), Some("XMAS".to_string()));
    }

    #[test]
    fn test_get_text_up() {
        let grid = create_grid();
        assert_eq!(grid.get_text_up(0, 0, 4), None);
        assert_eq!(grid.get_text_up(1, 0, 4), None);
        assert_eq!(grid.get_text_up(2, 0, 4), None);
        assert_eq!(grid.get_text_up(3, 0, 4), Some("MAMM".to_string()));
        assert_eq!(grid.get_text_up(4, 0, 4), Some("XMAM".to_string()));
        assert_eq!(grid.get_text_up(8, 0, 4), Some("MSSX".to_string()));
        assert_eq!(grid.get_text_up(9, 0, 4), Some("MMSS".to_string()));
    }

    #[test]
    fn test_get_text_right_up() {
        let grid = create_grid();
        assert_eq!(grid.get_text_right_up(0, 0, 4), None);
        assert_eq!(grid.get_text_right_up(1, 0, 4), None);
        assert_eq!(grid.get_text_right_up(2, 0, 4), None);
        assert_eq!(grid.get_text_right_up(3, 0, 4), Some("MMAS".to_string()));
        assert_eq!(grid.get_text_right_up(3, 5, 4), Some("SAMS".to_string()));
        assert_eq!(grid.get_text_right_up(3, 6, 4), Some("MASM".to_string()));
        assert_eq!(grid.get_text_right_up(3, 7, 4), None);
        assert_eq!(grid.get_text_right_up(3, 8, 4), None);
        assert_eq!(grid.get_text_right_up(3, 9, 4), None);
        assert_eq!(grid.get_text_right_up(9, 5, 4), Some("XMAS".to_string()));
        assert_eq!(grid.get_text_right_up(9, 6, 4), Some("MMAS".to_string()));
        assert_eq!(grid.get_text_right_up(9, 7, 4), None);
        assert_eq!(grid.get_text_right_up(9, 8, 4), None);
        assert_eq!(grid.get_text_right_up(9, 9, 4), None);
    }

    #[test]
    fn test_spawn_possible_texts() {
        let grid = create_grid();

        assert_eq!(
            grid.spawn_possible_texts(3, 3, 4),
            vec![
                String::from("MASM"),
                String::from("MAXS"),
                String::from("MSMM"),
                String::from("MAXS"),
                String::from("MASM"),
                String::from("MXSM"),
                String::from("MSMS"),
                String::from("MXMM")
            ]
        );
    }

    #[test]
    fn test_find_word_count() {
        let grid = create_grid();
        assert_eq!(grid.find_word_count("XMAS"), 18);
    }
}
