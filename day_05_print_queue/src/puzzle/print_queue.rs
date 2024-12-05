use std::collections::HashMap;

use super::{later_numbers::LaterNumbers, page::Page};

#[derive(Default)]
pub struct PrintQueue {
    // Holds mapping between 'first' number and 'later' number
    order: HashMap<usize, LaterNumbers>,

    // Pages to analyze
    pages: Vec<Page>,
}

impl PrintQueue {
    pub fn new() -> Self {
        Self {
            order: HashMap::new(),
            pages: Vec::new(),
        }
    }

    pub fn insert_order(&mut self, first: usize, numbers: Vec<usize>) {
        let later_numbers = self.order.entry(first).or_default();

        for number in numbers {
            later_numbers.insert(number);
        }
    }

    pub fn insert_page(&mut self, pages: Vec<Page>) {
        for page in pages {
            self.pages.push(page);
        }
    }

    fn filter_pages_in_correct_order(&self) -> Vec<&Page> {
        self.pages
            .iter()
            .filter(|&page| self.is_page_order_correct(page))
            .collect()
    }

    fn is_page_order_correct(&self, page: &Page) -> bool {
        for (index, current_number) in page.iter().enumerate() {
            // If we have defined later numbers check correctness
            if let Some(later_numbers) = self.order.get(current_number) {
                // If we can find in previous number any of "later_numbers" we have invalid order
                let previous_numbers = page.slice(index);

                if previous_numbers
                    .iter()
                    .any(|previous| later_numbers.contains(*previous))
                {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_middle_pages_in_correct_order(&self) -> usize {
        self.filter_pages_in_correct_order()
            .iter()
            .map(|page| {
                page.get_middle()
                    .unwrap_or_else(|| panic!("Failed to find middle element in '{:?}'", page))
            })
            .sum()
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
            Page::new(vec![75, 47, 61, 53, 29]),
            Page::new(vec![97, 61, 53, 29, 13]),
            Page::new(vec![75, 29, 13]),
            Page::new(vec![75, 97, 47, 61, 53]),
            Page::new(vec![61, 13, 29]),
            Page::new(vec![97, 13, 75, 29, 47]),
        ]);

        print_queue
    }

    #[test]
    fn test_is_page_order_correct() {
        let print_queue = create_print_queue();

        assert!(print_queue.is_page_order_correct(&Page::new(vec![75, 47, 61, 53, 29])));
        assert!(print_queue.is_page_order_correct(&Page::new(vec![97, 61, 53, 29, 13])));
        assert!(print_queue.is_page_order_correct(&Page::new(vec![75, 29, 13])));
        assert!(!print_queue.is_page_order_correct(&Page::new(vec![75, 97, 47, 61, 53])));
        assert!(!print_queue.is_page_order_correct(&Page::new(vec![61, 13, 29])));
        assert!(!print_queue.is_page_order_correct(&Page::new(vec![97, 13, 75, 29, 47])));
    }

    #[test]
    fn test_count_middle_pages_in_correct_order() {
        let print_queue = create_print_queue();

        assert_eq!(print_queue.count_middle_pages_in_correct_order(), 143);
    }
}
