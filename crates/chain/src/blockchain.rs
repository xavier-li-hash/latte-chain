use crate::storage::BlockStorage;
use crate::error::ChainError;
use latte_primitives::hash::Hash256;
use latte_types::block::Block;
use std::collections::HashMap;
use std::sync::Arc;

/// 说明：后续添加存储实现时，支持从缓存中如果取不到，从本地存储加载区块，比如rockDB，内存只缓存一部分区块
pub struct Blockchain {
    /// 指向当前主链的最顶端
    head: Hash256,
    /// 当前主链的高度
    height: u64,
    /// 内存中的热数据缓存 (后续替代为 LruCache)
    blocks: HashMap<Hash256, Block>,
    /// 数据库句柄 (替代 HashMap)
    storage: Box<dyn BlockStorage>,
}

impl Blockchain {
    pub fn height(&self) -> u64 {
        self.height
    }
    pub fn head(&self) -> Hash256 {
        self.head
    }

    pub fn append_block(&self, block: Block) -> Result<Hash256, ChainError> {
        unimplemented!()
    }

    pub fn get_block(&self, hash256: Hash256) -> Option<&Block> {
        self.blocks.get(&hash256)
    }

    pub fn get_next_difficulty(&self) {}
}
