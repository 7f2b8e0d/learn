pub fn balance_text(balance: Option<u32>) -> String {
    match balance {
        Some(n) => format!("balance {n}"),
        None => "missing".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_includes_the_number() {
        assert_eq!(balance_text(Some(12)), "balance 12");
    }

    #[test]
    fn some_zero_is_still_present() {
        assert_eq!(balance_text(Some(0)), "balance 0");
    }

    #[test]
    fn none_is_missing() {
        assert_eq!(balance_text(None), "missing");
    }
}
