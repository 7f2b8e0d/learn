pub struct Pair<T> {
    left: T,
    #[allow(dead_code)]
    right: T,
}

impl<T> Pair<T> {
    pub fn left(&self) -> &T {
        &self.left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_left_number() {
        let pair = Pair { left: 1, right: 2 };
        assert_eq!(pair.left(), &1);
    }

    #[test]
    fn returns_the_left_name() {
        let pair = Pair {
            left: "alice".to_string(),
            right: "bob".to_string(),
        };
        assert_eq!(pair.left(), "alice");
    }
}
