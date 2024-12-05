#[derive(Debug, PartialEq)]
pub struct Page {
    internal: Vec<usize>,
}

impl Page {
    pub fn new(pages: Vec<usize>) -> Self {
        Self { internal: pages }
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.internal.iter()
    }

    pub fn slice(&self, end: usize) -> &[usize] {
        &self.internal[..end]
    }

    pub fn get_middle(&self) -> Option<&usize> {
        match self.internal.len() % 2 == 0 {
            true => None,
            false => Some(&self.internal[self.internal.len() / 2]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_middle() {
        assert_eq!(Page::new(vec![]).get_middle(), None);
        assert_eq!(Page::new(vec![1]).get_middle(), Some(&1));
        assert_eq!(Page::new(vec![1, 2]).get_middle(), None);
        assert_eq!(Page::new(vec![1, 2, 3]).get_middle(), Some(&2));
    }
}
