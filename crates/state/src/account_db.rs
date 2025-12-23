use latte_primitives::address::Address;
use latte_types::account::Account;

pub trait AccountReader {
    fn get(&self, addr: &Address) -> Option<&Account>;
}

pub trait AccountWriter {
    fn get_mut(&mut self, addr: &Address) -> Option<&mut Account>;
}
