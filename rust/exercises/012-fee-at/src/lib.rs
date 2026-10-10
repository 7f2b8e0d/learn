#[allow(unused_variables)]
pub fn fee_at(fees: Vec<u32>, index: usize) -> Option<u32> {
    fees.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_first() {
        assert_eq!(fee_at(vec![10, 20], 0), Some(10));
    }

    #[test]
    fn reads_the_second() {
        assert_eq!(fee_at(vec![10, 20], 1), Some(20));
    }

    #[test]
    fn missing_index_is_none() {
        assert_eq!(fee_at(vec![10], 3), None);
    }
}
