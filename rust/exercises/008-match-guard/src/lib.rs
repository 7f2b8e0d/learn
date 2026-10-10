#[allow(unused_variables)]
pub fn kind_of(balance: Option<u32>) -> &'static str {
    match balance {
        Some(n) if n == 0 => "zero",
        Some(n) => "positive",
        None => "missing"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_its_own_kind() {
        assert_eq!(kind_of(Some(0)), "zero");
    }

    #[test]
    fn other_numbers_are_positive() {
        assert_eq!(kind_of(Some(12)), "positive");
    }

    #[test]
    fn none_is_missing() {
        assert_eq!(kind_of(None), "missing");
    }
}
