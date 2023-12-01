use tracing_subscriber::fmt::format;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output = input.lines().map(process_line).sum::<u32>();
    Ok(output.to_string())
}

fn process_line(line: &str) -> u32 {
    // let first_digit = line
    //     .char_indices()
    //     .find_map(|(i, character)| match character.is_digit(10) {
    //         true => Some(character),
    //         false => match &line[i..] {
    //             s if s.starts_with("one") => Some('1'),
    //             s if s.starts_with("two") => Some('2'),
    //             s if s.starts_with("three") => Some('3'),
    //             s if s.starts_with("four") => Some('4'),
    //             s if s.starts_with("five") => Some('5'),
    //             s if s.starts_with("six") => Some('6'),
    //             s if s.starts_with("seven") => Some('7'),
    //             s if s.starts_with("eight") => Some('8'),
    //             s if s.starts_with("nine") => Some('9'),
    //             _ => None,
    //         },
    //     })
    //     .expect("There should be a number");

    // let last_digit = line
    //     .char_indices()
    //     .rev()
    //     .find_map(|(i, character)| match character.is_digit(10) {
    //         true => Some(character),
    //         false => match &line[..i + 1] {
    //             s if s.ends_with("one") => Some('1'),
    //             s if s.ends_with("two") => Some('2'),
    //             s if s.ends_with("three") => Some('3'),
    //             s if s.ends_with("four") => Some('4'),
    //             s if s.ends_with("five") => Some('5'),
    //             s if s.ends_with("six") => Some('6'),
    //             s if s.ends_with("seven") => Some('7'),
    //             s if s.ends_with("eight") => Some('8'),
    //             s if s.ends_with("nine") => Some('9'),
    //             _ => None,
    //         },
    //     })
    //     .expect("There should be a number");

    // Version 2
    let mut first_digit = 0;

    for i in 0..line.len() {
        let f_line = &line[i..];

        let res_first = match f_line {
            s if s.starts_with("one") => '1',
            s if s.starts_with("two") => '2',
            s if s.starts_with("three") => '3',
            s if s.starts_with("four") => '4',
            s if s.starts_with("five") => '5',
            s if s.starts_with("six") => '6',
            s if s.starts_with("seven") => '7',
            s if s.starts_with("eight") => '8',
            s if s.starts_with("nine") => '9',
            _ => f_line.chars().next().unwrap(),
        };

        match res_first.to_digit(10) {
            Some(digit) => {
                first_digit = digit;
                break;
            }
            None => (),
        }
    }

    let mut last_digit = 0;

    for i in (0..line.len() + 1).rev() {
        let b_line = &line[..i];

        let res_last = match b_line {
            s if s.ends_with("one") => '1',
            s if s.ends_with("two") => '2',
            s if s.ends_with("three") => '3',
            s if s.ends_with("four") => '4',
            s if s.ends_with("five") => '5',
            s if s.ends_with("six") => '6',
            s if s.ends_with("seven") => '7',
            s if s.ends_with("eight") => '8',
            s if s.ends_with("nine") => '9',
            _ => b_line.chars().last().unwrap(),
        };

        match res_last.to_digit(10) {
            Some(digit) => {
                last_digit = digit;
                break;
            }
            None => (),
        };
    }

    // Version 1
    // let mut it = (0..line.len()).filter_map(|i| {
    //     let c_line = &line[i..];

    //     let res = match c_line {
    //         s if s.starts_with("one") => '1',
    //         s if s.starts_with("two") => '2',
    //         s if s.starts_with("three") => '3',
    //         s if s.starts_with("four") => '4',
    //         s if s.starts_with("five") => '5',
    //         s if s.starts_with("six") => '6',
    //         s if s.starts_with("seven") => '7',
    //         s if s.starts_with("eight") => '8',
    //         s if s.starts_with("nine") => '9',
    //         _ => c_line.chars().next().unwrap(),
    //     };
    //     res.to_digit(10)
    // });

    // let first_digit = it.next().expect("should have a first digit");
    // let last_digit = it.last().unwrap_or(first_digit);

    format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .expect("should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!("281", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_process_line(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(input));
    }
}
