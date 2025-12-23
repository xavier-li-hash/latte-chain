use serde::{Serialize, Deserialize};

///
/// 元组结构体 (Tuple Struct)
///
/// u8 代表无符号 8 位整数（即一个字节 Byte），即Unsigned 8-bit integer
/// [u8; 32] 代表一个长度固定为 32 个字节的数组。
/// 前面的 pub 表示结构体内部的这个数据字段也是公开的可访问。
///
/// $$32 \text{ Bytes} \times 8 \text{ bits/Byte} = 256 \text{ bits}$$
///
#[derive(Clone,Copy,Debug, PartialEq, Eq,Hash,  Serialize, Deserialize)]
pub struct Hash256(pub [u8;32]);


/// sha256工具函数
///
/// # Arguments
/// * `data` - 一个字节切片，代表需要处理的原始数据。
/// # Returns
///
/// 返回一个Hash256结构体，包含了经过sha256 hash计算的结果
pub fn sha256(data: &[u8]) -> Hash256 {
    // 1. 局部引入：只在这个函数内部引入 sha2 crate 的 trait 和结构体
    use sha2::{Digest};
    // 2. 初始化：创建一个新的 Sha256 生成器实例
    let mut hasher = sha2::Sha256::new();
    // 3. 输入数据：将传入的字节切片 data 喂给生成器，update 可以多次调用，适合处理大文件流
    hasher.update(data);
    // 4. 计算结果并转换：
    // - hasher.finalize()：完成计算，返回一个 GenericArray 类型的结果
    // - .into()：利用 Rust 的 From/Into 机制，将结果转为 [u8; 32] 数组
    // - Hash256(...)：用你自定义的元组结构体包裹这个数组并返回Ï
    Hash256(hasher.finalize().into())
}

/// blake3快速hash工具方法
///
/// # Arguments
/// * `data` - 一个字节切片，代表需要处理的原始数据。
/// # Returns
///
/// 返回一个Hash256结构体，包含了经过blake3::hash计算的结果
pub fn blake2s(data: &[u8]) -> Hash256 {
    // 这里的 blake3::hash 实际上是直接使用了包名作为路径；
    // 在 Cargo.toml 里添加了 blake3 依赖，那么 blake3 这个名字在整个项目的根层级都是可见的。
    Hash256(blake3::hash(data).into())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256() {
        let param: [u8; 4] = [1, 2, 3, 222];
        let hash_result = sha256(&param);

        println!("{:?}", hash_result);

        // 注意：hash_result.0 是访问元组结构体内部的 [u8; 32]
        assert_eq!(hash_result.0[0], 208);
        assert_eq!(hash_result.0[1], 248);

        // 或者直接与完整的预期数组比较
        let expected = [
            208, 248, 244, 16, 7, 69, 127, 26, 105, 104, 227, 224, 134, 164, 114, 37, 51, 198, 130, 164, 190, 76, 95, 84, 106, 3, 67, 92, 122, 5, 128, 250
        ];
        assert_eq!(hash_result.0, expected);
    }
}
