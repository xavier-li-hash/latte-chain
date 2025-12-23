use crate::header::BlockHeader;
use crate::transaction::Transaction;
use serde::{Serialize, Deserialize};

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}


