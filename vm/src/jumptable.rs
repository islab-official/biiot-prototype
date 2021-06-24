use crate::instruction::{op_add, op_sub, op_mul, op_div, op_mod, op_exp, op_not, op_lt, op_eq, op_iszero, op_and, op_sha3, op_address, op_callvalue, op_calldataload, op_calldatasize, op_codecopy, op_pop, op_coinbase, op_mload, op_mstore, op_jump, op_jumpi, op_jumpdest, op_msize, op_return, op_revert, op_stop, op_push1, make_log0, make_log1, make_log2, make_log3, make_log4, make_push2, make_push3, make_push4, make_push32, make_push31, make_push30, make_push29, make_push28, make_push27, make_push26, make_push25, make_push24, make_push23, make_push22, make_push21, make_push20, make_push19, make_push18, make_push17, make_push16, make_push15, make_push14, make_push13, make_push12, make_push11, make_push10, make_push9, make_push8, make_push7, make_push6, make_push5, make_dup1, make_dup2, make_dup3, make_dup4, make_dup5, make_dup6, make_dup7, make_dup8, make_dup9, make_dup10, make_dup11, make_dup12, make_dup13, make_dup14, make_dup15, make_dup16, make_swap16, make_swap15, make_swap14, make_swap13, make_swap12, make_swap11, make_swap10, make_swap9, make_swap8, make_swap7, make_swap6, make_swap5, make_swap4, make_swap3, make_swap2, make_swap1, op_calldatacopy};
use crate::err::RunError;
use std::cell::Cell;
use crate::interpreter::Interpreter;
use crate::context::CallContext;
use std::collections::HashMap;
use crate::stack::Stack;
use crate::opcode::OpCode;
use crate::memory::{memory_sha3, memory_code_copy, memory_mload, memory_mstore, memory_return, memory_revert, memory_log, memory_calldata_copy};
use crate::evm::VirtualMachine;

// pub type ExecuteFn = fn(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext)
//                         -> (Option<Vec<u8>>, Option<RunError>);
pub type ExecuteFn = fn(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                        -> (Option<Vec<u8>>, Option<RunError>);

pub type MemSizeFn = fn(_stack: &Stack) -> (u64, bool);

// fn a() -> ExecuteFn {
//     return |_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext|
//             -> (Option<Vec<u8>>, Option<RunError>) {
//         (None, None)
//     };
// }
fn a() -> ExecuteFn {
    return |_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext|
            -> (Option<Vec<u8>>, Option<RunError>) {
        (None, None)
    };
}

#[derive(Copy, Clone)]
pub struct Operation {
    pub execute: ExecuteFn,
    pub memory_size: Option<MemSizeFn>,
    pub min_stack: i16,
    pub max_stack: i16,
    pub halts: bool,
    pub jumps: bool,
    pub writes: bool,
    pub reverts: bool,
    pub returns: bool,
}

pub fn get_instruction_set() -> HashMap<u8, Operation> {
    let is: HashMap<u8, Operation> = [
        (crate::opcode::ADD, // 0x01
         Operation {
             execute: op_add,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SUB, // 0x03
         Operation {
             execute: op_sub,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::MUL, // 0x02
         Operation {
             execute: op_mul,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DIV, // 0x04
         Operation {
             execute: op_div,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::MOD, // 0x06
         Operation {
             execute: op_mod,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::EXP, // 0x0A
         Operation {
             execute: op_exp,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::NOT, // 0x19
         Operation {
             execute: op_not,
             memory_size: None,
             min_stack: crate::stack::min_stack(&1i16, &1i16),
             max_stack: crate::stack::max_stack(&1i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LT, // 0x10
         Operation {
             execute: op_lt,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::EQ, // 0x14
         Operation {
             execute: op_eq,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::ISZERO, // 0x15
         Operation {
             execute: op_iszero,
             memory_size: None,
             min_stack: crate::stack::min_stack(&1i16, &1i16),
             max_stack: crate::stack::max_stack(&1i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::AND, // 0x16
         Operation {
             execute: op_and,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SHA3, // 0x20
         Operation {
             execute: op_sha3,
             memory_size: Some(memory_sha3),
             min_stack: crate::stack::min_stack(&2i16, &1i16),
             max_stack: crate::stack::max_stack(&2i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::ADDRESS, // 0x30
         Operation {
             execute: op_address,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::CALLVALUE, // 0x34
         Operation {
             execute: op_callvalue,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::CALLDATALOAD, // 0x35
         Operation {
             execute: op_calldataload,
             memory_size: None,
             min_stack: crate::stack::min_stack(&1i16, &1i16),
             max_stack: crate::stack::max_stack(&1i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::CALLDATASIZE, // 0x36
         Operation {
             execute: op_calldatasize,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::CALLDATACOPY, // 0x37
         Operation {
             execute: op_calldatacopy,
             memory_size: Some(memory_calldata_copy),
             min_stack: crate::stack::min_stack(&3i16, &0i16),
             max_stack: crate::stack::max_stack(&3i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }
        ),
        (crate::opcode::CODECOPY, // 0x39
         Operation {
             execute: op_codecopy,
             memory_size: Some(memory_code_copy),
             min_stack: crate::stack::min_stack(&3i16, &0i16),
             max_stack: crate::stack::max_stack(&3i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::POP, // 0x50
         Operation {
             execute: op_pop,
             memory_size: None,
             min_stack: crate::stack::min_stack(&1i16, &0i16),
             max_stack: crate::stack::max_stack(&1i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::COINBASE, // 0x41
         Operation {
             execute: op_coinbase,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::MLOAD, // 0x51
         Operation {
             execute: op_mload,
             memory_size: Some(memory_mload),
             min_stack: crate::stack::min_stack(&1i16, &1i16),
             max_stack: crate::stack::max_stack(&1i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::MSTORE, // 0x52
         Operation {
             execute: op_mstore,
             memory_size: Some(memory_mstore),
             min_stack: crate::stack::min_stack(&2i16, &0i16),
             max_stack: crate::stack::max_stack(&2i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::JUMP, // 0x56
         Operation {
             execute: op_jump,
             memory_size: None,
             min_stack: crate::stack::min_stack(&1i16, &0i16),
             max_stack: crate::stack::max_stack(&1i16, &0i16),
             halts: false,
             jumps: true,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::JUMPI, // 0x57
         Operation {
             execute: op_jumpi,
             memory_size: None,
             min_stack: crate::stack::min_stack(&2i16, &0i16),
             max_stack: crate::stack::max_stack(&2i16, &0i16),
             halts: false,
             jumps: true,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::JUMPDEST, // 0x5B
         Operation {
             execute: op_jumpdest,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &0i16),
             max_stack: crate::stack::max_stack(&0i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::MSIZE, // 0x59
         Operation {
             execute: op_msize,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::RETURN, // 0xF3
         Operation {
             execute: op_return,
             memory_size: Some(memory_return),
             min_stack: crate::stack::min_stack(&2i16, &0i16),
             max_stack: crate::stack::max_stack(&2i16, &0i16),
             halts: true,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::REVERT, // 0xFD
         Operation { // wrong place
             execute: op_revert,
             memory_size: Some(memory_revert),
             min_stack: crate::stack::min_stack(&2i16, &0i16),
             max_stack: crate::stack::max_stack(&2i16, &0i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: true,
             returns: true,
         }),
        (crate::opcode::STOP, // 0x00
         Operation {
             execute: op_stop,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &0i16),
             max_stack: crate::stack::max_stack(&0i16, &0i16),
             halts: true,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LOG0, // 0xA0
         Operation {
             execute: make_log0,
             memory_size: Some(memory_log),
             min_stack: crate::stack::min_stack(&2i16, &0i16),
             max_stack: crate::stack::max_stack(&2i16, &0i16),
             halts: false,
             jumps: false,
             writes: true,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LOG1, // 0xA1
         Operation {
             execute: make_log1,
             memory_size: Some(memory_log),
             min_stack: crate::stack::min_stack(&3i16, &0i16),
             max_stack: crate::stack::max_stack(&3i16, &0i16),
             halts: false,
             jumps: false,
             writes: true,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LOG2, // 0xA2
         Operation {
             execute: make_log2,
             memory_size: Some(memory_log),
             min_stack: crate::stack::min_stack(&4i16, &0i16),
             max_stack: crate::stack::max_stack(&4i16, &0i16),
             halts: false,
             jumps: false,
             writes: true,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LOG3, // 0xA3
         Operation {
             execute: make_log3,
             memory_size: Some(memory_log),
             min_stack: crate::stack::min_stack(&5i16, &0i16),
             max_stack: crate::stack::max_stack(&5i16, &0i16),
             halts: false,
             jumps: false,
             writes: true,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::LOG4, // 0xA4
         Operation {
             execute: make_log4,
             memory_size: Some(memory_log),
             min_stack: crate::stack::min_stack(&6i16, &0i16),
             max_stack: crate::stack::max_stack(&6i16, &0i16),
             halts: false,
             jumps: false,
             writes: true,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH1, // 0x60
         Operation {
             execute: op_push1,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH2, // 0x61
         Operation {
             execute: make_push2,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH3, // 0x62
         Operation {
             execute: make_push3,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH4, // 0x63
         Operation {
             execute: make_push4,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH5, // 0x64
         Operation {
             execute: make_push5,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH6, // 0x65
         Operation {
             execute: make_push6,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH7, // 0x66
         Operation {
             execute: make_push7,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH8, // 0x67
         Operation {
             execute: make_push8,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH9, // 0x68
         Operation {
             execute: make_push9,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH10, // 0x69
         Operation {
             execute: make_push10,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH11, // 0x6A
         Operation {
             execute: make_push11,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH12, // 0x6B
         Operation {
             execute: make_push12,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH13, // 0x6C
         Operation {
             execute: make_push13,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH14, // 0x6D
         Operation {
             execute: make_push14,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH15, // 0x6E
         Operation {
             execute: make_push15,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH16, // 0x6F
         Operation {
             execute: make_push16,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH17, // 0x70
         Operation {
             execute: make_push17,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH18, // 0x71
         Operation {
             execute: make_push18,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH19, //0x72
         Operation {
             execute: make_push19,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH20, // 0x73
         Operation {
             execute: make_push20,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH21, // 0x74
         Operation {
             execute: make_push21,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH22, // 0x75
         Operation {
             execute: make_push22,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH23, // 0x76
         Operation {
             execute: make_push23,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH24, // 0x77
         Operation {
             execute: make_push24,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH25, // 0x78
         Operation {
             execute: make_push25,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH26, // 0x79
         Operation {
             execute: make_push26,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH27, // 0x7A
         Operation {
             execute: make_push27,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH28, // 0x7B
         Operation {
             execute: make_push28,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH29, // 0x7C
         Operation {
             execute: make_push29,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH30, // 0x7D
         Operation {
             execute: make_push30,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH31, //0x7E
         Operation {
             execute: make_push31,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::PUSH32, // 0x7F
         Operation {
             execute: make_push32,
             memory_size: None,
             min_stack: crate::stack::min_stack(&0i16, &1i16),
             max_stack: crate::stack::max_stack(&0i16, &1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP1, // 0x80
         Operation {
             execute: make_dup1,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&1i16),
             max_stack: crate::stack::max_dup_stack(&1i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP2, // 0x81
         Operation {
             execute: make_dup2,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&2i16),
             max_stack: crate::stack::max_dup_stack(&2i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP3, // 0x82
         Operation {
             execute: make_dup3,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&3i16),
             max_stack: crate::stack::max_dup_stack(&3i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP4, // 0x83
         Operation {
             execute: make_dup4,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&4i16),
             max_stack: crate::stack::max_dup_stack(&4i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP5, // 0x84
         Operation {
             execute: make_dup5,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&5i16),
             max_stack: crate::stack::max_dup_stack(&5i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP6, // 0x85
         Operation {
             execute: make_dup6,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&6i16),
             max_stack: crate::stack::max_dup_stack(&6i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP7, // 0x86
         Operation {
             execute: make_dup7,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&7i16),
             max_stack: crate::stack::max_dup_stack(&7i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP8, // 0x87
         Operation {
             execute: make_dup8,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&8i16),
             max_stack: crate::stack::max_dup_stack(&8i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP9, // 0x88
         Operation {
             execute: make_dup9,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&9i16),
             max_stack: crate::stack::max_dup_stack(&9i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP10, // 0x89
         Operation {
             execute: make_dup10,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&10i16),
             max_stack: crate::stack::max_dup_stack(&10i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP11, // 0x8A
         Operation {
             execute: make_dup11,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&11i16),
             max_stack: crate::stack::max_dup_stack(&11i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP12, // 0x8B
         Operation {
             execute: make_dup12,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&12i16),
             max_stack: crate::stack::max_dup_stack(&12i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP13, // 0x8C
         Operation {
             execute: make_dup13,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&13i16),
             max_stack: crate::stack::max_dup_stack(&13i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP14, // 0x8D
         Operation {
             execute: make_dup14,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&14i16),
             max_stack: crate::stack::max_dup_stack(&14i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP15, // 0x8E
         Operation {
             execute: make_dup15,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&15i16),
             max_stack: crate::stack::max_dup_stack(&15i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::DUP16, // 0x8F
         Operation {
             execute: make_dup16,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&16i16),
             max_stack: crate::stack::max_dup_stack(&16i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP1, // 0x90
         Operation {
             execute: make_swap1,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&2i16),
             max_stack: crate::stack::max_dup_stack(&2i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP2, // 0x91
         Operation {
             execute: make_swap2,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&3i16),
             max_stack: crate::stack::max_dup_stack(&3i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP3, // 0x92
         Operation {
             execute: make_swap3,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&4i16),
             max_stack: crate::stack::max_dup_stack(&4i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP4, // 0x93
         Operation {
             execute: make_swap4,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&5i16),
             max_stack: crate::stack::max_dup_stack(&5i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP5, // 0x94
         Operation {
             execute: make_swap5,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&6i16),
             max_stack: crate::stack::max_dup_stack(&6i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP6, // 0x95
         Operation {
             execute: make_swap6,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&7i16),
             max_stack: crate::stack::max_dup_stack(&7i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP7, // 0x96
         Operation {
             execute: make_swap7,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&8i16),
             max_stack: crate::stack::max_dup_stack(&8i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP8, // 0x97
         Operation {
             execute: make_swap8,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&9i16),
             max_stack: crate::stack::max_dup_stack(&9i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP9, // 0x98
         Operation {
             execute: make_swap9,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&10i16),
             max_stack: crate::stack::max_dup_stack(&10i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP10, // 0x99
         Operation {
             execute: make_swap10,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&11i16),
             max_stack: crate::stack::max_dup_stack(&11i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP11, // 0x9A
         Operation {
             execute: make_swap11,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&12i16),
             max_stack: crate::stack::max_dup_stack(&12i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP12, // 0x9B
         Operation {
             execute: make_swap12,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&13i16),
             max_stack: crate::stack::max_dup_stack(&13i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP13, // 0x9C
         Operation {
             execute: make_swap13,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&14i16),
             max_stack: crate::stack::max_dup_stack(&14i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP14, // 0x9D
         Operation {
             execute: make_swap14,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&15i16),
             max_stack: crate::stack::max_dup_stack(&15i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP15, // 0x9E
         Operation {
             execute: make_swap15,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&16i16),
             max_stack: crate::stack::max_dup_stack(&16i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
        (crate::opcode::SWAP16, // 0x9F
         Operation {
             execute: make_swap16,
             memory_size: None,
             min_stack: crate::stack::min_dup_stack(&17i16),
             max_stack: crate::stack::max_dup_stack(&17i16),
             halts: false,
             jumps: false,
             writes: false,
             reverts: false,
             returns: false,
         }),
    ].iter().cloned().collect();
    return is;
}

// pub static mut instruction_set: HashMap<u8, Operation> = get_instruction_set();

pub fn get_operation(_v: u8) -> Option<Operation> {
    let opc = _v as OpCode;
    let instset = get_instruction_set();
    let a = instset.get(&opc)
        .expect("invalid opcode");
    Some(a.clone())
}