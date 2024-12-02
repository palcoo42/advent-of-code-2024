#[derive(Debug, PartialEq)]
pub struct Report {
    internal: Vec<usize>,
}

impl Report {
    pub fn new(numbers: Vec<usize>) -> Self {
        Self { internal: numbers }
    }

    pub fn is_safe(&self) -> bool {
        match self.internal[0].cmp(&self.internal[1]) {
            std::cmp::Ordering::Equal => false,
            std::cmp::Ordering::Less => Self::compare_less(&self.internal),
            std::cmp::Ordering::Greater => {
                Self::compare_less(&self.internal.iter().rev().copied().collect::<Vec<_>>())
            }
        }
    }

    fn compare_less(numbers: &[usize]) -> bool {
        // All have to be ascending with diff <1; 3>
        for i in 0..numbers.len() - 1 {
            if numbers[i] >= numbers[i + 1] || numbers[i + 1] - numbers[i] > 3 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_safe() {
        assert!(Report::new(vec![7, 6, 4, 2, 1]).is_safe());
        assert!(!Report::new(vec![1, 2, 7, 8, 9]).is_safe());
        assert!(!Report::new(vec![9, 7, 6, 2, 1]).is_safe());
        assert!(!Report::new(vec![1, 3, 2, 4, 5]).is_safe());
        assert!(!Report::new(vec![8, 6, 4, 4, 1]).is_safe());
        assert!(Report::new(vec![1, 3, 6, 7, 9]).is_safe());
    }
}
