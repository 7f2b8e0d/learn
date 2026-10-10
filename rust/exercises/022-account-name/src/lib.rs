pub trait Named {
    fn name(&self) -> &str;
}

pub struct Account {
    name: String,
}

impl Named for Account {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_the_account_name() {
        let account = Account {
            name: "alice".to_string(),
        };
        assert_eq!(account.name(), "alice");
    }

    #[test]
    fn returns_an_empty_name() {
        let account = Account {
            name: String::new(),
        };
        assert_eq!(account.name(), "");
    }
}
