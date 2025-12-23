use latte_primitives::hash::Hash256;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: Hash256,
    //
    // State Root 是一个 Merkle Patricia Tree (MPT) 的根哈希。 想象一个巨大的数据库，里面存储了成千上万个账户：
    // Key: 账户地址（Address）。
    // Value: 包含 nonce（交易次数）、balance（余额）、storage_root（合约存储根）、code_hash（智能合约代码哈希）的结构体。
    // 将所有的 Key-Value 对组织成一棵树，这棵树的最顶端哈希值就是 State Root。
    // 保证：计算结果没有被改动
    //
    // 计算方式：
    // 初始状态：从上一个区块（$N-1$）的 state_root 开始，这代表了当前的账本快照。执行交易：虚拟机（VM）按顺序运行区块 $N$ 中的每一笔交易。
    // 例如：交易是 A 向 B 转账 10 元。VM 会修改状态树中 A 的余额（减 10）和 B 的余额（加 10）。
    // 应用变更：所有交易跑完后，所有的修改都被应用到状态树中。
    // 计算哈希：底层叶子节点哈希改变。哈希值沿树向上层层传递（Re-hashing）。生成全新的根哈希。
    // 写入区块头：这个生成的哈希值被填入区块头的 state_root 字段。
    // 使用：SPV 验证、防欺诈与共识、防欺诈与共识，通过 state_root，节点可以快速从数据库中定位并恢复到历史任何一个区块结束时的账本状态
    pub state_root: Hash256,
    // 保证：输入数据没有被改动；Merkle Root。它确保了区块体里的交易列表是完整且顺序正确
    pub tx_root: Hash256,
    pub number: u64, // 高度，与height是一个东西
    pub timestamp: u64, // second
}
