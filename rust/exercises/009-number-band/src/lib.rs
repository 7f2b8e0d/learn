#[allow(unused_variables)]
pub fn band_of(n: u32) -> &'static str {
    match n {
        n if n < 10 => "low",
        _ => "high"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_low() {
        assert_eq!(band_of(0), "low");
    }

    #[test]
    fn nine_is_low() {
        assert_eq!(band_of(9), "low");
    }

    #[test]
    fn ten_is_high() {
        assert_eq!(band_of(10), "high");
    }

    #[test]
    fn twelve_is_high() {
        assert_eq!(band_of(12), "high");
    }
}
