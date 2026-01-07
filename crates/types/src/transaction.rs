use latte_primitives::{address::Address, hash::Hash256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Option<Address>,
    pub value: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
}
