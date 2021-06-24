
use std::sync::{Arc, Mutex};
use ledger::ledger::Ledger;

mod p2p;
mod urpc;
mod rpc;
mod options;

fn main() {
    let rpc = Arc::new(crate::rpc::server::RpcServer::new("127.0.0.1", 8545));
    {
        // let rpc = rpc.clone();
        std::thread::spawn(move || {
            let rpc = crate::rpc::server::RpcServer::new("127.0.0.1", 8545);
            rpc.bind();
        });
    }
    // let ledger = Arc::new(Mutex::new(Ledger::new()));
    // let ip = "127.0.0.1";
    // let port: u16 = 8545;
    // let rpc = Arc::new(
    //     crate::rpc::engine::RpcService::new(ip, &port)
    // );
    //
    // // rpc.run_loop();
    // {
    //     let ledger = ledger.clone();
    //     let rpc = rpc.clone();
    //     std::thread::spawn(move || {
    //         rpc.bind();
    //     });
    // }
    loop {}
}