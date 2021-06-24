use crate::stack::{Stack, ReturnStack};
use crate::memory::Memory;
use crate::contract::Contract;

pub struct CallContext {
    pub stack: Stack,
    pub memory: Memory,
    pub rstack: ReturnStack,
    pub contract: Contract
}