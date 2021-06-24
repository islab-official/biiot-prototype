use ethereum_types::{Address, U256, H256};
use std::collections::HashMap;
use crate::opcode::{opcode_to_u8, JUMPDEST};

#[derive(Default)]
pub struct Contract {
    pub code: Vec<u8>,
    pub codehash: H256,
    pub address: Address,
    pub caller: Address,
    pub jump_points: HashMap<Vec<u8>, usize>,
    pub input: Vec<u8>
}

impl Contract {
    pub fn valid_jumpdest(&self, _dest: &U256) -> bool {
        let udest = _dest.as_u64();
        // PC cannot go beyong len(code) and certainly cannot be bigger than 63bits.
        if opcode_to_u8(self.code[udest as usize]) != JUMPDEST { return false; }

        return true;
        // return self.is_code(udest);
    }

    pub fn get_byte(&self, _n: u64) -> u8 {
        if _n < self.code.len() as u64 {
            return self.code[_n as usize];
        }
        return 0;
    }

    // fn get_op(&self, _n: u64) -> u8 {
    //     return
    // }

    // is_code returns true if the provided PC location is an actual opcode,
    // as opposed to a data-segment following a PUSHN operation.
    // fn is_code(&self, _udest: u64) -> bool {
    //     if self.codehash != H256::zero() {
    //         if !e
    //     }
    //     return true;
    // }
}