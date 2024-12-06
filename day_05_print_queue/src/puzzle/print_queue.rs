use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct PrintQueue {
    // This represents graph (adjacency list)
    orders: HashMap<usize, HashSet<usize>>,

    // Pages to analyze
    pages: Vec<Vec<usize>>,
}

impl PrintQueue {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            pages: Vec::new(),
        }
    }

    pub fn insert_order(&mut self, first: usize, numbers: Vec<usize>) {
        let later_numbers = self.orders.entry(first).or_default();

        for number in numbers {
            later_numbers.insert(number);
        }
    }

    pub fn insert_page(&mut self, pages: Vec<Vec<usize>>) {
        for page in pages {
            self.pages.push(page);
        }
    }

    fn filter_pages_in_order(&self) -> Vec<&Vec<usize>> {
        self.pages
            .iter()
            .filter(|&page| self.is_page_in_order(page))
            .collect()
    }

    fn is_page_in_order(&self, page: &[usize]) -> bool {
        for index in 1..page.len() {
            if let Some(later_numbers) = self.orders.get(&page[index]) {
                // If 'previous_number' is in expected later_number we know the order is wrong
                if page
                    .iter()
                    .take(index)
                    .any(|previous_number| later_numbers.contains(previous_number))
                {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_middle_pages_in_order(&self) -> usize {
        self.filter_pages_in_order()
            .iter()
            .map(|page| {
                Self::get_page_middle(page)
                    .unwrap_or_else(|| panic!("Failed to find middle element in '{:?}'", page))
            })
            .sum()
    }

    fn get_page_middle(numbers: &[usize]) -> Option<usize> {
        match numbers.len() / 2 == 0 {
            true => None,
            false => Some(numbers[numbers.len() / 2]),
        }
    }

    pub fn count_middle_pages_in_only_fixed_order(&self) -> usize {
        self.pages
            .iter()
            .filter_map(|page| {
                // Count only pages which needs to be fixed
                match self.is_page_in_order(page) {
                    true => None,
                    false => {
                        let ordered = self.fix_order(page);
                        Some(Self::get_page_middle(&ordered).unwrap_or_else(|| {
                            panic!("Failed to find middle element in '{:?}'", ordered)
                        }))
                    }
                }
            })
            .sum()
    }

    fn fix_order(&self, page: &[usize]) -> Vec<usize> {
        let mut reordered = page.to_vec();
        let mut index = 1;

        while index < reordered.len() {
            let number = reordered[index];
            let dependencies = self.orders.get(&number);

            if dependencies.is_none() {
                index += 1;
                continue;
            }

            let dependencies = dependencies.expect("Failed to unwrap dependencies");

            // Go through previous numbers
            for prev_index in 0..index {
                // Check if number should be behind
                if dependencies.contains(&reordered[prev_index]) {
                    // Remove 'number from old position'
                    reordered.remove(index);

                    // Move 'number' to earlier position
                    reordered.insert(prev_index, number);

                    // Continue with next item after new 'number' position
                    index = prev_index; // Note: +1 is done at the end of while loop

                    // Break from for loop
                    break;
                }
            }

            // No move -> continue with next item
            index += 1;
        }

        reordered
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_print_queue() -> PrintQueue {
        let mut print_queue = PrintQueue::new();

        print_queue.insert_order(47, vec![53]);
        print_queue.insert_order(97, vec![13]);
        print_queue.insert_order(97, vec![61]);
        print_queue.insert_order(97, vec![47]);
        print_queue.insert_order(75, vec![29]);
        print_queue.insert_order(61, vec![13]);
        print_queue.insert_order(75, vec![53]);
        print_queue.insert_order(29, vec![13]);
        print_queue.insert_order(97, vec![29]);
        print_queue.insert_order(53, vec![29]);
        print_queue.insert_order(61, vec![53]);
        print_queue.insert_order(97, vec![53]);
        print_queue.insert_order(61, vec![29]);
        print_queue.insert_order(47, vec![13]);
        print_queue.insert_order(75, vec![47]);
        print_queue.insert_order(97, vec![75]);
        print_queue.insert_order(47, vec![61]);
        print_queue.insert_order(75, vec![61]);
        print_queue.insert_order(47, vec![29]);
        print_queue.insert_order(75, vec![13]);
        print_queue.insert_order(53, vec![13]);

        print_queue.insert_page(vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]);

        print_queue
    }

    #[test]
    fn test_is_page_in_order() {
        let print_queue = create_print_queue();

        assert!(print_queue.is_page_in_order(&[75, 47, 61, 53, 29]));
        assert!(print_queue.is_page_in_order(&[97, 61, 53, 29, 13]));
        assert!(print_queue.is_page_in_order(&[75, 29, 13]));
        assert!(!print_queue.is_page_in_order(&[75, 97, 47, 61, 53]));
        assert!(!print_queue.is_page_in_order(&[61, 13, 29]));
        assert!(!print_queue.is_page_in_order(&[97, 13, 75, 29, 47]));
    }

    #[test]
    fn test_count_middle_pages_in_order() {
        let print_queue = create_print_queue();
        assert_eq!(print_queue.count_middle_pages_in_order(), 143);
    }

    #[test]
    fn test_fix_order() {
        let print_queue = create_print_queue();

        assert_eq!(
            print_queue.fix_order(&[75, 47, 61, 53, 29]),
            vec![75, 47, 61, 53, 29]
        );

        assert_eq!(
            print_queue.fix_order(&[97, 61, 53, 29, 13]),
            vec![97, 61, 53, 29, 13]
        );

        assert_eq!(print_queue.fix_order(&[75, 29, 13]), vec![75, 29, 13]);

        assert_eq!(
            print_queue.fix_order(&[75, 97, 47, 61, 53]),
            vec![97, 75, 47, 61, 53]
        );

        assert_eq!(print_queue.fix_order(&[61, 13, 29]), vec![61, 29, 13]);

        assert_eq!(
            print_queue.fix_order(&[97, 13, 75, 29, 47]),
            vec![97, 75, 47, 29, 13]
        );
    }

    #[test]
    fn test_count_middle_pages_in_only_fixed_order() {
        let print_queue = create_print_queue();
        assert_eq!(print_queue.count_middle_pages_in_only_fixed_order(), 123);
    }
}
