use std::collections::HashSet;

/// Holds collection of values which are after specific value
#[derive(Default)]
pub struct LaterNumbers {
    internal: HashSet<usize>,
}
impl LaterNumbers {
    pub fn new(numbers: Vec<usize>) -> Self {
        Self {
            internal: numbers.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, number: usize) {
        self.internal.insert(number);
    }

    pub fn contains(&self, number: usize) -> bool {
        self.internal.contains(&number)
    }
}
