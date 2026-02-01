use crate::error::StateError;
use crate::state::WorldState;
use crate::vm::VmEngine;
use latte_types::transaction::Transaction;

pub struct Executor<'a, V: VmEngine> {
    vm: &'a V,
}

// 提交交易，扣款和加钱
impl<'a, V: VmEngine> Executor<'a, V> {
    pub fn apply_tx(&self, state: &mut WorldState, tx: &Transaction) -> Result<(), StateError> {
        let from = tx.from;
        let sender = state.get_account_mut(&from).unwrap();

        // 1. 校验nonce
        if sender.nonce != tx.nonce {
            return Err(StateError::InvalidNonce);
        }
        // 2. 校验balance
        if sender.balance < tx.value as u128 {
            return Err(StateError::InsufficientBalance);
        }
        // 3. 扣款
        sender.balance -= tx.value as u128;
        sender.nonce += 1;
        // 4. 收款
        if let Some(to) = tx.to {
            let receiver = state.get_account_mut(&to).unwrap();
            receiver.balance += tx.value as u128;
        }
        if !tx.data.is_empty() {
            // 调用抽象的vm
            self.vm.execute(state, from, tx)?;
        }
        Ok(())
    }
}
