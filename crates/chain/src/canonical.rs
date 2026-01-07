use latte_types::transaction::Transaction;

pub trait CanonicalEncode {
    fn canonical_bytes(&self) -> Vec<u8>;
}

impl CanonicalEncode for Transaction {
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(self.from.0);

        match &self.to {
            Some(addr) => out.extend(addr.0),
            None => out.extend([0u8; 20]), // 约定 None = 20 个 0
        }

        out.extend(self.value.to_be_bytes());
        out.extend(self.nonce.to_be_bytes());
        out.extend(self.gas_limit.to_be_bytes());
        out.extend(self.gas_price.to_be_bytes());
        out.extend(&self.data);
        out.extend(&self.signature);

        out
    }
}
