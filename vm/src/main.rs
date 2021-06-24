mod constants;
mod context;
mod contract;
mod err;
mod evm;
mod instruction;
mod interpreter;
mod jumptable;
mod memory;
mod opcode;
mod stack;

use std::cell::{Cell, RefCell};
use std::fs::File;
use std::{fs, io};
use std::io::{Read, Error};
use crate::interpreter::Interpreter;
use ethereum_types::{Address, H256};
use crate::contract::Contract;
use std::collections::HashMap;
use std::net::{UdpSocket, SocketAddr};

pub fn get_file_as_vec(fname: &str) -> Vec<u8> {
    let mut f = File::open(&fname).unwrap();
    let meta = fs::metadata(&fname).unwrap();
    let mut buffer = vec![0; meta.len() as usize];
    f.read(&mut buffer).unwrap();
    buffer
}

pub fn deploy_contract(contract_bin_name: &str, raw_calldata: &Vec<u8>) -> Contract {
    let mut calldata = Cell::new(raw_calldata);
    let mut interpreter = Interpreter::new(Address::zero());
    let bs = get_file_as_vec(contract_bin_name);
    let deploy_ready_contract = RefCell::new(Contract {
        code: bs,   // smart contract bytecode
        input: calldata.get_mut().clone(),  // constructor parameters
        address: Address::zero(),   // smart contract address. in this time, it filled with zero
        caller: interpreter.origin.clone(),
        codehash: H256::random(),
        jump_points: HashMap::new(),
    });
    // return deploy_ready_contract.take();

    let (contract_code, dply_err) = interpreter.run_contract(&deploy_ready_contract, calldata.get_mut().to_vec());
    let deployed_contract = Contract {
        code: contract_code.unwrap(),
        input: calldata.get_mut().clone(),
        address: Address::zero(),
        caller: interpreter.origin.clone(),
        codehash: H256::random(),
        jump_points: HashMap::new(),
    };
    return deployed_contract;

}

pub fn run_function(deployed_contract: &RefCell<Contract>, raw_calldata: &Vec<u8>) {

    // FnSig(4b) Param1(32b)
    let mut calldata = Cell::new(raw_calldata);
    // let bs = get_file_as_vec(contract_bin_name);
    // print!("bytecode:: ({}bytes) ", bs.len());
    // for b in &bs { print!("{:x} ", b); }
    // println!();

    let mut interpreter = Interpreter::new(Address::zero());
    // let contract = Contract {
    //     code: bs,
    //     input: calldata.get_mut().clone(),
    //     address: Address::zero(),
    //     caller: interpreter.origin.clone(),
    //     codehash: H256::random(),
    //     jump_points: HashMap::new(),
    // };
    //
    // // Deploy smart contract
    // let (contract_code, dply_err) = interpreter.run_contract(contract, calldata.get_mut().to_vec());
    //
    // let deployed_contract = Contract {
    //     code: contract_code.unwrap(),
    //     input: calldata.get_mut().clone(),
    //     address: Address::zero(),
    //     caller: interpreter.origin.clone(),
    //     codehash: H256::random(),
    //     jump_points: HashMap::new(),
    // };

    // Run specific smart contract function
    let (result, rt_err) = interpreter.run_contract(deployed_contract, calldata.get_mut().to_vec());
}

fn main() {
    const event_contract: &str = "event_sample.code";
    const storage_contract: &str = "storage_sample.code";
    let deployed_contract = RefCell::new(deploy_contract(event_contract, &Vec::new()));
    let deployed_contract2 = RefCell::new(deploy_contract(event_contract, &Vec::new()));
    let myaddr = ethereum_types::Address::random();
    let lightaddr = ethereum_types::Address::random();
    let sonaraddr = ethereum_types::Address::random();
    let contractaddr = ethereum_types::Address::random();

    // let mut emit_log_zero_calldata: Vec<u8> = vec![
    //     0x3A, 0xA5, 0x66, 0xA9, // fnsig:: emitLogZero()
    // ];
    // run_function(&deployed_contract, &emit_log_zero_calldata);

    let mut emit_log_with_uint_calldata: Vec<u8> = vec![
        0x6D, 0x48, 0xDB, 0x17, // fnsig:: emitLogWithUint(uint256)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // uint256 a = 2
    ];
    // run_function(&deployed_contract, &emit_log_with_uint_calldata);

    let mut emit_log_with_uint2_calldata = vec![
        0x18, 0x7F, 0x2A, 0xE9, // fnsig:: emitLogWithUint2(uint256)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // uint256 a = 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, // uint256 a = 2
    ];
    // run_function(&deployed_contract, &emit_log_with_uint2_calldata);

    // let mut emit_log_with_idx_uint_calldata: Vec<u8> = vec![
    //     0x44, 0xCC, 0x31, 0x42, // fnsig:: emitLogWithIdxUint(uint256)
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // uint256 a = 2
    // ];
    // run_function(&deployed_contract, &emit_log_with_idx_uint_calldata);

    let mut enter_number_calldata: Vec<u8> = vec![
        0x35, 0x14, 0x0E, 0x02, // fnsig:: enterNumber(uint256)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, // uint256 a = 2
    ];

    let mut emit_log_with_str2_calldata: Vec<u8> = vec![
        0xab, 0x3b, 0x7b, 0x90,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x13,
        0x41, 0x6c, 0x69, 0x63, 0x65, 0x20, 0x69, 0x6e,
        0x20, 0x57, 0x6f, 0x6e, 0x64, 0x65, 0x72, 0x6c,
        0x61, 0x6e, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7b,
        0x27, 0x41, 0x6c, 0x69, 0x63, 0x65, 0x20, 0x69,
        0x6e, 0x20, 0x57, 0x6f, 0x6e, 0x64, 0x65, 0x72,
        0x6c, 0x61, 0x6e, 0x64, 0x27, 0x20, 0x69, 0x73,
        0x20, 0x61, 0x20, 0x66, 0x61, 0x69, 0x72, 0x79,
        0x74, 0x61, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72,
        0x20, 0x79, 0x6f, 0x75, 0x6e, 0x67, 0x20, 0x6b,
        0x69, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
        0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c,
        0x73, 0x6f, 0x20, 0x67, 0x6f, 0x6f, 0x64, 0x20,
        0x66, 0x6f, 0x72, 0x20, 0x61, 0x64, 0x75, 0x6c,
        0x74, 0x20, 0x77, 0x68, 0x6f, 0x20, 0x72, 0x65,
        0x61, 0x6c, 0x6c, 0x79, 0x20, 0x77, 0x61, 0x6e,
        0x74, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x6f, 0x20,
        0x69, 0x6e, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x6e,
        0x74, 0x61, 0x73, 0x79, 0x20, 0x77, 0x6f, 0x72,
        0x6c, 0x64, 0x2e, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    emit_log_with_str2_calldata = hex::decode("ab3b7b90000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000800000000000000000000000000000000000000000000000000000000000000013416c69636520696e20576f6e6465726c616e6400000000000000000000000000000000000000000000000000000000000000000000000000000000000000007b27416c69636520696e20576f6e6465726c616e6427206973206120666169727974616c6520666f7220796f756e67206b69647320616e6420697420697320616c736f20676f6f6420666f72206164756c742077686f207265616c6c792077616e7420746f20676f20696e746f2066616e7461737920776f726c642e0000000000").unwrap();

    let emit_log_with_str_calldata = hex::decode("59ef0ca900000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000013416c69636520696e20576f6e6465726c616e6400000000000000000000000000").unwrap();
    // run_function(&deployed_contract, &emit_log_with_str_calldata);

    println!();
    print!("[Commit] Contract Creation .. ");
    print!("TXID :: {} ..", ethereum_types::H256::random());
    println!("Contract Address :: {}", &contractaddr);
    let socket = UdpSocket::bind("0.0.0.0:8504").unwrap();
    socket.set_nonblocking(true);
    socket.set_broadcast(true);
    let mut processing_cmd = false;
    loop {
        let mut line = String::new();
        let mut buf = [0u8;4096];
        let (msg_size, _) = loop {
            match socket.recv_from(&mut buf){
                Ok(n) => break n,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    if !processing_cmd {
                        print!(">");
                        std::io::stdin().read_line(&mut line).unwrap();
                        processing_cmd = true;
                        let size = line.len();
                        line.remove(size - 1);
                        match line.as_str() {
                            "propagate(\"./light_on_tx\")" => {
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                socket.send_to(vec![0x20, 0x10].as_slice(), "255.255.255.255:8504");
                                dummy_commit_tx_message(&contractaddr);
                                run_function(&deployed_contract, &emit_log_with_uint2_calldata);
                                socket.send_to(vec![0x20, 0x30].as_slice(), "255.255.255.255:8504");
                            },
                            &_ => { println!("unknown request"); }
                        }
                        // socket.send_to(line.as_bytes(), "255.255.255.255:8504");
                    } else{}
                }
                Err(e) => { panic!("something goes wrong"); }
            }
        };
        let msg = &mut buf[..msg_size];
        if msg[0] == 0x10 && msg[1] == 0x01 { // machine control on!
            // processing_cmd = false;
            dummy_pending_tx_message(&ethereum_types::Address::from_slice(&[255;20]));
            // break;
        }
        // light off by sensor
        if msg[0] == 0x10 && msg[1] == 0x20 { run_function(&deployed_contract2, &emit_log_with_uint2_calldata); }
        for idx in 0..buf.len() { buf[idx] = 0; }
    }
    // socket.rec
}

fn dummy_pending_tx_message(contractaddr: &ethereum_types::Address) {
    println!("[Pending] Contract Execution .. to::{}", contractaddr);
}

fn dummy_commit_tx_message(contractaddr: &ethereum_types::Address) {
    print!("[Commit] Contract Execution .. to:{} .. TXID :: {}", contractaddr, ethereum_types::H256::random());
}

// fn internal_loop() {
//     let socket = UdpSocket::bind("127.0.0.1:10000").unwrap();
//     socket.set_nonblocking(true).unwrap();
//     socket.set_broadcast(true).unwrap();
//     let mut buf = [0u8;8096];
//     let (msg_size, _) = loop {
//         match socket.recv_from(&mut buf) {
//             Ok(n) => break n,
//             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
//
//             }
//             Err(e) => {}
//         }
//     };
//
// }