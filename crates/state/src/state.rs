//! WorldState 抽象

use latte_primitives::address::Address;
use latte_types::account::Account;
use std::collections::HashMap;
use crate::account_db::{AccountReader, AccountWriter};

#[derive(Default)]
pub struct WorldState {
    accounts: HashMap<Address, Account>,
}

impl WorldState {
    pub fn get_account(&self, addr: &Address) -> Option<&Account> {
        self.accounts.get(addr)
    }

    pub fn get_account_mut(&mut self, addr: &Address) -> Option<&mut Account> {
        self.accounts.get_mut(addr)
    }
}


impl AccountReader for WorldState {
    fn get(&self, addr: &Address) -> Option<&Account> {
        self.get_account(addr)
    }
}

impl AccountWriter for WorldState {
    fn get_mut(&mut self, addr: &Address) -> Option<&mut Account> {
        self.get_account_mut(addr)
    }
}