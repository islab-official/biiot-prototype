pub mod constants;
pub mod context;
pub mod contract;
pub mod err;
pub mod evm;
pub mod instruction;
pub mod interpreter;
pub mod jumptable;
pub mod memory;
pub mod opcode;
pub mod stack;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
