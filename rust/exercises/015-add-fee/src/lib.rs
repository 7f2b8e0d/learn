#[allow(unused_variables)]
pub fn add_fee(fees: &mut Vec<u32>, fee: u32) {
    fees.push(fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_on_the_callers_list() {
        let mut fees = vec![1];
        add_fee(&mut fees, 2);
        assert_eq!(fees, vec![1, 2]);
    }

    #[test]
    fn appends_to_an_empty_list() {
        let mut fees = vec![];
        add_fee(&mut fees, 0);
        assert_eq!(fees, vec![0]);
    }
}
