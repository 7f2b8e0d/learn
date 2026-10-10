#[derive(Debug, PartialEq)]
pub enum Fault {
    Empty,
    NotANumber,
}

#[allow(unused_variables)]
pub fn parse_count(text: &str) -> Result<u32, Fault> {
    if text.is_empty() {
        return Err(Fault::Empty);
    }
    match text.parse::<u32>() {
        Ok(number) => Ok(number),
        _ => Err(Fault::NotANumber)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_number() {
        assert_eq!(parse_count("12"), Ok(12));
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(parse_count(""), Err(Fault::Empty));
    }

    #[test]
    fn rejects_words() {
        assert_eq!(parse_count("nope"), Err(Fault::NotANumber));
    }
}
