pub fn lookup(name: &str) -> Option<u32> {
    match name {
        "alice" => Some(12),
        "bob" => Some(0),
        _ => None,
    }
}

pub fn doubled(name: &str) -> Option<u32> {
    let n = lookup(name)?;
    Some(n * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_alice() {
        assert_eq!(doubled("alice"), Some(24));
    }

    #[test]
    fn doubles_zero() {
        assert_eq!(doubled("bob"), Some(0));
    }

    #[test]
    fn missing_stays_missing() {
        assert_eq!(doubled("carol"), None);
    }
}
