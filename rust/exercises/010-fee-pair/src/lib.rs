#[allow(unused_variables)]
pub fn pair(left: u32, right: u32) -> Vec<u32> {
    vec![left, right]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_order() {
        assert_eq!(pair(1, 2), vec![1, 2]);
    }

    #[test]
    fn keeps_zeros() {
        assert_eq!(pair(0, 0), vec![0, 0]);
    }
}
