use latte_primitives::bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Receipt {
    pub transaction_hash: Bytes,
    pub status: Bytes,
    pub gas_used: u64,
    pub logs: Vec<Bytes>,
}

impl Receipt {
    pub fn new(transaction_hash: Bytes, status: Bytes, gas_used: u64, logs: Vec<Bytes>) -> Self {
        Receipt{
            transaction_hash,
            status,
            gas_used,
            logs
        }
    }
}