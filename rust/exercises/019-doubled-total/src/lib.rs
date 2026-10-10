pub struct Ledger {
    fees: Vec<u32>,
}

impl Ledger {
    pub fn add(&mut self, fee: u32) {
        self.fees.push(fee);
    }

    #[allow(unused_variables)]
    pub fn doubled_total(&self) -> u32 {
        self.fees.iter().map(|fee| fee*2).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_then_adds() {
        let mut ledger = Ledger { fees: vec![] };
        ledger.add(1);
        ledger.add(2);
        assert_eq!(ledger.doubled_total(), 6);
    }

    #[test]
    fn empty_ledger_is_zero() {
        let ledger = Ledger { fees: vec![] };
        assert_eq!(ledger.doubled_total(), 0);
    }
}
