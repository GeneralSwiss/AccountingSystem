mod account;
mod event;
mod ledger;
mod transaction;

use crate::{
    account::{Account, AccountId},
    ledger::Ledger,
};
use std::collections::HashMap;

pub struct AccountingSystem {
    account_registry: HashMap<AccountId, Account>,
    ledger: Ledger,
}

impl AccountingSystem {
    pub fn register_account(&mut self, account: Account) {
        self.account_registry.insert(account.id, account);
    }
}
