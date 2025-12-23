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

impl Transaction {
    pub fn hash(&self) -> Hash256 {
        // 使用bincode包将结构化数据用 二进制 格式序列化
        let bytes = bincode::serialize(self).expect("tx serialize");
        // 将交易进行hash256计算
        latte_primitives::hash::sha256(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn hash_tx() {
        // 在Cargo.toml引入primitives后，整个模块全局都直接使用primitives导出的类；不需要先use，可以通过路径直接使用
        let from_addr = Address::from_pubkey(&[1, 2, 3, 4, 5]);
        let to_addr = Address::from_pubkey(&[1, 2, 3, 4, 5]);

        let transaction = Transaction {
            from: from_addr,
            to: Option::Some(to_addr),
            value: 1,
            nonce: 1,
            gas_limit: 10,
            gas_price: 1,
            data: vec![1, 2, 3],
            signature: vec![1, 2, 3],
        };

        let hash_res = transaction.hash();
        assert_eq!(
            "7523ce1be16598af7559f0a0b4cd9aa40b4f75df6184e5e29daf735a19820f66",
            hex::encode(hash_res.0)
        );
    }
}
