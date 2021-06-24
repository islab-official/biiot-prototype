use crate::contract::Contract;
use ethereum_types::Address;
use crate::memory::{Memory, to_u64_size};
use crate::stack::{Stack, ReturnStack};
use crate::context::CallContext;
use crate::jumptable::{get_instruction_set, get_operation};
use crate::opcode::{OpCode, get_opcode_name};
use crate::err::RunError;
use crate::err::RunError::ExecutionReverted;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::ops::AddAssign;
use std::thread::sleep;
use std::time::Duration;
use crate::evm::VirtualMachine;
use std::thread;
use std::sync::Arc;

pub struct Interpreter {
    pub origin: Address,
    pub return_data: Vec<u8>,
    pub return_with_err: u8
}

impl Interpreter {
    pub fn new(_origin: Address) -> Self {
        let mut intp = Interpreter{
            origin: _origin,
            return_data: Vec::new(),
            return_with_err: 0
        };
        return intp;
    }
}

impl Interpreter {
    pub fn run_contract(&mut self, _contract: &RefCell<Contract>, _input: Vec<u8>) -> (Option<Vec<u8>>, Option<RunError>) {
        let mut op = 0u8;
        let mut memory = Memory::new();
        let mut stack = Stack::default();
        let mut rstack = ReturnStack::new();
        let mut call_context = CallContext{
            stack, memory, rstack,
            contract: _contract.take()
        };
        let mut pc = Cell::new(0u64);
        call_context.contract.input = _input;

        let mut steps = 0;
        loop {
            sleep(Duration::from_millis(100));
            steps += 1;
            if steps % 1000 == 0 { break; }
            print!("step: {}, ", steps);
            println!("pc: {}", pc.get());

            // Get the operation from the jump-table and validate the stack to ensure
            // there are enough stack items available to perform the operation.
            op = call_context.contract.get_byte(pc.get());
            let str_op = get_opcode_name(&op);
            println!("find opcode:{:x}({})", op, str_op);
            let operation = get_operation(op).unwrap();

            // validate stack (not implemented)

            // memory size evaluation (not implemented)
            let mut memory_size = 0u64;

            if !operation.memory_size.is_none() {
                let memfn = operation.memory_size.unwrap();
                let memsize_n_overflow = memfn(&call_context.stack);
                let memorysize_n_overflow = common::vecutil::safe_mul(to_u64_size(memsize_n_overflow.0), 32);
                memory_size = memorysize_n_overflow.0;
            }

            if memory_size > 0 { call_context.memory.resize(memory_size); }

            // execute the operation
            let exec_fn = operation.execute;
            let ledger = Arc::new(ledger::ledger::Ledger::new());
            let tmp_evm = VirtualMachine::new(ledger);
            let (res, err) = exec_fn(&mut pc, &tmp_evm, self, &mut call_context);
            call_context.stack.print_stack();

            // thread::sleep(Duration::from_millis(50));

            let res_copy = res.clone();
            if operation.returns {
                if res.is_some() {
                    self.return_data = res.clone().unwrap();
                } else { self.return_with_err = 1; }
            }

            if err != None::<RunError> {
                println!("error:: contract stop");
                return (None, err);
            }
            else if operation.reverts {
                println!("operation:: revert");
                if res.is_some() {
                    return (Some(res.unwrap()), Some(ExecutionReverted));
                } else {
                    return (None, Some(ExecutionReverted));
                }
            }
            else if operation.halts {
                println!("operation:: halts");
                if !res.is_none() {
                    println!("::return value::");
                    common::printutil::print_u8vec(&res.clone().unwrap());
                }
                return (res, None);
            }
            else if !operation.jumps {
                println!("update program counter:: +1");
                pc.get_mut().add_assign(1);
            }
        }

        return (None, None)
        // return Err("unknown exception occurred");
    }
}