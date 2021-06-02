use ethereum_types::{H256, Address, U256};
use crate::table::{Table, Container};
use rusqlite::{Connection, Statement};
use std::io::Error;
use rlp::Encodable;
use std::collections::HashMap;

// pub struct AccountTrie {
//     pub value: [AccountNode]
// }
//
// impl AccountTrie {
//     pub fn new_account(address: Address) {
//     }
// }
pub struct AccountState {
    node: AccountNode,
    storage: StorageTableManager,
}

impl AccountState {
    pub fn new(node: AccountNode, storage: StorageTableManager) -> Self {
        AccountState { node, storage }
    }

    pub fn get_node(&self) -> &AccountNode { &self.node }

    pub fn get_storage(&self) -> &StorageTableManager { &self.storage }
}

pub struct AccountNode {
    pub key: H256,
    pub nonce: H256,
    pub storage_root: H256,
    pub codehash: Vec<u8>,
}

impl Default for AccountNode {
    fn default() -> Self {
        AccountNode {
            key: H256::zero(),  // all nodes in secure trie get its own hash
            nonce: H256::zero(),
            storage_root: H256::zero(),
            codehash: vec![],
        }
    }
}


pub struct AccountStorage {
    pub key: H256,
    pub value: H256,
}

impl AccountStorage {
    pub fn new() -> Self {
        AccountStorage { key: H256::zero(), value: H256::zero() }
    }
}

pub struct AccountStorageTable {
    pub connection: Connection,
}

impl AccountStorageTable {
    pub fn new(contract_address: &Address) -> Self {
        let str_address = hex::encode(contract_address.as_bytes());
        let path = format!("{}.db", str_address);
        AccountStorageTable { connection: Connection::open(path).unwrap() }
    }
}

impl Default for AccountStorageTable {
    fn default() -> Self {
        let a = Address::zero();
        AccountStorageTable::new(&a)
    }
}

impl Table for AccountStorageTable {
    fn get_table_name(&self) -> String {
        "storage".to_string()
    }

    fn get_create_table_query(&self) -> String {
        "CREATE TABLE IF NOT EXISTS storage(\
            key BLOB,\
            value BLOB)"
            .to_string()
    }

    fn get_drop_table_query(&self) -> String {
        "DROP TABLE storage".to_string()
    }

    fn get_select_query(&self, where_type: &str) -> String {
        format!("SELECT * FROM storage WHERE {} = ?", where_type)
    }

    fn get_insert_query(&self) -> String {
        "INSERT INTO storage (key, value) VALUES (?, ?)".to_string()
    }

    fn get_update_query(&self) -> String {
        "UPDATE storage SET key = ?, value = ? WHERE key = ?".to_string()
    }

    fn get_delete_query(&self, where_type: &str) -> String {
        format!("DELETE FROM storage WHERE {} = ?", where_type)
    }

    fn make_statement(&self, query: &str) -> rusqlite::Statement {
        self.connection.prepare(query).unwrap()
    }
}

pub struct StorageTableManager {
    table: Vec<Box<dyn std::any::Any>>,
    pub database_name: String,
    pub table_name: String,
    pub contract_address: Address,
}

impl Container for StorageTableManager {
    fn initialize(&self) {
        let query = self.get_table().get_create_table_query();
        let mut stmt = self.get_table().make_statement(query.as_str());
        stmt.execute([]);
    }
}

impl StorageTableManager {
    pub fn new(contract_address: &Address) -> Self {
        let str_address = hex::encode(contract_address.as_bytes());
        let mut mngr = StorageTableManager {
            table: vec![],
            database_name: str_address,
            table_name: "storage".to_string(),
            contract_address: contract_address.clone(),
        };
        mngr.table.push(Box::new(AccountStorageTable::new(contract_address)));
        return mngr;
    }

    pub fn get_table(&self) -> &AccountStorageTable {
        self.table.get(0).unwrap().downcast_ref::<AccountStorageTable>().unwrap()
    }

    pub fn get_storage_value(&self, key: &H256) -> AccountStorage {
        let tbl = self.table.get(0).unwrap().downcast_ref::<AccountStorageTable>().unwrap();
        let query = tbl.get_select_query("key");
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let rows = stmt
            .query_map([key.as_bytes()], |row| {
                let keyvec: Vec<u8> = row.get(0)?;
                let valuevec: Vec<u8> = row.get(1)?;

                Ok(AccountStorage {
                    key: H256::from(common::vecutil::copy_to_bytes32(&keyvec)),
                    value: H256::from(common::vecutil::copy_to_bytes32(&valuevec)),
                })
            });
        AccountStorage { key: H256::zero(), value: H256::zero() }
    }

    pub fn insert_storage_value(&self, account_storage: &AccountStorage) -> Result<(), ()> {
        let tbl = self.table.get(0).unwrap().downcast_ref::<AccountStorageTable>().unwrap();
        let query = tbl.get_insert_query();
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let cnt = stmt.execute([
            account_storage.key.as_bytes(),
            account_storage.value.as_bytes()
        ]);
        Ok(())
    }

    pub fn update_storage_value(&self, account_storage: &AccountStorage) -> std::result::Result<(), ()> {
        let storage = self.get_storage_value(&account_storage.key);
        if storage.key == H256::zero() && storage.value == H256::zero() { return Err(()); }
        let tbl = self.table.get(0).unwrap().downcast_ref::<AccountStorageTable>().unwrap();
        let query = tbl.get_update_query();
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let cnt = stmt.execute([
            account_storage.key.as_bytes(),
            account_storage.value.as_bytes(),
            account_storage.key.as_bytes(),
        ]);
        return Ok(());
    }
}

/// AccountTable is a SQLite version of WorldStateTrie.
pub struct WorldStateTable {
    pub connection: Connection,
}

impl WorldStateTable {
    pub fn new() -> Self {
        let conn = Connection::open(crate::constant::DatabasePath);
        let tbl = WorldStateTable { connection: conn.unwrap() };
        // let query = tbl.get_create_table_query();
        // let mut stmt = tbl.make_statement(query.as_str());
        // stmt.execute([]);
        return tbl;
    }
}

impl Default for WorldStateTable {
    fn default() -> Self {
        WorldStateTable::new()
    }
}

impl Table for WorldStateTable {
    fn get_table_name(&self) -> String {
        "account".to_string()
    }

    fn get_create_table_query(&self) -> String {
        "CREATE TABLE IF NOT EXISTS account(\
            key BLOB,\
            nonce INTEGER,\
            storage_root BLOB,\
            codehash BLOB)"
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

    fn get_update_query(&self) -> String {
        "UPDATE account SET key = ?, nonce = ?, storage_root = ?, codehash = ? \
            WHERE key = ?"
            .to_string()
    }

    fn get_delete_query(&self, where_type: &str) -> String {
        format!("DELETE FROM account WHERE {} = ?", where_type)
    }

    fn make_statement(&self, query: &str) -> Statement {
        self.connection.prepare(query).unwrap()
    }
}

pub struct WorldStateTableManager {
    table: Vec<Box<dyn std::any::Any>>,
    pub table_name: String,
}

impl Container for WorldStateTableManager {
    fn initialize(&self) {
        let query = self.get_table().get_create_table_query();
        let mut stmt = self.get_table().make_statement(query.as_str());
        stmt.execute([]);
    }
}

impl WorldStateTableManager {
    pub fn new() -> Self {
        let tbl = WorldStateTable::new();
        let mut container = WorldStateTableManager { table: vec![], table_name: "account".to_string() };
        container.table.push(Box::new(tbl));
        return container;
    }

    pub fn get_table(&self) -> &WorldStateTable {
        self.table.get(0).unwrap().downcast_ref::<WorldStateTable>().unwrap()
    }

    pub fn exist(&self, account_key: &H256) -> bool {
        let tbl = self.table.get(0).unwrap().downcast_ref::<WorldStateTable>().unwrap();
        let query = tbl.get_select_query("key");
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let account_iter = stmt.query_map([account_key.as_bytes()], |row| {
            return Ok(true);
        });
        return false;
    }

    pub fn get_account(&self, address: &Address) -> AccountNode {
        let keccak_value = crypto::hash::keccak256(address.as_bytes());
        let keccak_address = H256::from(&keccak_value);
        let tbl = self.table.get(0).unwrap().downcast_ref::<WorldStateTable>().unwrap();
        let query = tbl.get_select_query("key");
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let account_iter = stmt.query_map([keccak_value.to_vec()], |row| {
            let key = crate::sql_util::to_h256(row.get(0)?);
            let nonce = crate::sql_util::to_h256(row.get(1)?);
            let storage_root = crate::sql_util::to_h256(row.get(2)?);
            // let codehash = crate::sql_util::to_H256(row.get(3)?);
            let codehash: Vec<u8> = row.get(3).unwrap();
            return Ok(AccountNode {
                key,
                nonce,
                storage_root,
                codehash,
            });
        });
        let mut not_found = AccountNode::default();
        not_found.key = keccak_address;
        return not_found;
    }

    pub fn update_account(&self, node: &AccountNode) -> Result<(), ()> {
        let tbl = self.table.get(0).unwrap().downcast_ref::<WorldStateTable>().unwrap();
        let query = tbl.get_update_query();
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let cnt = stmt
            .execute(
                [node.key.as_bytes(),
                    node.nonce.as_bytes(),
                    node.storage_root.as_bytes(),
                    // node.codehash.as_bytes(),
                    node.codehash.as_slice(),
                    node.key.as_bytes()]
            );
        return match cnt {
            Ok(n) => {
                if n != 0 { return Ok(()); }
                Err(())
            }
            Err(_) => { Err(()) }
        };
    }

    pub fn insert_account(&self, node: &AccountNode) -> Result<(), ()> {
        let tbl = self.table.get(0).unwrap().downcast_ref::<WorldStateTable>().unwrap();
        let query = tbl.get_insert_query();
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let cnt = stmt
            // .execute([node.key.as_bytes(), node.nonce.as_bytes(), node.storage_root.as_bytes(), node.codehash.as_bytes()]);
            .execute([node.key.as_bytes(), node.nonce.as_bytes(), node.storage_root.as_bytes(), node.codehash.as_slice()]);
        return match cnt {
            Ok(n) => {
                if n != 0 { return Ok(()); }
                Err(())
            }
            Err(_) => { Err(()) }
        };
    }
}