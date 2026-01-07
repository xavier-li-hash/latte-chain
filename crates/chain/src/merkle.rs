use crate::canonical::CanonicalEncode;
use latte_primitives::hash;
use latte_primitives::hash::{Hash256, sha256};
use latte_types::transaction::Transaction;

///
/// 求一系列交易的默克尔树hash值
/// 未处理交易数为0的情况
///
pub fn tx_root_hash(transactions: &Vec<Transaction>) -> Hash256 {
    let hash_vec: Vec<Hash256> = transactions
        .iter()
        .map(|transaction| {
            return sha256(&transaction.canonical_bytes());
        })
        .collect();
    root_hash(hash_vec)
}

/// 求默克尔树hash值
pub fn root_hash(mut hashes: Vec<Hash256>) -> Hash256 {
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            // 解引用后，如果hash256实现了copy，可以在当前栈得到一份克隆的数据
            let last = *hashes.last().unwrap();
            hashes.push(last);
        }

        let mut next_level = Vec::new();

        for index in (0..hashes.len()).step_by(2) {
            let first = hashes.get(index).unwrap();
            let second = hashes.get(index + 1).unwrap();
            let merge_hash = hash::sha256(&[&first.0[..], &second.0[..]].concat());
            next_level.push(merge_hash);
        }

        hashes = next_level;
    }
    hashes.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merkle() {
        let h1: [u8; 4] = [1, 2, 3, 4];
        let h2: [u8; 4] = [20, 30, 40, 50];
        let hashes = vec![hash::sha256(&h1), hash::sha256(&h2)];
        let hash256 = root_hash(hashes);
        println!("{:?}", hash256);
    }
}
