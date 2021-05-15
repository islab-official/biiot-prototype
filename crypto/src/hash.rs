use sha3::Digest;

pub fn keccak256(value: &[u8]) -> [u8;32] {
    let mut keccak_hash = [0u8;32];
    let h = sha3::Keccak256::digest(value.as_ref());
    let mut cnt = 0;
    for v in h.iter() { keccak_hash[cnt] = v.clone(); cnt += 1; }
    return keccak_hash;
}