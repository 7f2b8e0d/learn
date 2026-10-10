pub struct Ledger {
    fees: Vec<u32>,
}

impl Ledger {
    pub fn add(&mut self, fee: u32) {
        self.fees.push(fee);
    }

    #[allow(unused_variables)]
    pub fn total(&self) -> u32 {
        let mut sum = 0;
        for fee in &self.fees {
            sum += fee;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_added_fees() {
        let mut ledger = Ledger { fees: vec![] };
        ledger.add(1);
        ledger.add(2);
        assert_eq!(ledger.total(), 3);
    }

    #[test]
    fn empty_ledger_is_zero() {
        let ledger = Ledger { fees: vec![] };
        assert_eq!(ledger.total(), 0);
    }
}
