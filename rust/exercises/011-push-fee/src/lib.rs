#[allow(unused_variables)]
pub fn with_fee(mut fees: Vec<u32>, fee: u32) -> Vec<u32> {
    fees.push(fee);
    fees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_at_the_end() {
        assert_eq!(with_fee(vec![1], 2), vec![1, 2]);
    }

    #[test]
    fn appends_to_an_empty_list() {
        assert_eq!(with_fee(vec![], 0), vec![0]);
    }
}
