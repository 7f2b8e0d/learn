#[derive(Debug, PartialEq)]
pub enum Fault {
    Empty,
    NotANumber,
}

pub fn parse_count(text: &str) -> Result<u32, Fault> {
    if text.is_empty() {
        return Err(Fault::Empty);
    }
    match text.parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(Fault::NotANumber),
    }
}

#[allow(unused_variables)]
pub fn double_count(text: &str) -> Result<u32, Fault> {
    let n = parse_count(text)?;
    Ok(n * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_a_number() {
        assert_eq!(double_count("12"), Ok(24));
    }

    #[test]
    fn keeps_empty_error() {
        assert_eq!(double_count(""), Err(Fault::Empty));
    }

    #[test]
    fn keeps_word_error() {
        assert_eq!(double_count("nope"), Err(Fault::NotANumber));
    }
}
