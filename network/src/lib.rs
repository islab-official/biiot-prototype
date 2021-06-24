pub mod p2p;
pub mod urpc;
mod rpc;
mod options;

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use ledger::ledger::Ledger;
    use crate::p2p::P2pService;
    use crate::rpc::engine::RpcService;

    #[test]
    fn run_all_networks() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn run_rpc() {
        let ledger = Arc::new(Mutex::new(Ledger::new()));
        let ip = "127.0.0.1";
        let port: u16 = 8545;
        let rpc = Arc::new(
            RpcService::new(ip, &port)
        );

        // rpc.run_loop();
        {
            let ledger = ledger.clone();
            let rpc = rpc.clone();
            std::thread::spawn(move || {
                rpc.bind();
            });
        }
        loop {}
    }

    #[test]
    fn run_p2p() {
        let ip = "127.0.0.1";
        let port: u16 = 8504;
        // let p2p = Arc::new(P2pService::new(ip, &port));
    }
}
