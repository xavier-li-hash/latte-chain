use crate::hash::Hash256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Address(pub [u8; 20]);

/// 从公钥生成地址
/// 流程：公钥 -> BLAKE3 哈希 (32字节) -> 截取前 20 字节 -> 地址
impl Address {
    pub fn from_pubkey(pubkey: &[u8]) -> Self {
        // 1. 使用 BLAKE3 算法对原始公钥进行哈希运算。
        // BLAKE3 比 SHA-256 更快且具有同等的安全性。
        // .into() 将 blake3::Hash 类型转换为原生的 [u8; 32] 数组。
        let raw_bytes: [u8; 32] = blake3::hash(pubkey).into();

        // 2. 将原始字节包装进自定义的 Hash256 结构体中。
        let hash = Hash256(raw_bytes);

        // 3. 初始化一个长度为 20 字节的零数组，用于存放最终地址。
        let mut addr = [0u8; 20];

        // 4. 从 32 字节的哈希值中截取前 20 字节 (索引 0 到 19)。
        // 这是一种常见的降维做法，旨在平衡安全性和存储效率。
        addr.copy_from_slice(&hash.0[0..20]);

        // 5. 将这 20 字节封装进 Address 结构体并返回。
        Address(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_address() {
        let address = Address::from_pubkey(&[1; 20]);
        let expected = [
            32, 235, 88, 148, 153, 41, 86, 52, 183, 56, 113, 19, 251, 165, 110, 215, 178, 26, 169,
            142,
        ];
        assert_eq!(address.0, expected);
    }
}
