use crate::storage::BlockStorage;
use crate::error::ChainError;
use latte_codec::codec::Codec;
use latte_primitives::hash::Hash256;
use latte_types::block::Block;
use std::collections::HashMap;
use std::sync::Arc;
use crate::validator::BlockValidator;

/// 说明：后续添加存储实现时，支持从缓存中如果取不到，从本地存储加载区块，比如rockDB，内存只缓存一部分区块
pub struct Blockchain<C: Codec> {
    /// 指向当前主链的最顶端
    head: Hash256,
    /// 当前主链的高度
    height: u64,
    /// 内存中的热数据缓存 (后续替代为 LruCache)
    blocks: HashMap<Hash256, Block>,
    /// 数据库句柄 (替代 HashMap)
    storage: Box<dyn BlockStorage>,
    /// 统一的编码器，用于计算区块哈希等
    codec: C,
}

impl<C: Codec> Blockchain<C> {
    pub fn height(&self) -> u64 {
        self.height
    }
    pub fn head(&self) -> Hash256 {
        self.head
    }

    pub fn append_block(&mut self, block: Block) -> Result<Hash256, ChainError> {
        // 1. 创建验证器实例
        let validator = BlockValidator {};
        
        // 2. 获取父区块哈希
        let parent_hash = block.header.parent_hash;
        
        // 3. 验证区块
        validator.verify_block(&block, parent_hash, self)?;
        
        // 4. 计算区块哈希
        let block_hash = block
            .hash_with(&self.codec)
            .map_err(ChainError::InvalidBlock)?;
        
        // 5. 存储区块到内存缓存
        self.blocks.insert(block_hash, block.clone());
        
        // 6. 存储区块到持久化存储
        self.storage.put_block(&block)?;
        
        // 7. 更新区块链头部和高度
        self.head = block_hash;
        self.height = block.header.number;
        
        // 8. TODO: 广播新区块到P2P网络
        // 这里需要调用P2P模块的广播功能，通知其他节点有新区块
        
        Ok(block_hash)
    }

    pub fn get_block(&self, hash256: Hash256) -> Option<&Block> {
        self.blocks.get(&hash256)
    }

    pub fn get_next_difficulty(&self) {}
}
