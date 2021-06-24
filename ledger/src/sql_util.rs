use ethereum_types::{U256, H256};

pub fn to_u256(v: Vec<u8>) -> U256 {
    let h = to_h256(v);
    U256::from_little_endian(h.as_bytes())
}

pub fn to_h256(v: Vec<u8>) -> H256 {
    let mut data = [0u8;32];
    let mut cnt = 0;
    for element in v.iter() {
        data[cnt] = element.clone();
        cnt += 1;
    }
    H256::from(data)
}
