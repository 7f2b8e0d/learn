use std::collections::HashMap;

#[allow(unused_variables)]
pub fn remember(book: &mut HashMap<String, u32>, name: &str, amount: u32) {
    book.insert(name.to_string(), amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_a_balance() {
        let mut book = HashMap::new();
        remember(&mut book, "alice", 10);
        assert_eq!(book.get("alice").copied(), Some(10));
    }

    #[test]
    fn later_amount_replaces_the_old_one() {
        let mut book = HashMap::new();
        remember(&mut book, "alice", 10);
        remember(&mut book, "alice", 3);
        assert_eq!(book.get("alice").copied(), Some(3));
    }

    #[test]
    fn missing_name_is_none() {
        let book: HashMap<String, u32> = HashMap::new();
        assert_eq!(book.get("bob").copied(), None);
    }
}
