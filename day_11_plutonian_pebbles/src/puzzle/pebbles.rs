use std::collections::HashMap;

// HashMap holds pebble number and its count
type PebblesCollection = HashMap<usize, usize>;

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

    fn blink(&self, blinks: usize) -> PebblesCollection {
        // Use HashMap to store type of pebbles and their count to handle large number of blinks
        let mut pebbles: PebblesCollection =
            self.internal.iter().map(|&pebble| (pebble, 1)).collect();

        for _ in 0..blinks {
            // Make a copy of pebbles so we are traversing original (not edited yet) collection
            let current = pebbles.clone();

            for (pebble, count) in current {
                Self::blink_pebble(&mut pebbles, pebble, count);
            }
        }

        pebbles
    }

    fn blink_pebble(pebbles: &mut PebblesCollection, pebble: usize, count: usize) {
        // Rule 1
        if pebble == 0 {
            Self::decrement_pebbles(pebbles, 0, count);
            Self::increment_pebbles(pebbles, 1, count);
            return;
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

            Self::decrement_pebbles(pebbles, pebble, count);
            Self::increment_pebbles(pebbles, left, count);
            Self::increment_pebbles(pebbles, right, count);
            return;
        }

        // Default rule
        Self::decrement_pebbles(pebbles, pebble, count);
        Self::increment_pebbles(pebbles, pebble * 2024, count);
    }

    fn decrement_pebbles(pebbles: &mut PebblesCollection, key: usize, count: usize) {
        pebbles
            .entry(key)
            .and_modify(|pebble_count| *pebble_count -= count);
    }

    fn increment_pebbles(pebbles: &mut PebblesCollection, key: usize, count: usize) {
        // Increment new value, if it does not exist yet, insert count directly as initial value
        pebbles
            .entry(key)
            .and_modify(|pebble_count| *pebble_count += count)
            .or_insert(count);
    }

    pub fn blink_stones_count(&self, blinks: usize) -> usize {
        // Collection contains also keys with 0 count, i.e. these numbers are not present
        self.blink(blinks)
            .into_iter()
            .filter_map(|(_, count)| match count {
                0 => None,
                _ => Some(count),
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_increment_and_decrement_pebbles() {
        let mut pebbles: PebblesCollection = [1, 2, 3].iter().map(|&pebble| (pebble, 1)).collect();

        assert_eq!(pebbles.get(&1), Some(&1));
        assert_eq!(pebbles.get(&2), Some(&1));
        assert_eq!(pebbles.get(&3), Some(&1));

        Pebbles::increment_pebbles(&mut pebbles, 1, 41);

        assert_eq!(pebbles.get(&1), Some(&42));
        assert_eq!(pebbles.get(&2), Some(&1));
        assert_eq!(pebbles.get(&3), Some(&1));

        Pebbles::decrement_pebbles(&mut pebbles, 1, 11);

        assert_eq!(pebbles.get(&1), Some(&31));
        assert_eq!(pebbles.get(&2), Some(&1));
        assert_eq!(pebbles.get(&3), Some(&1));

        Pebbles::decrement_pebbles(&mut pebbles, 2, 1);

        assert_eq!(pebbles.get(&1), Some(&31));
        assert_eq!(pebbles.get(&2), Some(&0));
        assert_eq!(pebbles.get(&3), Some(&1));

        Pebbles::increment_pebbles(&mut pebbles, 3, 1);

        assert_eq!(pebbles.get(&1), Some(&31));
        assert_eq!(pebbles.get(&2), Some(&0));
        assert_eq!(pebbles.get(&3), Some(&2));
    }

    // PebblesCollection is a HashMap and it can contain keys with 0 value. In expected vectors
    // we however have only values != 0, so we need to adapt collection to vector here.
    fn pebbles_as_vector_sorted(pebbles: PebblesCollection) -> Vec<usize> {
        let mut v = pebbles
            .into_iter()
            .filter_map(|(key, value)| match value {
                0 => None,
                _ => Some(key),
            })
            .collect::<Vec<_>>();

        v.sort();
        v
    }

    fn solution_as_vector_sorted(solution: Vec<usize>) -> Vec<usize> {
        let unique: HashSet<usize> = solution.iter().copied().collect();
        let mut v: Vec<usize> = unique.iter().copied().collect();

        v.sort();
        v
    }

    #[test]
    fn test_blink() {
        let pebbles = Pebbles::new(vec![125, 17]);

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(1)),
            solution_as_vector_sorted(vec![253000, 1, 7])
        );

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(2)),
            solution_as_vector_sorted(vec![253, 0, 2024, 14168])
        );

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(3)),
            solution_as_vector_sorted(vec![512072, 1, 20, 24, 28676032])
        );

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(4)),
            solution_as_vector_sorted(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032])
        );

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(5)),
            solution_as_vector_sorted(vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32
            ])
        );

        assert_eq!(
            pebbles_as_vector_sorted(pebbles.blink(6)),
            solution_as_vector_sorted(vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ])
        );
    }

    #[test]
    fn test_blink_stones_count() {
        let pebbles = Pebbles::new(vec![125, 17]);

        assert_eq!(pebbles.blink_stones_count(6), 22);
        assert_eq!(pebbles.blink_stones_count(25), 55312);
    }
}
