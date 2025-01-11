#[derive(Debug, PartialEq, Default)]
pub struct Pebbles {
    internal: Vec<usize>,
}

impl Pebbles {
    pub fn new(pebbles: Vec<usize>) -> Self {
        Self {
            internal: pebbles.into_iter().collect(),
        }
    }

    fn blink(&self, blinks: usize) -> Vec<usize> {
        let mut pebbles = self.internal.clone();

        for _ in 0..blinks {
            pebbles = Self::blink_once(pebbles);
        }

        pebbles
    }

    fn blink_once(pebbles: Vec<usize>) -> Vec<usize> {
        // *2 is worst case after blink
        let mut blinked = Vec::with_capacity(pebbles.len() * 2);

        for pebble in pebbles {
            let mut blinked_pebble = Self::blink_pebble(pebble);
            blinked.append(&mut blinked_pebble);
        }

        blinked
    }

    fn blink_pebble(pebble: usize) -> Vec<usize> {
        // Rule 1
        if pebble == 0 {
            return vec![1];
        }

        // Rule 2
        let pebble_string = pebble.to_string();
        let pebble_len = pebble_string.len();
        if pebble_len % 2 == 0 {
            let middle = pebble_len / 2;
            let left = pebble_string[0..middle]
                .parse::<usize>()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to convert left '{}' to usize with an error '{}'",
                        &pebble_string[0..middle],
                        err
                    );
                });

            let right = pebble_string[middle..]
                .parse::<usize>()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to convert right '{}' to usize with an error '{}'",
                        &pebble_string[middle..],
                        err
                    );
                });

            return vec![left, right];
        }

        // Default rule
        vec![pebble * 2024]
    }

    pub fn blink_stones_count(&self, blinks: usize) -> usize {
        self.blink(blinks).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink_pebble() {
        assert_eq!(Pebbles::blink_pebble(0), vec![1]);
        assert_eq!(Pebbles::blink_pebble(1), vec![2024]);
        assert_eq!(Pebbles::blink_pebble(10), vec![1, 0]);
        assert_eq!(Pebbles::blink_pebble(99), vec![9, 9]);
        assert_eq!(Pebbles::blink_pebble(999), vec![2021976]);
        assert_eq!(Pebbles::blink_pebble(9876), vec![98, 76]);
    }

    #[test]
    fn test_blink_once() {
        assert_eq!(
            Pebbles::blink_once(vec![0, 1, 10, 99, 999]),
            vec![1, 2024, 1, 0, 9, 9, 2021976]
        );
    }

    #[test]
    fn test_blink() {
        let pebbles = Pebbles::new(vec![125, 17]);

        assert_eq!(pebbles.blink(1), vec![253000, 1, 7]);
        assert_eq!(pebbles.blink(2), vec![253, 0, 2024, 14168]);
        assert_eq!(pebbles.blink(3), vec![512072, 1, 20, 24, 28676032]);
        assert_eq!(
            pebbles.blink(4),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            pebbles.blink(5),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            pebbles.blink(6),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_blink_stones_count() {
        let pebbles = Pebbles::new(vec![125, 17]);

        assert_eq!(pebbles.blink_stones_count(6), 22);
        assert_eq!(pebbles.blink_stones_count(25), 55312);
    }
}
