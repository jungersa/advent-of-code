use crate::custom_error::AocError;

#[derive(Debug)]
struct PartNumber {
    number: u32,
    x: usize,
    y: usize,
    length: usize,
}

impl PartNumber {
    fn is_valid(&self, schema: &[&str], valid_set: &[char]) -> bool {
        let get_char_at = |x: usize, y: usize| schema.get(x).and_then(|line| line.chars().nth(y));

        let check_neighbor = |x, y| get_char_at(x, y).map_or(false, |c| !valid_set.contains(&c));

        let check_vertical_neighbors = |x, y: usize, length| {
            (y..y + length + 2)
                // .flat_map(|yy| get_char_at(x, yy))
                .any(|yy| check_neighbor(x, yy))
        };
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
            };
            for (idy, c) in line.chars().enumerate() {
                if let Some(number) = c.to_digit(10) {
                    if current_number.number == 0 {
                        current_number.y = idy;
                        current_number.number = number;
                    } else {
                        current_number.number = (10 * current_number.number) + number;
                    }
                } else if current_number.number != 0 {
                    current_number.length = idy - current_number.y;
                    acc.push(current_number);
                    current_number = PartNumber {
                        number: 0,
                        x: idx,
                        y: 0,
                        length: 0,
                    }
                }
            }
            if current_number.number != 0 {
                current_number.length = line.len() - current_number.y;
                acc.push(current_number);
            }
            acc
        })
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let valid_set: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

    let numbers = build_part_number(input);

    let input_lines: Vec<&str> = input.lines().collect();

    let valid_numbers: Vec<&PartNumber> = numbers
        .iter()
        .filter(|number| number.is_valid(&input_lines, &valid_set))
        .collect();

    let valid_number: Vec<u32> = valid_numbers.iter().map(|value| value.number).collect();
    // println!("{:?}", valid_number);

    let sum: u32 = valid_number.iter().sum();
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
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
