use std::collections::HashMap;

#[derive(Default)]
pub struct Designer {
    patterns: Vec<String>,
    words: Vec<String>,
}

impl Designer {
    pub fn new(patterns: Vec<String>, words: Vec<String>) -> Self {
        Self { patterns, words }
    }

    pub fn find_solutions(&self) -> usize {
        let mut cache = HashMap::new();
        let mut solutions = 0;

        for word in &self.words {
            let sol = self.count_solutions(word, &mut cache);
            if sol > 0 {
                solutions += 1;
            }
        }

        solutions
    }

    fn count_solutions(&self, word: &str, cache: &mut HashMap<String, usize>) -> usize {
        // Use cache to speedup
        if let Some(count) = cache.get(word) {
            return *count;
        }

        if word.is_empty() {
            return 1;
        }

        let mut solutions = 0;

        for pattern in &self.patterns {
            if word.starts_with(pattern) {
                solutions += self.count_solutions(&word[pattern.len()..], cache);
            }
        }

        // Update cache
        cache.insert(word.to_string(), solutions);
        solutions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_designer() -> Designer {
        Designer::new(
            vec![
                "r".to_string(),
                "wr".to_string(),
                "b".to_string(),
                "g".to_string(),
                "bwu".to_string(),
                "rb".to_string(),
                "gb".to_string(),
                "br".to_string(),
            ],
            vec![
                "brwrr".to_string(),
                "bggr".to_string(),
                "gbbr".to_string(),
                "rrbgbr".to_string(),
                "ubwu".to_string(),
                "bwurrg".to_string(),
                "brgr".to_string(),
                "bbrgwb".to_string(),
            ],
        )
    }

    #[test]
    fn test_count_possible_designs() {
        let designer = build_designer();

        assert_eq!(designer.find_solutions(), 6);
    }
}
