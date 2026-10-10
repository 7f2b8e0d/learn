#[derive(Debug, PartialEq)]
pub enum Notice {
    Ready { count: u32 },
    Failed(&'static str),
    Waiting,
}

#[allow(unused_variables)]
pub fn text_of(notice: &Notice) -> String {
    match notice {
        Notice::Ready { count } => format!("ready {}", count),
        Notice::Failed(reason) => format!("failed: {}", reason),
        Notice::Waiting => "waiting".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_includes_count() {
        let notice = Notice::Ready { count: 2 };
        assert_eq!(text_of(&notice), "ready 2");
    }

    #[test]
    fn failed_includes_reason() {
        let notice = Notice::Failed("timeout");
        assert_eq!(text_of(&notice), "failed: timeout");
    }

    #[test]
    fn waiting_is_fixed_text() {
        assert_eq!(text_of(&Notice::Waiting), "waiting");
    }
}
