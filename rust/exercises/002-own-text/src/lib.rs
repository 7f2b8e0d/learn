pub struct Note {
    text: String,
}

#[allow(unused_variables)]
pub fn store(name: &str) -> Note {
    Note {
        text: name.to_string(),
    }
}

pub fn read(note: &Note) -> &str {
    &note.text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_its_own_copy() {
        let mut original = String::from("ping");
        let note = store(&original);
        original.push('!');
        assert_eq!(original, "ping!");
        assert_eq!(read(&note), "ping");
    }

    #[test]
    fn stores_empty_text() {
        let note = store("");
        assert_eq!(read(&note), "");
    }
}
