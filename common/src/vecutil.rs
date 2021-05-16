use ethereum_types::H256;

pub fn copy_from(src: &Vec<u8>) -> Vec<u8> {
    let mut v = vec![] as Vec<u8>;
    for element in src.iter() { v.push(element.clone()); }
    return v;
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