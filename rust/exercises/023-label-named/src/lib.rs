pub trait Named {
    fn name(&self) -> &str;
}

pub struct Account {
    name: String,
}

pub struct Contract {
    name: String,
}

impl Named for Account {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Named for Contract {
    fn name(&self) -> &str {
        &self.name
    }
}

#[allow(unused_variables)]
pub fn label(item: &impl Named) -> &str {
    &item.name()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_an_account() {
        let account = Account {
            name: "alice".to_string(),
        };
        assert_eq!(label(&account), "alice");
    }

    #[test]
    fn labels_a_contract() {
        let contract = Contract {
            name: "vault".to_string(),
        };
        assert_eq!(label(&contract), "vault");
    }

    #[test]
    fn labels_an_empty_name() {
        let account = Account {
            name: String::new(),
        };
        assert_eq!(label(&account), "");
    }
}
