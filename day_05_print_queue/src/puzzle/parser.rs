use advent_of_code::puzzles::puzzle_error::PuzzleError;

use super::print_queue::PrintQueue;

enum ParserState {
    Order,
    Page,
}

pub struct Parser {}

impl Parser {
    pub fn parse_lines(lines: &[&str]) -> Result<PrintQueue, PuzzleError> {
        let mut print_queue = PrintQueue::new();
        let mut parser_state = ParserState::Order;

        for line in lines {
            // Empty line separates 'orders' and 'pages'
            if line.is_empty() {
                parser_state = ParserState::Page;
                continue;
            }

            match parser_state {
                ParserState::Order => {
                    let (first, second) = Self::decode_order(line)?;
                    print_queue.insert_order(first, vec![second]);
                }
                ParserState::Page => {
                    let page = Self::decode_page(line)?;
                    print_queue.insert_page(vec![page]);
                }
            }
        }

        Ok(print_queue)
    }

    fn decode_order(line: &str) -> Result<(usize, usize), PuzzleError> {
        let splitted = line.split_terminator("|").collect::<Vec<_>>();

        if splitted.len() != 2 {
            return Err(PuzzleError::InvalidContentError(format!(
                "Order should contain 2 elements but {} were found",
                splitted.len(),
            )));
        }

        let first = splitted[0].parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{}' to usize with an error '{}'",
                splitted[0], err
            ))
        })?;

        let second = splitted[1].parse::<usize>().map_err(|err| {
            PuzzleError::InvalidContentError(format!(
                "Failed to convert '{}' to usize with an error '{}'",
                splitted[1], err
            ))
        })?;

        Ok((first, second))
    }

    fn decode_page(line: &str) -> Result<Vec<usize>, PuzzleError> {
        let splitted = line.split_terminator(",").collect::<Vec<_>>();

        if splitted.is_empty() {
            return Err(PuzzleError::InvalidContentError(format!(
                "Page should contain at least 1 element but {} were found",
                splitted.len(),
            )));
        }

        let mut pages = Vec::new();

        for split in splitted {
            let number = split.parse::<usize>().map_err(|err| {
                PuzzleError::InvalidContentError(format!(
                    "Failed to convert '{}' to usize with an error '{}'",
                    split, err
                ))
            })?;

            pages.push(number);
        }

        Ok(pages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_order() {
        let result = Parser::decode_order("42|24");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), (42, 24));
    }

    #[test]
    fn test_decode_page() {
        let result = Parser::decode_page("1,2,3");

        assert!(result.is_ok(), "Result: {:?}", result);
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }
}
