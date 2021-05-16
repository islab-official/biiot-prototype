use ethereum_types::{H256, Address, U256};
use crate::table::Table;
use rusqlite::Connection;

pub struct AccountTrie {
    pub value: [AccountNode]
}

impl AccountTrie {
    pub fn new_account(address: Address) {

    }
}

pub struct AccountNode {
    pub key: H256,
    pub nonce: u64,
    pub storage_root: H256,
    pub codehash: Address
}

impl Default for AccountNode {
    fn default() -> Self {
        AccountNode {
            key: H256::zero(),  // all nodes in secure trie get its own hash
            nonce: 0,
            storage_root: H256::zero(),
            codehash: Address::zero()
        }
    }
}


pub struct AccountStorage {
    key: U256,
    value: U256
}


pub struct AccountTable {
    connection: Connection
}

impl AccountTable {
    pub fn new(contract_address: &Address) -> Self {
        let v = contract_address.0.to_vec();
        let conn = Connection::open(hex::encode(v));
        AccountTable {
            connection: conn?
        }
    }
}

impl Default for AccountTable {
    fn default() -> Self {
        AccountTable::new(&Address::zero())
    }
}

impl Table for AccountTable {
    fn get_table_name(&self) -> String {
        "account".to_string()
    }

    fn get_create_table_query(&self) -> String {
        "CREATE TABLE IF NOT EXISTS account(\
            key BLOB,\
            nonce INTEGER,\
            storage_root BLOB,\
            codehash BLOB"
            .to_string()
    }

    fn get_drop_table_query(&self) -> String {
        "DROP TABLE account".to_string()
    }

    fn get_select_query(&self, where_type: &str) -> String {
        format!("SELECT * FROM account WHERE {} = ?", where_type)
    }

    fn get_insert_query(&self) -> String {
        "INSERT INTO account \
            (key, nonce, storage_root, codehash) \
            VALUES (?, ?, ?, ?)"
            .to_string()
    }

    fn get_delete_query(&self, where_type: &str) -> String {
        format!("DELETE FROM account WHERE {} = ?", where_type)
    }
}
