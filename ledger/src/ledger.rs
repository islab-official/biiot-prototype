use crate::account::{WorldStateTable, AccountNode, WorldStateTableManager, StorageTableManager, AccountState};
use ethereum_types::{Address, H256};
use crate::transaction::{TransactionTable, TransactionTableManager};
use std::collections::HashMap;
use crate::pool::TxPool;
use crate::table::{Table, Container};
use crate::dirty_state::DirtyStates;
use std::sync::Arc;

const DatabasePath: &str = "biiot.db";

pub struct Ledger {
    pub accounts: WorldStateTableManager,
    pub transactions: TransactionTableManager,
    pub pool: TxPool,
    pub dirty_state: Arc<DirtyStates>,
}

/// Property
impl Ledger {
    pub fn get_accounts(&self) -> &WorldStateTableManager { &self.accounts }
    pub fn get_transactions(&self) -> &TransactionTableManager { &self.transactions }
    pub fn get_pool(&self) -> &TxPool { &self.pool }
    pub fn get_dirty_state(&mut self) -> Arc<DirtyStates> { self.dirty_state.clone() }
}

/// Methods
impl Ledger {
    pub fn new() -> Self {
        let ledger = Ledger {
            accounts: WorldStateTableManager::new(),
            transactions: TransactionTableManager::new(),
            pool: TxPool::new(),
            dirty_state: Arc::new(DirtyStates::new()),
        };
        return ledger;
    }

    pub fn account_state(address: &Address) -> StorageTableManager {
        return StorageTableManager::new(address);
    }

    pub fn initialize(&self) {
        self.accounts.initialize();
        self.transactions.initialize();
    }

    pub fn get_account(&mut self, address: &Address) -> AccountState {
        let node = self.accounts.get_account(address);
        let storage = StorageTableManager::new(address);
        storage.initialize();
        return AccountState::new(node, storage);
    }

    pub fn upsert_account(&mut self, node: &AccountNode) {
        match self.accounts.exist(&node.key) {
            true => { self.accounts.update_account(node); }
            false => { self.accounts.insert_account(node); }
        }
    }
}