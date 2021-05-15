use ethereum_types::H256;
use crate::hash::keccak256;
use std::io::Write;

struct Message {
    value: Vec<u8>
}

impl Message {
    pub fn new(value: Vec<u8>) -> Self {
        let mut msg = Message { value: vec![] };
        for v in value.iter() { msg.value.push(v.clone()); }
        msg
    }

    pub fn from_h256(value: &H256) -> Message { Self::new(value.0.to_vec()) }

    pub fn as_slice(&self) -> &[u8] { self.value.as_slice() }
}

pub struct HashMessage {
    value: Vec<u8>
}

impl HashMessage {
    pub fn new() -> Self { HashMessage { value: vec![] } }

    pub fn from_string(value: &String) -> Self {
        let mut hmsg = HashMessage::new();
        let h = keccak256(value.as_ref());
        hmsg.value.write_all(&h);
        hmsg
    }

    pub fn from_vec(value: &Vec<u8>) -> Self {
        let mut hmsg = HashMessage::new();
        let h = keccak256(value.as_ref());
        hmsg.value.write_all(&h);
        hmsg
    }

    pub fn to_32u8(&self) -> [u8;32] {
        let mut fixed_hash = [0u8;32];
        let mut cnt = 0;
        for v in self.value.iter() {
            fixed_hash[cnt] = v.clone();
            cnt += 1;
        }
        fixed_hash
    }
}