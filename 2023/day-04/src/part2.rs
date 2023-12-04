use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input.lines().map(process_line);

    let mut temp = vec![1; output.clone().count()];

    let output = output.enumerate().fold(0, |acc, (idx, number_wins)| {
        let number_card = temp.get(idx).cloned().unwrap_or(0);
        for i in idx..idx + number_wins {
            temp[i + 1] += number_card;
        }

        acc + temp.get(idx).unwrap_or(&0)
    });

    println!("{:?}", temp);

    Ok(output.to_string())
}

fn process_line(input: &str) -> usize {
    let game = input.split(":").nth(1).unwrap();
    let mut split = game.split("|");
    let wining_numbers = split.next().unwrap();
    let game_numbers = split.next().unwrap();

    let wining_numbers: Vec<usize> = wining_numbers
        .trim()
        .split_whitespace()
        .map(|str_number| str_number.parse::<usize>().unwrap())
        .collect();

    let game_numbers: Vec<usize> = game_numbers
        .trim()
        .split_whitespace()
        .map(|str_number| str_number.parse::<usize>().unwrap())
        .collect();

    let number_hit = game_numbers.into_iter().fold(0, |mut acc, number| {
        if wining_numbers.contains(&number) {
            acc += 1;
        }
        acc
    });

    number_hit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}
