use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input.lines().map(process_line).sum::<u32>().to_string();
    Ok(output.to_string())
}

fn process_line(input: &str) -> u32 {
    input
        .split(':')
        .nth(1)
        .expect("There should be 2 elements")
        .split(';')
        .flat_map(|game| {
            game.split(',').map(|draw| {
                let parts_draw: Vec<&str> = draw.trim().split_whitespace().collect();
                let number: u32 = parts_draw[0].parse().unwrap_or(0);

                match parts_draw.get(1) {
                    Some(&"red") => [number, 0, 0],
                    Some(&"green") => [0, number, 0],
                    Some(&"blue") => [0, 0, number],
                    _ => [0, 0, 0],
                }
            })
        })
        .fold([0; 3], |acc, color_counts| {
            [
                acc[0].max(color_counts[0]),
                acc[1].max(color_counts[1]),
                acc[2].max(color_counts[2]),
            ]
        })
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
