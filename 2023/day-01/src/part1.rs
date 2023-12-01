use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: i32 = input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .expect("no digit found");
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_ascii_digit())
                .expect("no digit found");
            format!("{}{}", first, last)
                .parse::<i32>()
                .expect("not a number")
        })
        .sum();
    Ok(output.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
