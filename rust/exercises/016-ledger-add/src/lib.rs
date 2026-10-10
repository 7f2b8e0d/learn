pub struct Ledger {
    #[allow(dead_code)]
    fees: Vec<u32>,
}

impl Ledger {
    #[allow(unused_variables)]
    pub fn add(&mut self, fee: u32) {
        self.fees.push(fee);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_onto_the_ledger() {
        let mut ledger = Ledger { fees: vec![1] };
        ledger.add(2);
        assert_eq!(ledger.fees, vec![1, 2]);
    }

    #[test]
    fn adds_to_an_empty_ledger() {
        let mut ledger = Ledger { fees: vec![] };
        ledger.add(0);
        assert_eq!(ledger.fees, vec![0]);
    }
}
