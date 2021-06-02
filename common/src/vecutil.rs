use ethereum_types::H256;

pub fn copy_from(src: &Vec<u8>) -> Vec<u8> {
    let mut v = vec![] as Vec<u8>;
    for element in src.iter() { v.push(element.clone()); }
    return v;
}

pub fn copy_to_bytes32(src: &Vec<u8>) -> [u8;32] {
    let mut u832 = [0u8;32];
    for idx in 0..src.len() {
        u832[idx] = src.get(idx).unwrap().clone();
    }
    return u832;
}

pub fn copy_between(src: &Vec<u8>, dst: &mut Vec<u8>) {
    if dst.len() != 0 { dst.clear(); }
    for element in src.iter() { dst.push(element.clone()); }
}

pub fn bytes_to_h256(b: Vec<u8>) -> H256 {
    return H256::from_slice(b.as_slice());
}

pub fn right_pad_bytes(_slice: &Vec<u8>, _length: u64) -> Vec<u8> {
    let slice_size = _slice.len() as u64;
    if _length <= slice_size { return _slice.clone(); }
    let mut padded = _slice.clone();
    padded.resize_with(_length as usize, Default::default);
    padded
}

pub fn safe_mul(_x: u64, _y: u64) -> (u64, bool) {
    let x = _x.clone();
    u64::overflowing_mul(x, _y)
}

/// LE 기반의 벡터를 32 byte 배열로 변환한다.
pub fn levec_to_bytes32(v: &Vec<u8>) -> [u8;32] {
    let mut result = [0u8;32];
    for idx in 0..v.len() {
        result[idx] = v.get(idx).unwrap().clone();
    }
    return result;
}

/// BE 기반의 벡터를 32 byte 배열로 변환한다.
pub fn bevec_to_bytes32(v: &Vec<u8>) -> [u8;32] {
    let mut result = [0u8;32];
    let mut pos = 0;
    for idx in (0..v.len()).rev() {
        result[pos] = v.get(idx).unwrap().clone();
        pos += 1;
    }
    return result;
}