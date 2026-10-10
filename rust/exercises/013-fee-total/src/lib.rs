#[allow(unused_variables)]
pub fn total(fees: Vec<u32>) -> u32 {
    let mut sum = 0;
    for item in fees {
        sum += item;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_every_fee() {
        assert_eq!(total(vec![1, 2, 3]), 6);
    }

    #[test]
    fn empty_list_is_zero() {
        assert_eq!(total(vec![]), 0);
    }

    #[test]
    fn zero_does_not_change_the_sum() {
        assert_eq!(total(vec![0, 5]), 5);
    }
}
