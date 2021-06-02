use rlp::{Decodable, Encodable, RlpStream, DecoderError, Rlp};
use ethereum_types::{U256, Address, H256, H160};
use crate::table::{Table, Container};
use rusqlite::{Connection, Statement, Row, Error};
use crate::account::WorldStateTableManager;

pub struct Hash160Vector(Vec<H160>);

impl Hash160Vector {
    pub fn new() -> Self { Hash160Vector { 0: vec![] }}
}

impl From<Vec<u8>> for Hash160Vector {
    fn from(value: Vec<u8>) -> Self {
        if value.len() % 20 != 0 { return Hash160Vector { 0: vec![] }; }
        let mut addresses: Vec<Address> = vec![];
        let mut temp_vec: Vec<u8> = vec![];
        for idx in 0..value.len() {
            if value.len() % 20 == 0 {
                addresses.push(Address::from_slice(temp_vec.as_slice()));
                temp_vec.clear();
            }
            temp_vec.push(value.get(idx).unwrap().clone());
        }

        addresses.push(Address::from_slice(temp_vec.as_slice()));
        return Hash160Vector { 0: addresses };
    }
}

impl Encodable for Hash160Vector {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(self.0.len());
        for element in self.0.iter() { s.append(element); }
    }
}

impl Decodable for Hash160Vector {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let mut result = Hash160Vector::new();
        let v = rlp.as_list::<H160>().unwrap();
        for element in v.iter() {
            result.0.push(element.clone());
        }
        return Ok(result);
    }
}

pub struct Hash256Vector(Vec<H256>);

impl Encodable for Hash256Vector {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.0);
    }
}

impl Decodable for Hash256Vector {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let mut result = Hash256Vector{ 0: vec![] };
        let v = rlp.as_list::<H256>().unwrap();
        for element in v.iter() {
            result.0.push(element.clone());
        }
        return Ok(result);
    }
}

pub struct RawTransaction {
    pub nonce: usize,
    pub gas_price: U256,
    pub gas: U256,
    pub recipient: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub v: usize,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl RawTransaction {
    pub fn get_sender(&self) -> Address {
        let rlp_tx = rlp::encode(self).to_vec();
        let hmsg = crypto::hash::keccak256(rlp_tx.as_slice());
        let mut r = [0u8;32];
        let mut s = [0u8;32];
        for idx in 0..self.r.len() {
            r[idx] = self.r.get(idx).unwrap().clone();
            s[idx] = self.s.get(idx).unwrap().clone();
        }
        let public_key = crypto::secp256k1::recover_from_vrs(&hmsg, self.v as i32, r, s).to_vec();
        let mut raw_address = [0u8;20];
        for idx in 0..public_key.len() {
            raw_address[idx] = public_key.get(12 + idx).unwrap().clone();
        }
        Address::from_slice(&raw_address)
    }
}

impl Encodable for RawTransaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.recipient);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl Decodable for RawTransaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(RawTransaction {
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas: rlp.val_at(2)?,
            recipient: rlp.val_at(3)?,
            value: rlp.val_at(4)?,
            data: rlp.val_at(5)?,
            v: rlp.val_at(6)?,
            r: rlp.val_at(7)?,
            s: rlp.val_at(8)?
        })
    }
}


pub struct Transaction {
    pub nonce: usize,
    pub recipient: Address,
    pub data: Vec<u8>,
    pub v: usize,
    pub r: Vec<u8>,     // always 32 bytes length
    pub s: Vec<u8>,     // always 32 bytes length
    pub timestamp: u64,
    pub state_hash: H256,
    pub parent_hash: Vec<u8>,
    pub committer: Address,
    pub validators: Hash160Vector,
}

impl Transaction {
    pub fn from_raw_transaction(ptx: &RawTransaction) -> Self {
        let mut tx = Transaction::default();
        tx.nonce = ptx.nonce.clone();
        tx.recipient = ptx.recipient.clone();
        tx.data = common::vecutil::copy_from(&ptx.data);
        tx.v = ptx.v.clone();
        common::vecutil::copy_between(&ptx.r, &mut tx.r);
        common::vecutil::copy_between(&ptx.s, &mut tx.s);
        tx.committer = Address::zero();
        tx.validators = Hash160Vector::new();
        return tx
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            nonce: 0,
            recipient: Address::zero(),
            data: Vec::new(),
            v: 0,
            r: Vec::new(),
            s: vec![],
            timestamp: 0,
            state_hash: H256::zero(),
            parent_hash: vec![],
            committer: Address::zero(),
            validators: Hash160Vector::new(),
        }
    }
}

impl From<&Row<'_>> for Transaction {
    fn from(row: &Row) -> Self {
        let address_vec: Vec<u8> = row.get(1).unwrap();
        let state_hash_vec: Vec<u8> = row.get(7).unwrap();
        let committer_vec: Vec<u8> = row.get(9).unwrap();
        let validators_vec: Vec<u8> = row.get(10).unwrap();

        Transaction {
            nonce: row.get(0).unwrap(),
            recipient: Address::from_slice(address_vec.as_slice()),
            data: row.get(2).unwrap(),
            v: row.get(3).unwrap(),
            r: row.get(4).unwrap(),
            s: row.get(5).unwrap(),
            timestamp: row.get(6).unwrap(),
            state_hash: H256::from_slice(state_hash_vec.as_slice()),
            parent_hash: row.get(8).unwrap(),
            committer: Address::from_slice(committer_vec.as_slice()),
            validators: Hash160Vector::from(validators_vec),
        }
    }
}

impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(9);
        s.append(&self.nonce);
        s.append(&self.recipient);
        s.append(&self.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
        s.append(&self.timestamp);
        s.append(&self.state_hash);
        s.append(&self.parent_hash);
        s.append(&self.committer);
        s.append(&self.validators);
    }
}

impl Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Transaction{
            nonce: rlp.val_at(0)?,
            recipient: rlp.val_at(1)?,
            data: rlp.val_at(2)?,
            v: rlp.val_at(3)?,
            r: rlp.val_at(4)?,
            s: rlp.val_at(5)?,
            timestamp: rlp.val_at(6)?,
            state_hash: rlp.val_at(7)?,
            parent_hash: rlp.val_at(8)?,
            committer: rlp.val_at(9)?,
            validators: rlp.val_at(10)?
        })
    }
}

pub struct TransactionTable {
    connection: Connection
}

impl TransactionTable {
    pub fn new() -> Self {
        let conn = Connection::open(crate::constant::DatabasePath);
        TransactionTable { connection: conn.unwrap() }
    }
}

impl Default for TransactionTable {
    fn default() -> Self {
        TransactionTable::new()
    }
}

impl Table for TransactionTable {
    fn get_table_name(&self) -> String {
        "tx".to_string()
    }

    fn get_create_table_query(&self) -> String {
        "CREATE TABLE IF NOT EXISTS tx(\
            nonce INTEGER,\
            recipient BLOB,\
            data BLOB,\
            v INTEGER,\
            r BLOB,\
            s BLOB,\
            timestamp INTEGER,\
            state_hash BLOB,\
            parent_hash BLOB,\
            committer BLOB,\
            validators BLOB)"    // rlp-encoded list of address
            .to_string()
    }

    fn get_drop_table_query(&self) -> String {
        "DROP TABLE tx".to_string()
    }

    fn get_select_query(&self, where_type: &str) -> String {
        format!("SELECT * FROM tx WHERE {} = ?", where_type)
    }

    fn get_insert_query(&self) -> String {
        "INSERT INTO tx \
            (nonce, recipient, data, v, r, s, timestamp, state_hash, parent_hash, committer, validators) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            .to_string()
    }

    fn get_update_query(&self) -> String {
        "".to_string()
    }

    fn get_delete_query(&self, where_type: &str) -> String {
        format!("DELETE FROM tx WHERE {} = ?", where_type)
    }

    fn make_statement(&self, query: &str) -> Statement {
        self.connection.prepare(query).unwrap()
    }
}

pub struct TransactionTableManager {
    table: Vec<Box<dyn std::any::Any>>,
    pub table_name: String,
}

impl Container for TransactionTableManager {
    fn initialize(&self) {
        let query = self.get_table().get_create_table_query();
        let mut stmt = self.get_table().make_statement(query.as_str());
        stmt.execute([]);
    }
}

impl TransactionTableManager {
    pub fn new() -> Self {
        let tbl = TransactionTable::new();
        let mut result = TransactionTableManager {
            table: vec![],
            table_name: tbl.get_table_name(),
        };
        result.table.push(Box::new(tbl));
        return result;
    }

    pub fn get_table(&self) -> &TransactionTable {
        self.table.get(0).unwrap().downcast_ref::<TransactionTable>().unwrap()
    }

    pub fn insert_transaction(&self, tx: Transaction) -> Result<(), ()> {
        let tbl = self.table.get(0).unwrap().downcast_ref::<TransactionTable>().unwrap();
        let query = tbl.get_insert_query();
        let mut stmt = tbl.connection.prepare(query.as_str()).unwrap();
        let cnt = stmt.execute([
            H256::from_low_u64_le(tx.nonce as u64).as_bytes(),
            tx.recipient.as_bytes(),
            tx.data.as_slice(),
            H256::from_low_u64_le(tx.v as u64).as_bytes(),
            tx.r.as_slice(),
            tx.s.as_slice(),
            H256::from_low_u64_le(tx.timestamp).as_bytes(),
            tx.state_hash.as_bytes(),
            tx.parent_hash.as_slice()
        ]);
        Ok(())
    }

    pub fn first_transaction(&self) -> Result<Transaction, ()> {
        let tbl = self.get_table();
        let query = "SELECT * FROM transaction ORDER BY timestamp ASC LIMIT 1";
        let mut stmt = tbl.connection.prepare(query).unwrap();
        let datum = stmt.query_row([], |row| {
           Ok(Transaction::from(row))
        });
        return match datum {
            Ok(_) => { Ok(datum.unwrap()) }
            Err(_) => { Err(())}
        }
    }
}
