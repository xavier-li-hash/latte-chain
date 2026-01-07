// 如何原子性地：
// •	apply tx
// •	commit / rollback state
// •	如何计算：
// •	tx_root
// •	state_root

use latte_state::executor::Executor;
use latte_state::state::WorldState;
use latte_state::vm::VmEngine;
use latte_types::block::Block;
use latte_types::transaction::Transaction;
use latte_vm::engine::ScriptVm;
use crate::error::ChainError;

pub struct BlockExecutor<'a, V: VmEngine> {
    state: &'a WorldState,
    executor:&'a Executor<'a, V>,
}

impl<'a> BlockExecutor<'a, ScriptVm> {
    pub fn new(state: &'a mut WorldState, executor: &'a Executor<ScriptVm>) -> Self {
        Self{state,executor}
    }

    pub fn execute(&self, block: &Block) -> Result<(), ()> {
        unimplemented!()
    }

    pub fn apply_block(&self, block: &Block) -> Result<(), ChainError> {
        // 这里是处理区块，区块理论上是已经经过矿机挖矿后得到的
        //
        unimplemented!()
    }
}
