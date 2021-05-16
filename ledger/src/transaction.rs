use rlp::{Decodable, Encodable, RlpStream, DecoderError, Rlp};
use ethereum_types::{U256, Address, H256};
use crate::table::Table;
use rusqlite::Connection;

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
    pub parent_hash: HashVector,
}

impl Transaction {
    pub fn from_raw_transaction(ptx: &PseudoTransaction) -> Self {
        let mut tx = Transaction::default();
        tx.nonce = ptx.nonce.clone();
        tx.recipient = ptx.recipient.clone();
        tx.data = copy_from(&ptx.data);
        tx.v = ptx.v.clone();
        copy_between(&ptx.r, &mut tx.r);
        copy_between(&ptx.s, &mut tx.s);
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
            parent_hash: HashVector{0: vec![]},
        }
    }
}

pub struct TransactionTable {
    connection: Connection
}

impl TransactionTable {
    pub fn new(contract_address: &Address) -> Self {
        let v = contract_address.0.to_vec();
        let conn = Connection::open(hex::encode(v));
        TransactionTable {
            connection: conn?
        }
    }
}

impl Default for TransactionTable {
    fn default() -> Self {
        TransactionTable::new(&Address::zero())
    }
}

impl Table for TransactionTable {
    fn get_table_name(&self) -> String {
        "transaction".to_string()
    }

    fn get_create_table_query(&self) -> String {
        "CREATE TABLE IF NOT EXISTS transaction(\
            nonce INTEGER,\
            recipient BLOB,\
            data BLOB,\
            v INTEGER,\
            r BLOB,\
            s BLOB,\
            timestamp INTEGER,\
            state_hash BLOB,\
            parent_hash BLOB"
            .to_string()
    }

    fn get_drop_table_query(&self) -> String {
        "DROP TABLE transaction".to_string()
    }

    fn get_select_query(&self, where_type: &str) -> String {
        format!("SELECT * FROM transaction WHERE {} = ?", where_type)
    }

    fn get_insert_query(&self) -> String {
        "INSERT INTO transaction \
            (nonce, recipient, data, v, r, s, timestamp, state_hash, parent_hash) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            .to_string()
    }

    fn get_delete_query(&self, where_type: &str) -> String {
        format!("DELETE FROM transaction WHERE {} = ?", where_type)
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
            parent_hash: rlp.val_at(8)?
        })
    }
}