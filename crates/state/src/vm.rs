use crate::error::StateError;
use crate::state::WorldState;
use latte_primitives::address::Address;
use latte_types::transaction::Transaction;

pub trait VmEngine {
    fn execute(
        &self,
        state: &mut WorldState,
        caller: Address,
        tx: &Transaction,
    ) -> Result<(), StateError>;
}
