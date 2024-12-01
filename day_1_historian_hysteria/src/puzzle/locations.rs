pub type Location = usize;

#[derive(Debug, PartialEq, Default)]
pub struct Locations {
    left: Vec<Location>,
    right: Vec<Location>,
}

impl Locations {
    pub fn new(left: Vec<Location>, right: Vec<Location>) -> Self {
        let mut left = left;
        left.sort();

        let mut right = right;
        right.sort();

        Self { left, right }
    }

    pub fn get_left(&self) -> &Vec<Location> {
        &self.left
    }

    pub fn get_right(&self) -> &Vec<Location> {
        &self.right
    }

    pub fn get_total_distance(&self) -> usize {
        // Note: Collections are already sorted
        if self.left.len() != self.right.len() {
            panic!(
                "Length of lists differs, left: {}, right: {}",
                self.left.len(),
                self.right.len()
            );
        }

        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_distance() {
        let locations = Locations::new(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);
        assert_eq!(locations.get_total_distance(), 11);
    }
}
