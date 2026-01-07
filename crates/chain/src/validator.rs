use crate::blockchain::Blockchain;
use crate::error::ChainError;
use crate::merkle;
use chrono::{Duration, Utc};
use latte_primitives::hash::Hash256;
use latte_types::block::Block;
use std::hash::Hash;

pub struct BlockValidator {}

///
/// 校验一个新区块是否可以接入Ï
///
/// - 链完整性校验: hash、高度连续性、时间戳校验
/// - 状态合法性校验：确保所有节点在执行相同的交易后，得到的“账本结果”是完全一致的；偏向金额等数据的正确性
///     状态合法性校验要求实现一个**“只读模拟器”**。不能直接修改当前的数据库，而是要在内存中创建一个副本，跑完所有交易，计算根值，最后再抛弃这个副本
/// - 共识规则校验：根据不同的共识算法做相应的校验，比如挖矿，检查hash是否满足难度要求，gas总和是否超过设定的gas_limit
/// - 默克尔树根校验:证明某笔交易确实存在于该区块中;数据没有被篡改或者丢失，偏向数据的完整性
///
/// 原则：校验应该从易到难，避免不必要的计算
impl BlockValidator {
    pub fn verify_block(
        &self,
        block: &Block,
        parent_hash256: Hash256,
        blockchain: &Blockchain,
    ) -> Result<(), ChainError> {
        // 基础信息验证
        self.validate_basic_info(block, parent_hash256, blockchain)?;
        // todo 状态验证，需要模拟执行
        // 共识规则 校验， pow,pos等
        
        // 默克尔树 校验
        self.validate_tx_root(block)?;
        Ok(())
    }

    fn validate_basic_info(
        &self,
        block: &Block,
        parent_hash: Hash256,
        blockchain: &Blockchain,
    ) -> Result<(), ChainError> {
        let header = &block.header;
        let hash256 = header.parent_hash;
        // 校验hash
        if hash256.0 != parent_hash.0 {
            return Err(ChainError::InvalidParent);
        }

        // 校验高度
        match blockchain.get_block(parent_hash) {
            Some(parent_block) => {
                if parent_block.header.number != header.number {
                    return Err(ChainError::InvalidHeight);
                }
            }
            None => {
                return Err(ChainError::InvalidHeight);
            }
        }

        // 时间校验
        let now = Utc::now().timestamp();
        let diff_time = (now - header.timestamp as i64).abs();
        // 对比小时数转成的秒数 (i64)
        if diff_time > Duration::hours(1).num_seconds() {
            return Err(ChainError::TimeoutError);
        }
        Ok(())
    }

    fn validate_tx_root(&self, block: &Block) -> Result<(), ChainError> {
        let real_hash256 = merkle::tx_root_hash(&block.transactions);
        let tx_root = block.header.tx_root;

        if real_hash256.0 == tx_root.0 {
            Ok(())
        } else {
            Err(ChainError::TxRootMismatch)
        }
    }

    // fn validate_state_root(
    //     &self,
    //     block: &Block,
    //     pre_state: &impl StateProvider
    // ) -> Result<(), ChainError> {
    //     // 1. 初始化覆盖层
    //     let mut overlay = StateOverlay::new(pre_state);
    //
    //     // 2. 模拟执行
    //     for tx in &block.transactions {
    //         // 这里调用你的虚拟机逻辑
    //         // 比如：执行 A -> B 转账，修改 overlay 中的余额
    //         self.vm.execute_transaction(tx, &mut overlay)?;
    //     }
    //
    //     // 3. 处理区块奖励 (矿工加钱)
    //     self.apply_block_reward(&mut overlay, &block.header.proposer)?;
    //
    //     // 4. 计算最终根值
    //     // 你需要将 overlay.modified_accounts 与 pre_state 合并计算 Trie Root
    //     let calculated_root = self.trie_library.calculate_root(&overlay)?;
    //
    //     // 5. 比对
    //     if calculated_root != block.header.state_root {
    //         return Err(ChainError::InvalidStateRoot);
    //     }
    //
    //     Ok(())
    // }
}
