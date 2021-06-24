use std::collections::HashMap;
use ethereum_types::{H256, Address};
use rlp::{Encodable, Decodable, RlpStream, DecoderError, Rlp};

pub struct DirtyKeyValue(H256, H256);

impl Encodable for DirtyKeyValue {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.0);
        s.append(&self.1);
    }
}

impl Decodable for DirtyKeyValue {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(DirtyKeyValue {
            0: rlp.val_at(0)?,
            1: rlp.val_at(1)?,
        })
    }
}


pub struct DirtyKeyValues(HashMap<H256, H256>);

impl DirtyKeyValues {
    pub fn new() -> Self { DirtyKeyValues { 0: HashMap::new() } }

    /// HashMap을 Key 값을 기준으로 오름차 정렬한 vector로 반환하는 메서드
    fn sort_to_vec(&self) -> Vec<DirtyKeyValue> {
        let mut v = Vec::<DirtyKeyValue>::new();
        for element in self.0.iter() {
            let pos = self.get_pos(&v, element.0);
            v.insert(pos, DirtyKeyValue { 0: element.0.clone(), 1: element.1.clone() });
        }
        return v;
    }

    /// 주어진 리스트 l에서 new_key가 들어갈 곳의 index를 반환하는 메서드
    fn get_pos(&self, l: &Vec<DirtyKeyValue>, new_key: &H256) -> usize {
        for idx in 0..l.len() {
            let key = l.get(idx).unwrap().clone().0;
            if key < new_key.clone() { return idx; }
        }
        return 0;
    }
}

impl From<HashMap<H256, H256>> for DirtyKeyValues {
    fn from(account_state: HashMap<H256, H256>) -> Self {
        DirtyKeyValues { 0: account_state }
    }
}

impl Encodable for DirtyKeyValues {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(self.0.len());
        for kv in self.sort_to_vec().iter() {
            s.append(kv.clone());
        }
    }
}

impl Decodable for DirtyKeyValues {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let mut result = HashMap::<H256, H256>::new();
        let size = rlp.size();
        for idx in 0..size {
            let v = rlp.list_at(idx).unwrap() as Vec<H256>;
            result.insert(
                v.get(0).unwrap().clone(),
                v.get(1).unwrap().clone()
            );
        }
        Ok(DirtyKeyValues::from(result))
    }
}

// impl Copy for DirtyKeyValues {}

impl Clone for DirtyKeyValues {
    fn clone(&self) -> Self {
        let mut  result = DirtyKeyValues { 0: HashMap::new() };
        for element in self.0.iter() {
            result.0.insert(element.0.clone(), element.1.clone());
        }
        return result;
    }
}


pub struct DirtyState(Address, DirtyKeyValues);

impl Encodable for DirtyState {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(2);
        s.append(&self.0);
        s.append(&self.1);
    }
}

impl Decodable for DirtyState {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(DirtyState {
            0: rlp.val_at(0).unwrap(),
            1: rlp.val_at(1).unwrap()
        })
    }
}


pub struct DirtyStates(HashMap<Address, DirtyKeyValues>);

impl DirtyStates {
    pub fn new() -> Self { DirtyStates { 0: HashMap::new() } }

    fn sort_to_vec(&self) -> Vec<DirtyState> {
        let mut v: Vec<DirtyState> = vec![];
        for element in self.0.iter() {
            let map_key = element.0.clone();
            let map_vec_pos = self.get_pos(&v, element.0);
            let kvs = element.1.clone();
            let state = DirtyState { 0: map_key, 1: kvs };
            v.insert(map_vec_pos, state);
        }
        return v;
    }

    fn get_pos(&self, l: &Vec<DirtyState>, new_address: &Address) -> usize {
        for address_idx in 0..l.len() {
            let address = l.get(address_idx).unwrap().clone().0;
            if address < new_address.clone() { return address_idx; }
        }
        return 0;
    }
}

impl From<HashMap<Address, DirtyKeyValues>> for DirtyStates {
    fn from(account_states: HashMap<Address, DirtyKeyValues>) -> Self {
        DirtyStates { 0: account_states }
    }
}

impl Encodable for DirtyStates {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(self.0.len());
        for state in self.sort_to_vec().iter() {
            s.append(state);
        }
    }
}

impl Decodable for DirtyStates {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        let mut result = DirtyStates {0: HashMap::new()};
        let size = rlp.size();
        for state_idx in 0..size {
            let state: DirtyState = rlp.val_at(state_idx).unwrap();
            result.0.insert(state.0, state.1);
        }
        return Ok(result);
    }
}