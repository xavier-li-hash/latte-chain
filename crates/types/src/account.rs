use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Account {
    pub nonce: u64,
    pub balance: u128,
    pub storage: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Account {
    pub fn empty() -> Self {
        Account{
            nonce:0,
            balance:0,
            storage: BTreeMap::new(),
        }
    }
}
