#[allow(unused_variables)]
pub fn amount_or_zero(balance: Option<u32>) -> u32 {
    match balance {
        Some(n) => n,
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_returns_the_number() {
        assert_eq!(amount_or_zero(Some(12)), 12);
    }

    #[test]
    fn some_zero_stays_zero() {
        assert_eq!(amount_or_zero(Some(0)), 0);
    }

    #[test]
    fn none_returns_zero() {
        assert_eq!(amount_or_zero(None), 0);
    }
}
