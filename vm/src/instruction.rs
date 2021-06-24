use crate::interpreter::Interpreter;
use crate::context::CallContext;
use std::ops::{AddAssign, SubAssign, Deref, MulAssign, DivAssign, Not, BitAnd, Add, Div, Sub, Mul, BitOr, BitXor};
use ethereum_types::{U256, H256, Address, BigEndianHash};
use std::borrow::{BorrowMut, Borrow};
use crate::err::RunError;
use std::cell::{Cell, RefCell};
use crate::memory::get_data;
use crate::jumptable::ExecuteFn;
use crate::evm::VirtualMachine;

/// 27:: 0x01
pub fn op_add(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    // std::mem::replace(y.borrow_mut(), Box::new(U256::zero()));
    let res = x.get().add(y.get());
    let yself = y.get();
    y.get_mut().sub_assign(yself);
    y.get_mut().add_assign(res);
    // y.get_mut().add_assign(x.get());
    (None, None)
}

/// 33:: 0x03
pub fn op_sub(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    let res = x.get().sub(y.get());
    let yself = y.get();
    y.get_mut().sub_assign(yself);
    y.get_mut().add_assign(res);
    // y.get_mut().sub_assign(x.get());
    (None, None)
}

/// 39:: 0x02
pub fn op_mul(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    let res = x.get().mul(y.get());
    let yself = y.get();
    y.get_mut().sub_assign(yself);
    y.get_mut().add_assign(res);
    // y.get_mut().mul_assign(x.get());
    (None, None)
}

/// 45:: 0x04
pub fn op_div(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let mut x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    // print!("(x/y) :: ");
    let res = x.get().div(y.get());
    let yself = y.get();
    y.get_mut().sub_assign(yself); // make y as 0
    y.get_mut().add_assign(res);
    (None, None)
}

/// x/y, for signed numbers in tow's complement
// pub fn op_sdiv(_pc: &mut u64, _interpreter: &Interpreter, _call_context: &mut CallContext)
//               -> (Option<Vec<u8>>, Option<u8>) {
//     let x = _call_context.stack.pop().unwrap();
//     let mut y = _call_context.stack.peek().unwrap();
//     y.get().sdiv_assign(x.get());
//     (None, None)
// }

/// 57::
pub fn op_mod(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    let (value, rest) = y.get().div_mod(x.get());
    let cloy = y.get().clone();
    y.get_mut().sub_assign(cloy);
    y.get_mut().add_assign(value);
    (None, None)
}

/// x%y, for signed numbers in tow's complement
// pub fn op_smod(_pc: &mut u64, _interpreter: &Interpreter, _call_context: &mut CallContext)
//               -> (Option<Vec<u8>>, Option<u8>) {
//     let x = _call_context.stack.pop().unwrap();
//     let mut y = _call_context.stack.peek().unwrap();
//     y.get().sub_assign(x.get());
//     (None, None)
// }

/// 69::
pub fn op_exp(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let base = _call_context.stack.pop().unwrap();
    let mut exponent = _call_context.stack.peek().unwrap();
    let mut count = exponent.get().clone();
    let cloexp = exponent.get().clone();

    exponent.get_mut().sub_assign(cloexp);
    if count.is_zero() { exponent.get_mut().add_assign(U256::one()); }
    else {
        exponent.get_mut().add_assign(base.get());
        while true {
            count.sub_assign(U256::one());
            if count.is_zero() { break; }
            exponent.get_mut().mul_assign(base.get());
        }
    }
    (None, None)
}

/// 81::
pub fn op_not(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.peek().unwrap();
    x.get_mut().not();
    (None, None)
}

/// 87::
pub fn op_lt(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    if x.get() < y.get() {
        y.get_mut().mul_assign(U256::zero());
        y.get_mut().add_assign(U256::one());
    } else {
        y.get_mut().mul_assign(U256::zero());
    }
    (None, None)
}

/// 97::
pub fn op_gt(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    y.get_mut().mul_assign(U256::zero());
    if x.get() > y.get() {
        // if x is greater than y, set y as 1.
        y.get_mut().add_assign(U256::one());
    }
    (None, None)
}

/// 107::
pub fn op_slt(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    y.get_mut().mul_assign(U256::zero());
    if x.get() <= y.get() {
        // if x is smaller than y or same, set y as 1.
        y.get_mut().add_assign(U256::one());
    }
    (None, None)
}

/// 117::
pub fn op_sgt(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    y.get_mut().mul_assign(U256::zero());
    if x.get() >= y.get() {
        // if x is greater than y or same, set y as 1.
        y.get_mut().add_assign(U256::one());
    }
    (None, None)
}

/// 127::
pub fn op_eq(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    if x.get() == y.get() {
        y.get_mut().mul_assign(U256::zero());
        y.get_mut().add_assign(U256::one());
    } else {
        y.get_mut().mul_assign(U256::zero()); // y = 0
    }
    (None, None)
}

/// 137::
pub fn op_iszero(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.peek().unwrap();
    if x.get().is_zero() { x.get_mut().add_assign(U256::one()); } else {
        let clox = x.get().clone();
        x.get_mut().sub_assign(clox);
    }
    (None, None)
}

/// 147::
pub fn op_and(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let mut y = _call_context.stack.peek().unwrap();
    // let res = x.get().bitand(y.get());
    // let yself = y.get();
    // y.get_mut().sub_assign(res);
    // y.get_mut().add_assign(res);
    let band = y.get_mut().bitand(x.get());
    for idx in 0..4 { y.get_mut().0[idx] = band.0[idx]; }
    (None, None)
}

/// 153::
pub fn op_or(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    let bor = y.get_mut().bitor(x.get());
    for idx in 0..4 { y.get_mut().0[idx] = bor.0[idx]; }
    (None, None)
}

/// 159::
pub fn op_xor(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.pop().unwrap();
    let y = _call_context.stack.peek().unwrap();
    let bxor = y.get_mut().bitxor(x.get());
    for idx in 0..4 { y.get_mut().0[idx] = bxor.0[idx]; }
    (None, None)
}

/// 165::
pub fn op_byte(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
               -> (Option<Vec<u8>>, Option<RunError>) {
    let th = _call_context.stack.pop().unwrap();
    let val = _call_context.stack.peek().unwrap();
    let bbyte = val.get_mut().byte(th.get().as_usize());
    val.get_mut().mul_assign(U256::zero());
    val.get_mut().0[3] = bbyte as u64;
    (None, None)
}

/// 234::
pub fn op_sha3(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
               -> (Option<Vec<u8>>, Option<RunError>) {
    let offset = _call_context.stack.pop().unwrap();
    let size = _call_context.stack.peek().unwrap();
    let data = _call_context.memory.get_copy(
        offset.get().as_u64() as i64,
        size.get().as_u64() as i64,
    );

    // hasher not working properly.
    (None, None)
}

/// 254::
pub fn op_address(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    let addr = U256::from(_call_context.contract.address.as_bytes());
    let _ = _call_context.stack.push(&addr);
    (None, None)
}

/// 266::
// pub fn op_origin(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
//                  -> (Option<Vec<u8>>, Option<RunError>) {
//     _interpreter.
//     (None, None)
// }

/// 270::
pub fn op_caller(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    let addr_bytes = _call_context.contract.caller.as_bytes();
    let caller = U256::from(addr_bytes);
    _call_context.stack.push(&caller);
    (None, None)
}

/// 275::
pub fn op_callvalue(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                    -> (Option<Vec<u8>>, Option<RunError>) {
    _call_context.stack.push(&U256::zero());
    (None, None)
}

/// 281::
pub fn op_calldataload(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                       -> (Option<Vec<u8>>, Option<RunError>) {
    let x = _call_context.stack.peek().unwrap();
    // if x.get().0[1] == 0 && x.get().0[2] == 0 && x.get().0[3] == 0 {
    let mut data = get_data(&_call_context.contract.input, x.get().as_u64(), 32);
    // data.reverse(); // to little-endian..
    let u256data = U256::from_big_endian(&data.as_slice());
    x.get_mut().0[0] = u256data.0[0];
    x.get_mut().0[1] = u256data.0[1];
    x.get_mut().0[2] = u256data.0[2];
    x.get_mut().0[3] = u256data.0[3];
    // } else {
    //     x.get_mut().mul_assign(U256::zero()); // x = 0
    // }
    (None, None)
}

/// 292::
pub fn op_calldatasize(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                       -> (Option<Vec<u8>>, Option<RunError>) {
    let l = _call_context.contract.input.len() as u64;
    _call_context.stack.push(&U256::from(l));
    (None, None)
}

/// 297::
pub fn op_calldatacopy(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                       -> (Option<Vec<u8>>, Option<RunError>) {
    let mem_offset = _call_context.stack.pop().unwrap();
    let data_offset = _call_context.stack.pop().unwrap();
    let length = _call_context.stack.pop().unwrap();
    let mut data = data_offset.get().overflowing_neg();
    if data.1 {
        data.0.mul_assign(U256::zero());
        data.0.0[3] = 0xFFFFFFFFFFFFFFFF;
    }
    // These values are checked for overflow during gas cost calculation
    let memoff64 = mem_offset.get().as_u64();
    let l64 = length.get().as_u64();
    _call_context.memory.set(memoff64,
                             l64,
                             &get_data(&_call_context.contract.input,
                                       data_offset.get().as_u64(),
                                       l64)
    );
    (None, None)
}

/// 315::
pub fn op_returndatasize(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                         -> (Option<Vec<u8>>, Option<RunError>) {
    let rdatasize = _interpreter.return_data.len() as u64;
    let u256rdatasize = U256::from(rdatasize);
    _call_context.stack.push(&u256rdatasize);
    (None, None)
}

/// 320::
pub fn op_returndatacopy(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                         -> (Option<Vec<u8>>, Option<RunError>) {
    let memoffset = _call_context.stack.pop().unwrap();
    let mut dataoffset = _call_context.stack.pop().unwrap();
    let length = _call_context.stack.pop().unwrap();
    let offset64 = dataoffset.get().overflowing_neg();
    if offset64.1 { return (None, Some(crate::err::RunError::ReturnDataOutOfBounds))}

    dataoffset.get_mut().add_assign(length.get());
    let end64 = dataoffset.get().overflowing_neg();
    _call_context.memory.set(memoffset.get().as_u64(), length.get().as_u64(),
                             &_interpreter.return_data[offset64.0.as_usize()..end64.0.as_usize()].to_vec());
    (None, None)
}

/// 355::
pub fn op_codecopy(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    let mem_offset = _call_context.stack.pop().unwrap();
    let code_offset = _call_context.stack.pop().unwrap();
    let length = _call_context.stack.pop().unwrap();

    let mut u64code_offset = code_offset.get().as_u64();
    let overflow = false;
    if overflow { u64code_offset = 0xFFFFFFFFFFFFFFFF; }

    let codecopy = get_data(
        &_call_context.contract.code,
        u64code_offset,
        length.get().as_u64(),
    );
    _call_context.memory.set(mem_offset.get().as_u64(), length.get().as_u64(), &codecopy);
    (None, None)
}

pub fn op_pop(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
              -> (Option<Vec<u8>>, Option<RunError>) {
    _call_context.stack.pop().unwrap();
    (None, None)
}

/// 455::
pub fn op_coinbase(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    // let coinbase = Address::zero();
    _call_context.stack.push(&U256::from(0));
    (None, None)
}

/// 488::
pub fn op_mload(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                -> (Option<Vec<u8>>, Option<RunError>) {
    let mut v = _call_context.stack.peek().unwrap();
    let offset = v.get().as_u64() as i64;
    let clov = v.get().clone();
    v.get_mut().sub_assign(clov);
    let mut cv = _call_context.memory.get_copy(offset, 32).unwrap();
    // cv is big-endian style value. so we need to reverse it.
    cv.reverse();
    v.get_mut().add_assign(U256::from(cv.as_slice()));
    (None, None)
}

/// 495::
pub fn op_mstore(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    let offset = _call_context.stack.pop().unwrap();
    let val = _call_context.stack.pop().unwrap();
    // println!("mstore -> offset:{:x}, value:{:x}", &offset.get(), &val.get());
    _call_context.memory.print_memory();
    _call_context.memory.set32(offset.get().as_u64(), &val.get());
    _call_context.memory.print_memory();
    (None, None)
}

/// 524::
pub fn op_jump(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
               -> (Option<Vec<u8>>, Option<RunError>) {
    let pos = _call_context.stack.pop();
    // if !_call_context.contract.validate_jumpdest(&pos) {
    //     return (None, Some(RunError::InvalidJump));
    // }
    let pcself = _pc.get();
    _pc.get_mut().sub_assign(pcself);
    _pc.get_mut().add_assign(pos.unwrap().get().as_u64());
    (None, None)
}

/// 533::
pub fn op_jumpi(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                -> (Option<Vec<u8>>, Option<RunError>) {
    let mut pos = &_call_context.stack.pop().unwrap();
    let cond = &_call_context.stack.pop().unwrap();

    if !cond.get().is_zero() {
        if !_call_context.contract.valid_jumpdest(&pos.get()) {
            // println!("interpreter tried to jump pos:{:x}", pos.get());
            return (None, Some(RunError::InvalidJump));
        }
        let clopc = _pc.get().clone();
        _pc.get_mut().sub_assign(clopc);
        _pc.get_mut().add_assign(pos.get().as_u64());
    } else {
        _pc.get_mut().add_assign(1);
    }
    (None, None)
}

/// 546::
pub fn op_jumpdest(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    (None, None)
}

/// 587::
pub fn op_msize(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                -> (Option<Vec<u8>>, Option<RunError>) {
    let memlen = _call_context.memory.size() as u64;
    _call_context.stack.push(&U256::from(memlen));
    (None, None)
}

/// 674::
// pub fn op_call(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext, _size: u64, _push_byte_size: isize)
//               -> (Option<Vec<u8>>, Option<RunError>) {
//     let temp = _call_context.stack.pop().unwrap();
//     let addr = _call_context.stack.pop().unwrap();
//     let value = _call_context.stack.pop().unwrap();
//     let inoffset = _call_context.stack.pop().unwrap();
//     let insize = _call_context.stack.pop().unwrap();
//     let retoffset = _call_context.stack.pop().unwrap();
//     let retsize = _call_context.stack.pop().unwrap();
//
//     let toaddr = Address::from(H256::from_uint(&addr.get()));
//     let args = _call_context.memory.get_copy(
//         inoffset.get().as_u64() as i64,
//         insize.get().as_u64() as i64,
//     );
//
//     if !value.get().is_zero() {
//
//     }
//
//     let (ret, err) = _interpreter.call(&_call_context.contract, toaddr, args);
//
//     if err != RunError::NoError {
//
//     } else {
//
//     }
//     _call_context.stack.push(&temp.get());
//     if err == RunError::NoError || err == RunError::ExecutionReverted {
//         _call_context.memory.set(retoffset.get().as_u64(), retsize.get().as_u64(), ret);
//     }
//     (None, None)
// }

/// 799::
pub fn op_return(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    let offset = _call_context.stack.pop().unwrap();
    let size = _call_context.stack.pop().unwrap();
    let ret = _call_context.memory.get_copy(
        offset.get().as_u64() as i64,
        size.get().as_u64() as i64,
    );
    (ret, None)
}

/// 806::
pub fn op_revert(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    let offset = _call_context.stack.pop().unwrap();
    let size = _call_context.stack.pop().unwrap();
    let ret = _call_context.memory.get_copy(
        offset.get().as_u64() as i64,
        size.get().as_u64() as i64,
    );
    (ret, None)
}

/// 813::
pub fn op_stop(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
               -> (Option<Vec<u8>>, Option<RunError>) {
    (None, None)
}

// fn _make_log(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
//              -> (Option<Vec<u8>>, Option<RunError>) {
//     _call_context.stack.swap(_size as i16);
//     let mut topics = Vec::new();
//     for idx in 0.._size { topics.push(H256::zero()); }
//     let min_start = _call_context.stack.pop().unwrap();
//     let min_size = _call_context.stack.pop().unwrap();
//     for idx in 0.._size {
//         let addr = _call_context.stack.pop();
//         // topics[idx] = addr.byte32;
//     }
//
//     let d = _call_context.memory.get_copy(
//         min_start.get().as_u64() as i64,
//         min_size.get().as_u64() as i64,
//     );
//     // _interpreter.evm.stateDB.add_log( ... )
//     (None, None)
// }

/// 828::
fn make_log(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext, _size: isize)
            -> (Option<Vec<u8>>, Option<RunError>) {
    _call_context.stack.swap(_size as i16);
    let mut topics: Vec<H256> = Vec::new();
    // for idx in 0.._size { topics.push(H256::zero()); }

    let min_start = _call_context.stack.pop().unwrap();
    let min_size = _call_context.stack.pop().unwrap();

    for idx in 0.._size {
        let addr = _call_context.stack.pop().unwrap();
        topics.push(H256::from_uint(&addr.get()));
    }
    let d = _call_context.memory.get_copy(
        min_start.get().as_u64() as i64,
        min_size.get().as_u64() as i64,
    );

    println!("Event data check");
    for element in d.iter() {
        println!("Element");
        for ev in element {
            print!("{:02x} ", ev);
        }
        println!();
    }
    println!();
    (None, None)
}

pub fn make_log0(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_log(_pc, _interpreter, _call_context, 0)
}

pub fn make_log1(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_log(_pc, _interpreter, _call_context, 1)
}

pub fn make_log2(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_log(_pc, _interpreter, _call_context, 2)
}

pub fn make_log3(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_log(_pc, _interpreter, _call_context, 3)
}

pub fn make_log4(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_log(_pc, _interpreter, _call_context, 4)
}


/// 853::
pub fn op_push1(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                -> (Option<Vec<u8>>, Option<RunError>) {
    let codelen = _call_context.contract.code.len() as u64;
    let mut integer = U256::zero();
    _pc.get_mut().add_assign(1);

    if _pc.get() < codelen {
        let v = _call_context.contract.code[_pc.get() as usize] as u64;
        integer.0[0] = v;
        _call_context.stack.push(&integer);
    } else {
        _call_context.stack.push(&integer);
    }
    (None, None)
}

/// 868::
fn make_push(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext, _size: u64, _pushbsize: isize)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let codelen = _call_context.contract.code.len();

    let mut min_start = codelen.clone();
    let pcadd1 = (_pc.get() + 1) as usize;
    if pcadd1 < min_start { min_start = pcadd1.clone(); }

    let mut min_end = codelen.clone();
    let cmpson2 = min_start + (_pushbsize as usize);
    if cmpson2 < min_end { min_end = cmpson2; }

    let mut integer = U256::zero();
    let v = common::vecutil::right_pad_bytes(
        &_call_context.contract.code[min_start..min_end].to_vec(),
        _pushbsize as u64,
    );

    let mut a = v.clone();
    // a.reverse();
    integer = U256::from_big_endian(a.as_slice());
    _call_context.stack.push(&integer);
    _pc.get_mut().add_assign(_size);
    (None, None)
}

pub fn make_push2(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 2, 2)
}

pub fn make_push3(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 3, 3)
}

pub fn make_push4(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 4, 4)
}

pub fn make_push5(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 5, 5)
}

pub fn make_push6(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 6, 6)
}

pub fn make_push7(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 7, 7)
}

pub fn make_push8(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 8, 8)
}

pub fn make_push9(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 9, 9)
}

pub fn make_push10(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 10, 10)
}

pub fn make_push11(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 11, 11)
}

pub fn make_push12(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 12, 12)
}

pub fn make_push13(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 13, 13)
}

pub fn make_push14(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 14, 14)
}

pub fn make_push15(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 15, 15)
}

pub fn make_push16(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 16, 16)
}

pub fn make_push17(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 17, 17)
}

pub fn make_push18(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 18, 18)
}

pub fn make_push19(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 19, 19)
}

pub fn make_push20(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 20, 20)
}

pub fn make_push21(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 21, 21)
}

pub fn make_push22(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 22, 22)
}

pub fn make_push23(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 23, 23)
}

pub fn make_push24(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 24, 24)
}

pub fn make_push25(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 25, 25)
}

pub fn make_push26(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 26, 26)
}

pub fn make_push27(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 27, 27)
}

pub fn make_push28(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 28, 28)
}

pub fn make_push29(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 29, 29)
}

pub fn make_push30(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 30, 30)
}

pub fn make_push31(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 31, 31)
}

pub fn make_push32(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_push(_pc, _interpreter, _call_context, 32, 32)
}

/// 892::
fn make_dup(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext, _size: u64)
            -> (Option<Vec<u8>>, Option<RunError>) {
    _call_context.stack.dup(_size as i16);
    (None, None)
}

pub fn make_dup1(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 1)
}

pub fn make_dup2(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 2)
}

pub fn make_dup3(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 3)
}

pub fn make_dup4(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 4)
}

pub fn make_dup5(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 5)
}

pub fn make_dup6(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 6)
}

pub fn make_dup7(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 7)
}

pub fn make_dup8(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 8)
}

pub fn make_dup9(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                 -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 9)
}

pub fn make_dup10(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 10)
}

pub fn make_dup11(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 11)
}

pub fn make_dup12(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 12)
}

pub fn make_dup13(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 13)
}

pub fn make_dup14(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 14)
}

pub fn make_dup15(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 15)
}

pub fn make_dup16(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_dup(_pc, _interpreter, _call_context, 16)
}

/// 900::
fn make_swap(_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext, _size: u64)
             -> (Option<Vec<u8>>, Option<RunError>) {
    let mut size = _size + 1;
    _call_context.stack.swap(size as i16);
    (None, None)
}

// pub fn make_swap(_size: i64) -> ExecuteFn {
//     // swith n + 1 otherwise n would be swapped with n
//     let mut size = _size + 1;
//     return |_pc: &mut Cell<u64>, _interpreter: &Interpreter, _call_context: &mut CallContext|
//             -> (Option<Vec<u8>>, Option<RunError>) {
//         _call_context.stack.swap(_size as i16);
//         (None, None)
//     };
// }

pub fn make_swap1(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 1)
}

pub fn make_swap2(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 2)
}

pub fn make_swap3(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 3)
}

pub fn make_swap4(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 4)
}

pub fn make_swap5(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 5)
}

pub fn make_swap6(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 6)
}

pub fn make_swap7(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 7)
}

pub fn make_swap8(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 8)
}

pub fn make_swap9(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                  -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 9)
}

pub fn make_swap10(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 10)
}

pub fn make_swap11(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 11)
}

pub fn make_swap12(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 12)
}

pub fn make_swap13(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 13)
}

pub fn make_swap14(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 14)
}

pub fn make_swap15(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 15)
}

pub fn make_swap16(_pc: &mut Cell<u64>, _evm: &VirtualMachine, _interpreter: &Interpreter, _call_context: &mut CallContext)
                   -> (Option<Vec<u8>>, Option<RunError>) {
    make_swap(_pc, _interpreter, _call_context, 16)
}