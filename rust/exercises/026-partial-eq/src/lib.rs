// 在这里写 same。

#[cfg(test)]
mod tests {
    use super::*;

    struct Label(&'static str);

    impl PartialEq for Label {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

    #[test]
    fn same_numbers() {
        assert!(same(&1, &1));
    }

    #[test]
    fn different_numbers() {
        assert!(!same(&1, &2));
    }

    #[test]
    fn same_labels() {
        assert!(same(&Label("alice"), &Label("alice")));
    }

    #[test]
    fn different_labels() {
        assert!(!same(&Label("alice"), &Label("bob")));
    }
}
