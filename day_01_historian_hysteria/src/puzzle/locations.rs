pub type Location = usize;

#[derive(Debug, PartialEq, Default)]
pub struct Locations {
    left: Vec<Location>,
    right: Vec<Location>,
}

impl Locations {
    pub fn new(left: Vec<Location>, right: Vec<Location>) -> Self {
        if left.len() != right.len() {
            panic!(
                "Length of lists differs, left: {}, right: {}",
                left.len(),
                right.len()
            );
        }

        // Sort collections
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
        self.left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum()
    }

    pub fn get_similarity_score(&self) -> usize {
        self.left
            .iter()
            .map(|l| {
                let right_occurrences = self.right.iter().filter(|&r| l == r).count();
                l * right_occurrences
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_locations() -> Locations {
        Locations::new(vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    }

    #[test]
    fn test_get_total_distance() {
        let locations = create_locations();
        assert_eq!(locations.get_total_distance(), 11);
    }

    #[test]
    fn test_get_similarity_score() {
        let locations = create_locations();
        assert_eq!(locations.get_similarity_score(), 31);
    }
}
