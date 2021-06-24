use ethereum_types::U256;
use std::ops::Shr;

pub fn u256_to_le_u8vec(u256: &U256) -> Vec<u8> {
    let mut u8vec = Vec::new();
    for idx in 0..4 {
        for idx2 in 0..8 {
            u8vec.push(u256.0[idx].shr(idx2 * 8) as u8);
        }
    }
    return u8vec;
}