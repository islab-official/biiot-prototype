use network::urpc::URPCNetwork;
use network::p2p::P2pService;

pub struct Biiot {
    ledger: Ledger,
    urpc: URPCNetwork,
    p2p: P2pService,
    evm: Evm,
}

impl Biiot {
    pub fn new() -> Self {
        Biiot {}
    }
}