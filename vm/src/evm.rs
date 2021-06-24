use crate::interpreter::Interpreter;
use crate::contract::Contract;
use std::sync::Arc;
use ledger::ledger::Ledger;

pub struct VirtualMachine {
    ledger: Arc<Ledger>,
    interpreter: Option<Interpreter>,
    interpreters: Vec<Interpreter>,
    contract: Contract
}

impl VirtualMachine {
    pub fn new(ledger: Arc<Ledger>) -> Self {
        let vm = VirtualMachine{
            ledger,
            interpreter: None,
            interpreters: Vec::new(),
            contract: Default::default()
        };
        vm
    }
}

impl VirtualMachine {
    // pub fn set_contract(&mut self, _contract: Contract) {
    //     self.contract = _contract;
    // }
}