#[allow(unused_variables)]
pub fn total(fees: &[u32]) -> u32 {
    let mut sum = 0;
    for fee in fees {
        sum += *fee;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_every_fee() {
        assert_eq!(total(&[1, 2, 3]), 6);
    }

    #[test]
    fn empty_list_is_zero() {
        assert_eq!(total(&[]), 0);
    }

    #[test]
    fn caller_keeps_the_list() {
        let fees = vec![1, 2];
        assert_eq!(total(&fees), 3);
        assert_eq!(fees, vec![1, 2]);
    }
}
