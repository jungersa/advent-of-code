use crate::custom_error::AocError;

const NUMBER_RED_CUBES: u32 = 12;
const NUMBER_GREEN_CUBES: u32 = 13;
const NUMBER_BLUE_CUBES: u32 = 14;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if check_line(line) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum::<usize>()
        .to_string();
    Ok(output.to_string())
}

fn check_line(input: &str) -> bool {
    let game_line = input.split(':').nth(1).expect("There should 2 elements");

    game_line.split(";").all(|game| {
        let (number_red, number_green, number_blue) =
            game.split(",").fold((0, 0, 0), |acc, draw| {
                let parts_draw: Vec<&str> = draw.trim().split_whitespace().collect();
                let number: u32 = parts_draw[0].parse().unwrap_or(0);

                match parts_draw.get(1) {
                    Some(&"blue") => (acc.0, acc.1, acc.2 + number),
                    Some(&"red") => (acc.0 + number, acc.1, acc.2),
                    Some(&"green") => (acc.0, acc.1 + number, acc.2),
                    _ => acc,
                }
            });

        number_blue <= NUMBER_BLUE_CUBES
            && number_green <= NUMBER_GREEN_CUBES
            && number_red <= NUMBER_RED_CUBES
    })
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
