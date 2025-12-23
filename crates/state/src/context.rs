use latte_primitives::address::Address;

#[derive(Clone, Debug)]
pub struct ExecutorContext {
    pub caller: Option<Address>,
    pub gas_limit: u64,
}