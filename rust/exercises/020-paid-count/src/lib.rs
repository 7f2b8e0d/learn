pub struct Ledger {
    fees: Vec<u32>,
}

impl Ledger {
    pub fn add(&mut self, fee: u32) {
        self.fees.push(fee);
    }

    #[allow(unused_variables)]
    pub fn paid_count(&self) -> usize {
        self.fees.iter().filter(|fee| **fee > 0).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_fees_above_zero() {
        let mut ledger = Ledger { fees: vec![] };
        ledger.add(0);
        ledger.add(5);
        ledger.add(0);
        ledger.add(2);
        assert_eq!(ledger.paid_count(), 2);
    }

    #[test]
    fn zero_fees_count_as_none() {
        let mut ledger = Ledger { fees: vec![] };
        ledger.add(0);
        assert_eq!(ledger.paid_count(), 0);
    }

    #[test]
    fn empty_ledger_is_zero() {
        let ledger = Ledger { fees: vec![] };
        assert_eq!(ledger.paid_count(), 0);
    }
}
