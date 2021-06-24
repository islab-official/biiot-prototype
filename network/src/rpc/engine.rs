use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr, TcpListener, TcpStream, Shutdown, Incoming};
use std::str::FromStr;
use std::convert::Infallible;
use std::collections::HashMap;
use std::io::{Error, Read, Write};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use ledger::ledger::Ledger;
use crate::rpc::methods::{Web3ClientVersion, ProcedureCall, Web3Sha3, NetVersion, EthBlockNumber, EthGetBalance};
use crate::rpc::request::{RpcStringsRequest, RpcEmptyRequest};

pub struct RpcService {
    addr: SocketAddr,
}

impl RpcService {
    pub fn new(_ip: &str, _port: &u16) -> Self {
        let ip = Ipv4Addr::from_str(_ip).unwrap();
        let ipv4 = SocketAddrV4::new(ip, _port.clone());
        let socket: SocketAddr = SocketAddr::from(ipv4);

        let rpc = RpcService {
            addr: socket,
        };
        return rpc;
    }

    pub fn bind(&self) {
        let listener = std::net::TcpListener::bind(self.addr).unwrap();
        // listener.set_nonblocking(true);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0u8;1024];
        let request_length = stream.read(&mut buffer).unwrap();
        let raw_request = String::from_utf8_lossy(&buffer[..request_length]);
        let mut request_split = raw_request.split("\r\n\r\n");
        let maybe_header = request_split.next();
        let maybe_body = request_split.next();
        let request_header = maybe_header.unwrap();
        let mut request_body = "";
        match maybe_body { // some broken message does not contain body
            None => { return }
            Some(_) => { request_body = maybe_body.unwrap(); }
        }
        println!("::{}", raw_request);
        if request_body.len() == 0 {
            println!("Response OPTIONS message");
            stream.write(crate::options::HTTP_OPTIONS.as_bytes());
            return
        }
        let json_body: Value = serde_json::from_str(request_body).unwrap();

        match json_body["method"].as_str() {
            None => {}
            Some(method_name) => {
                let mut read_only_ledger = Ledger::new();
                let mut id = 0;
                if json_body["id"].as_u64().is_some() { id = json_body["id"].as_u64().unwrap(); }
                else { return }

                match method_name {
                    crate::rpc::method_names::Web3ClientVersion => {
                        let request: Web3ClientVersion = Web3ClientVersion::new(&0x0001);
                            // WEB3CLIENT_VERSION::from(RpcEmptyRequest::from(&json_body));
                        let response = request.receive(&mut read_only_ledger, vec![]);
                        stream.write(response.as_bytes());
                    }
                    crate::rpc::method_names::Web3Sha3 => {
                        let request: Web3Sha3 = Web3Sha3::new(&0x0001, vec![]);
                    }
                    crate::rpc::method_names::NetVersion => {
                        let request = NetVersion::new(&id);
                        let response = request.receive(&mut read_only_ledger, vec![]);
                        println!("netVersion::{}", response.as_str());
                        stream.write(response.as_bytes());
                    }
                    crate::rpc::method_names::NetListening => {}
                    crate::rpc::method_names::NetPeerCount => {}
                    crate::rpc::method_names::EthProtocolVersion => {}
                    crate::rpc::method_names::EthSyncing => {}
                    crate::rpc::method_names::EthCoinbase => {}
                    crate::rpc::method_names::EthMining => {}
                    crate::rpc::method_names::EthHashrate => {}
                    crate::rpc::method_names::EthGasPrice => {}
                    crate::rpc::method_names::EthAccounts => {}
                    crate::rpc::method_names::EthBlockNumber => {
                        let request  = EthBlockNumber::new(&id);
                        let response = request.receive(&mut read_only_ledger, vec![]);
                        println!("blockNumber::{}", response.as_str());
                        stream.write(response.as_bytes());
                    }
                    crate::rpc::method_names::EthGetBalance => {
                        let addr = ethereum_types::Address::from_str("0x21d86d3d81c9b3d8577bb2bf579b9ba8aaef367a").unwrap();
                        let request = EthGetBalance::new(&id, &addr);
                        let response = request.receive(&mut read_only_ledger, vec![]);
                        println!("getBalance::{}", response.as_str());
                        stream.write(response.as_bytes());
                    }
                    crate::rpc::method_names::EthStorageAt => {}
                    crate::rpc::method_names::EthGetTransactionCount => {}
                    crate::rpc::method_names::EthGetBlockTxCountByHash => {}
                    crate::rpc::method_names::EthGetUncleCountByBlockNumber => {}
                    crate::rpc::method_names::EthGetUncleCountByBlockHash => {}
                    crate::rpc::method_names::EthGetUncleCountByBlockNumber => {}
                    crate::rpc::method_names::EthGetCode => {}
                    crate::rpc::method_names::EthGetSign => {}
                    // "eth_signTransaction" => {}
                    crate::rpc::method_names::EthSendTransaction => {}
                    crate::rpc::method_names::EthSendRawTransaction => {}
                    crate::rpc::method_names::EthCall => {}
                    crate::rpc::method_names::EthEstimateGas => {}
                    crate::rpc::method_names::EthGetBlockByHash => {}
                    crate::rpc::method_names::EthGetBlockByNumber => {}
                    crate::rpc::method_names::EthGetTransactionByHash => {}
                    "eth_getTransactionByBlockHashAndIndex" => {}
                    "eth_getTransactionByBlockNumberAndIndex" => {}
                    "eth_getTransactionReceipt" => {}
                    "eth_getUncleByBlockHashAndIndex" => {}
                    "eth_getUncleByBlockNumberAndIndex" => {}
                    "eth_getCompilers" => {}
                    "eth_compileSolidity" => {}
                    "eth_compileLLL" => {}
                    "eth_compileSerpent" => {}
                    "eth_newFilter" => {}
                    "eth_newBlockFilter" => {}
                    "eth_newPendingTransactionFilter" => {}
                    "eth_uninstallFilter" => {}
                    "eth_getFilterChanges" => {}
                    "eth_getFilterLogs" => {}
                    "eth_getLogs" => {}
                    "eth_getWork" => {}
                    "eth_submitWork" => {}
                    "eth_submitHashrate" => {}
                    "db_putString" => {}
                    "db_getString" => {}
                    "db_putHex" => {}
                    "db_getHex" => {}
                    &_ => {}
                }
                stream.shutdown(Shutdown::Both);
            }
        }
    }

    // pub async fn run_loop(&self) {
    //     for stream in self.listener.incoming() {
    //         match stream {
    //             Ok(mut stream) => {
    //                 std::thread::spawn(move || {
    //                     stream.write("Hello World".as_ref()).unwrap();
    //                     handle_client(stream)
    //                 });
    //             }
    //             Err(e) => { panic!("{}", e) }
    //         }
    //     }
    //     // drop(self.listener);
    // }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}


// async fn call_eth_storage_at(request: Request<Body>) -> Result<Response<Body>, Infallible> {
//     let rlp_data = hyper::body::to_bytes(request.into_body()).await.unwrap();
//     let eth_storage_at = rlp::decode::<MtdStorageAt>(rlp_data.as_ref());
//     Ok(Response::new(Body::from("Hello World")))
// }