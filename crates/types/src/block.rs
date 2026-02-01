use crate::header::BlockHeader;
use crate::transaction::Transaction;
use latte_codec::codec::Codec;
use latte_primitives::hash::Hash256;
use serde::{Serialize, Deserialize};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn hash_with<C: Codec>(&self, codec: &C) -> Result<Hash256, String> {
        self.header.hash_with(codec)
    }
}
