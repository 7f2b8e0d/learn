// 在这里写 Pair、impl 和 same。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_numbers() {
        let pair = Pair { left: 1, right: 1 };
        assert!(pair.same());
    }

    #[test]
    fn different_numbers() {
        let pair = Pair { left: 1, right: 2 };
        assert!(!pair.same());
    }

    #[test]
    fn same_names() {
        let pair = Pair {
            left: "alice".to_string(),
            right: "alice".to_string(),
        };
        assert!(pair.same());
    }

    #[test]
    fn different_names() {
        let pair = Pair {
            left: "alice".to_string(),
            right: "bob".to_string(),
        };
        assert!(!pair.same());
    }
}
