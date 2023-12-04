use std::cell::RefCell;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct PartNumber {
    number: u32,
    x: usize,
    y: usize,
    length: usize,
    symbol: RefCell<Option<Symbol>>,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    x: usize,
    y: usize,
}

impl PartNumber {
    fn is_valid(&self, schema: &[&str], valid_set: &[char]) -> bool {
        let get_char_at = |x: usize, y: usize| schema.get(x).and_then(|line| line.chars().nth(y));

        let check_neighbor = |x, y| {
            if let Some(c) = get_char_at(x, y) {
                if valid_set.contains(&c) {
                    self.symbol.replace(Some(Symbol { x, y }));
                }
                valid_set.contains(&c)
            } else {
                false
            }
        };

        let check_vertical_neighbors =
            |x, y: usize, length: usize| (y..y + length + 2).any(|yy| check_neighbor(x, yy));

        let y = self.y.saturating_sub(1);

        check_neighbor(self.x, y)
            || check_neighbor(self.x, self.y + self.length)
            || check_vertical_neighbors(self.x.saturating_sub(1), y, self.length)
            || check_vertical_neighbors(self.x + 1, y, self.length)
    }
}

fn build_part_number(input: &str) -> Vec<PartNumber> {
    input
        .lines()
        .enumerate()
        .fold(Vec::new(), |mut acc, (idx, line)| {
            let mut current_number: PartNumber = PartNumber {
                number: 0,
                x: idx,
                y: 0,
                length: 0,
                symbol: RefCell::new(None),
            };
            for (idy, c) in line.chars().enumerate() {
                if let Some(number) = c.to_digit(10) {
                    if current_number.number == 0 {
                        current_number.y = idy;
                        current_number.number = number;
                    } else {
                        current_number.number = 10 * current_number.number + number;
                    }
                } else if current_number.number != 0 {
                    current_number.length = idy - current_number.y;
                    acc.push(current_number);
                    current_number = PartNumber {
                        number: 0,
                        x: idx,
                        y: 0,
                        length: 0,
                        symbol: RefCell::new(None),
                    };
                }
            }
            if current_number.number != 0 {
                current_number.length = line.len() - current_number.y;
                acc.push(current_number);
            }
            acc
        })
}

fn validate_gear(number: &PartNumber, valid_numbers: &[PartNumber]) -> Option<u32> {
    let symbol = number.symbol.borrow();
    let (x_symbol, y_symbol) = (symbol.as_ref()?.x, symbol.as_ref()?.y);

    let linked_numbers: Vec<&PartNumber> = valid_numbers
        .iter()
        .filter(|n| {
            let s = n.symbol.borrow();
            s.as_ref().unwrap().x == x_symbol && s.as_ref().unwrap().y == y_symbol
        })
        .collect();

    if linked_numbers.len() == 2 {
        Some(linked_numbers.iter().fold(1, |acc, item| item.number * acc))
    } else {
        None
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let valid_set: Vec<char> = vec!['*'];

    let mut numbers = build_part_number(input);

    let input_lines: Vec<&str> = input.lines().collect();

    let valid_numbers: Vec<PartNumber> = numbers
        .into_iter()
        .filter(|number| number.is_valid(&input_lines, &valid_set))
        .collect();

    let valid_gear: Vec<u32> = valid_numbers
        .iter()
        .filter_map(|number| validate_gear(number, &valid_numbers))
        .collect();

    let sum: u32 = valid_gear.iter().sum();
    let sum = sum / 2;
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
