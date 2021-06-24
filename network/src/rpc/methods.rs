use ethereum_types::{Address, U256, H256};
use rlp::{Decodable, Encodable, RlpStream, DecoderError, Rlp};
use serde::{Serialize, Deserialize};
use std::sync::{Mutex, Arc};
use ledger::ledger::Ledger;
use ledger::transaction::{RawTransaction, Transaction};
use std::collections::HashMap;
use crate::rpc::request::{RpcStringsRequest, RpcEmptyRequest};
use crate::rpc::response::{RpcStringResponse, RpcBoolResponse, RpcMapResponse, RpcStringArrayResponse};
use serde_json::Value;
use crate::rpc::method_names;
use std::fmt::Write;
use crate::rpc::constants::RPC_VERSION;

/// RPC에 대한 공통 행동이며 call(ledger) -> JsonStr을 갖는다.
pub trait ProcedureCall {
    /// 노드가 해당 RPC를 호출할 때 사용하는 메서드
    fn call(&self) -> String;
    /// 노드가 해당 RPC를 요청받을 때 사용하는 메서드
    fn receive(&self, ledger: &mut Ledger) -> String;
}


/// 운영되는 블록체인 노드의 버전을 제공하는 RPC
/// # Example
/// * "Biiot/v0.1.0/windows/rust1.52"
pub struct Web3ClientVersion(RpcEmptyRequest);

impl Web3ClientVersion {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::WEB3CLIENT_VERSION);
        return Web3ClientVersion { 0: request };
    }
}

impl From<RpcEmptyRequest> for Web3ClientVersion {
    fn from(request: RpcEmptyRequest) -> Self {
        Web3ClientVersion { 0: request }
    }
}

impl ProcedureCall for Web3ClientVersion {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    /// receive returns the current client version.
    /// String  - The current client version
    fn receive(&self, ledger: &mut Ledger) -> String {
        let result = "Biiot/v0.1.0/windows/rust1.52".to_string();
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string(&res).unwrap();
    }
}


/// 클라이언트가 전달하는 값을 keccak256 해싱하여 반환하는 RPC
/// # Example
/// Input : 0x0000...0000
/// Output: 290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563
pub struct Web3Sha3(RpcStringsRequest);

impl Web3Sha3 {
    pub fn new(id: &u64, data: Vec<u8>) -> Self {
        let mut str_data = String::new();
        write!(&mut str_data, "0x{}", hex::encode(data.as_slice())).unwrap();
        let params = vec![Value::from(str_data)];
        let request =
            RpcStringsRequest::new(id, RPC_VERSION, method_names::WEB3SHA3, &params);
        return Web3Sha3 { 0: request };
    }
}

impl From<RpcStringsRequest> for Web3Sha3 {
    fn from(request: RpcStringsRequest) -> Self {
        Web3Sha3 { 0: request }
    }
}

impl ProcedureCall for Web3Sha3 {
    fn call(&self) -> String {
        let request =
            RpcStringsRequest::new(&self.0.id, RPC_VERSION, method_names::WEB3SHA3, &vec![]);
        return serde_json::to_string::<RpcStringsRequest>(&request).unwrap();
    }

    /// receive returns Keccak-256 of the given data
    /// DATA    - the data to convert into a SHA3 hash
    fn receive(&self, ledger: &mut Ledger) -> String {
        let data = self.0.params.get(0).unwrap().as_bytes();
        let u8a32h = crypto::hash::keccak256(data);
        let result = hex::encode(u8a32h);
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string(&res).unwrap();
    }
}


/// 블록체인 네트워크의 version값을 반환하는 RPC
/// # Example
/// * 1 (Ethereum Mainnet)
/// * 9 (Biiot network version)
pub struct NetVersion(RpcEmptyRequest);

impl NetVersion {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::NET_VERSION);
        return NetVersion { 0: request };
    }
}

impl From<RpcEmptyRequest> for NetVersion {
    fn from(request: RpcEmptyRequest) -> Self {
        NetVersion { 0: request }
    }
}

impl ProcedureCall for NetVersion {
    fn call(&self) -> String {
        // let mut params = vec![];
        // for element in _params.iter() {
        //     let str = element.downcast_ref::<String>().unwrap();
        //     params.push(Value::from(str.clone()));
        // }
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let net_ver: String = "9".to_string();
        let res = RpcStringResponse::new(self.0.id, &net_ver);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 노드가 요청을 listening 하는 상태인지 반환하는 RPC
pub struct NetListening(RpcEmptyRequest);

impl NetListening {
    pub fn new(id: u64) -> Self {
        let request =
            RpcEmptyRequest::new(&id, RPC_VERSION, method_names::NET_LISTENING);
        return NetListening { 0: request };
    }
}

impl From<RpcEmptyRequest> for NetListening {
    fn from(request: RpcEmptyRequest) -> Self {
        NetListening { 0: request }
    }
}

impl ProcedureCall for NetListening {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let res = RpcBoolResponse::new(self.0.id, true);
        return serde_json::to_string::<RpcBoolResponse>(&res).unwrap();
    }
}


/// 해당 노드와 연결된 피어의 갯수를 반환하는 RPC
/// Biiot는 UDP 기반의 통신을 수행하므로 전달하는 값은 항상 0이다.
pub struct NetPeerCount(RpcEmptyRequest);

impl NetPeerCount {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::NET_PEER_COUNT);
        return NetPeerCount { 0: request };
    }
}

impl From<RpcEmptyRequest> for NetPeerCount {
    fn from(request: RpcEmptyRequest) -> Self {
        NetPeerCount { 0: request }
    }
}

impl ProcedureCall for NetPeerCount {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let value = "0x00";
        let res = RpcStringResponse::new(self.0.id, value);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 현재 수행되는 블록체인의 프로토콜 버전을 반환하는 RPC
/// 항상 십진수 54라는 값을 반환한다.
pub struct EthProtocolVersion(RpcEmptyRequest);

impl EthProtocolVersion {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::ETH_PROTOCOL_VERSION);
        return EthProtocolVersion { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthProtocolVersion {
    fn from(request: RpcEmptyRequest) -> Self {
        EthProtocolVersion { 0: request }
    }
}

impl ProcedureCall for EthProtocolVersion {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let proto_ver = "54".to_string();
        let res = RpcStringResponse::new(self.0.id, &proto_ver);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 현재 노드가 동기화를 수행중인지 반환하는 RPC
/// 이더리움은 {startingBlock, currentBlock, highestBlock} | boolean을 반환한다.
/// 본 블록체인에서는 모든 경우를 object 타입으로 반환한다.
pub struct EthSyncing(RpcEmptyRequest);

impl EthSyncing {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::ETH_SYNCING);
        return EthSyncing { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthSyncing {
    fn from(request: RpcEmptyRequest) -> Self {
        EthSyncing { 0: request }
    }
}

impl ProcedureCall for EthSyncing {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        // 본래는 sync 상태 여부에 따라 false, {startingBlock, currentBlock, highestBlock} 중
        // 하나를 반환해야 하지만 이를 판단하기 어려울 정도로 빠르게 업데이트 되므로 맵 타입만 반환한다.
        // 21.05.26 :: 컨트랙트 체인이 아닌 메인 체인에 대한 sync 값을 제공한다.
        let mut result = HashMap::<String, String>::new();
        result.insert("startingBlock".to_string(), "0x384".to_string());
        result.insert("currentBlock".to_string(), "0x386".to_string());
        result.insert("highestBlock".to_string(), "0x454".to_string());
        let res = RpcMapResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcMapResponse>(&res).unwrap();
    }
}


/// 노드가 가진 대표 account의 주소 값을 반환하는 RPC
pub struct EthCoinbase(RpcEmptyRequest);

impl EthCoinbase {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(&id, RPC_VERSION, method_names::ETH_COINBASE);
        return EthCoinbase { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthCoinbase {
    fn from(request: RpcEmptyRequest) -> Self {
        EthCoinbase { 0: request }
    }
}

impl ProcedureCall for EthCoinbase {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let account_result = accounts::DeviceAccount::read_account();
        return match account_result {
            Ok(account) => {
                let result =
                    format!("0x{}", hex::encode(account.address.as_bytes()));
                let res =
                    RpcStringResponse::new(self.0.id, result.as_str());
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
            Err(_) => {
                // 블록체인이 구성되지 않은 노드일 경우에 발생한다.
                let account = accounts::DeviceAccount::new();
                let result = format!("0x{}", hex::encode(account.address.as_bytes()));
                let res =
                    RpcStringResponse::new(self.0.id, result.as_str());
                serde_json::to_string::<RpcStringResponse>(&res).unwrap()
            }
        };
    }
}


/// 노드가 채굴중인 상태인지 확인하는 RPC
/// 모든 노드는 채굴 가능성을 가지므로 항상 true를 반환한다.
pub struct EthMining(RpcEmptyRequest);

impl EthMining {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::ETH_MINING);
        return EthMining { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthMining {
    fn from(request: RpcEmptyRequest) -> Self {
        EthMining { 0: request }
    }
}

impl ProcedureCall for EthMining {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let res = RpcBoolResponse::new(self.0.id, true);
        return serde_json::to_string::<RpcBoolResponse>(&res).unwrap();
    }
}


/// hashrate per second 값을 반환하는 RPC
/// 항상 0x00의 hashrate를 반환한다.
pub struct EthHashrate(RpcEmptyRequest);

impl EthHashrate {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(id, RPC_VERSION, method_names::ETH_HASHRATE);
        return EthHashrate { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthHashrate {
    fn from(request: RpcEmptyRequest) -> Self {
        EthHashrate { 0: request }
    }
}

impl ProcedureCall for EthHashrate {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let res = RpcStringResponse::new(self.0.id, "0x00");
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 현재 가스 가격을 반환하는 RPC
/// 항상 0x00의 gas price를 반환한다.
pub struct EthGasPrice(RpcEmptyRequest);

impl EthGasPrice {
    pub fn new(id: &u64) -> Self {
        let request = RpcEmptyRequest::new(&id, RPC_VERSION, method_names::ETH_GAS_PRICE);
        return EthGasPrice { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthGasPrice {
    fn from(request: RpcEmptyRequest) -> Self {
        EthGasPrice { 0: request }
    }
}

impl ProcedureCall for EthGasPrice {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let price = "0x00".to_string();
        let res = RpcStringResponse::new(self.0.id, &price);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 노드가 가진 모든 account의 address를 반환하는 RPC
/// 노드는 한 개의 account만 가지므로 coinbase와 동일한 역할을 수행한다.
pub struct EthAccounts(RpcEmptyRequest);

impl EthAccounts {
    pub fn new(id: &u64) -> Self {
        let request = RpcEmptyRequest::new(&id, RPC_VERSION, method_names::ETH_ACCOUNTS);
        return EthAccounts { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthAccounts {
    fn from(request: RpcEmptyRequest) -> Self {
        EthAccounts { 0: request }
    }
}

impl ProcedureCall for EthAccounts {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        todo!(); // accounts에서 현재 노드가 갖고 있는 account 정보 불러오기
        let result = vec![];
        let res = RpcStringArrayResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringArrayResponse>(&res).unwrap();
    }
}

/// 가장 최근 블록의 번호를 반환하는 RPC
/// 제안 모델은 가장 최근에 생성된 milestone의 height를 반환한다.
pub struct EthBlockNumber(RpcEmptyRequest);

impl EthBlockNumber {
    pub fn new(id: &u64) -> Self {
        let request =
            RpcEmptyRequest::new(&id, RPC_VERSION, method_names::ETH_BLOCK_NUMBER);
        return EthBlockNumber { 0: request };
    }
}

impl From<RpcEmptyRequest> for EthBlockNumber {
    fn from(request: RpcEmptyRequest) -> Self {
        EthBlockNumber { 0: request }
    }
}

impl ProcedureCall for EthBlockNumber {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        // todo!(); // 메인 체인의 최근 블록 번호를 ledger로부터 받아온다.
        let result = "0x0"; //"0x4b7";
        let res = RpcStringResponse::new(self.0.id, result);
        serde_json::to_string::<RpcStringResponse>(&res).unwrap()
    }
}


/// 특정 주소가 가진 balance를 반환하는 RPC
/// 항상 0x00의 balance를 반환한다.
pub struct EthGetBalance(RpcStringsRequest);

impl EthGetBalance {
    pub fn new(id: &u64, address: &Address) -> Self {
        let mut str_addr = String::new();
        write!(&mut str_addr, "0x{}", hex::encode(address.as_bytes()));
        let mut params = vec![Value::from(str_addr)];
        let request =
            RpcStringsRequest::new(
                &id,
                RPC_VERSION,
                method_names::ETH_GET_BALANCE,
                &params
            );
        return EthGetBalance { 0: request };
    }
}

impl From<RpcStringsRequest> for EthGetBalance {
    fn from(request: RpcStringsRequest) -> Self {
        EthGetBalance { 0: request }
    }
}

impl ProcedureCall for EthGetBalance {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        // 제안 솔루션은 화폐를 일절 사용하지 않는다.
        let res = RpcStringResponse::new(self.0.id, &"0x00".to_string());
        serde_json::to_string(&res).unwrap()
    }
}


/// 컨트랙트의 storage 값을 반환하는 RPC
/// [contract-address, variable-index]로 구성된다.
pub struct EthStorageAt(RpcStringsRequest);

impl EthStorageAt {
    pub fn new(id: &u64, contract_address: &Address, position: &H256) -> Self {
        let mut str_ca = String::new();
        write!(&mut str_ca, "0x{}", hex::encode(contract_address.as_bytes()));
        let mut str_pos = String::new();
        write!(&mut str_pos, "0x{}", hex::encode(position.as_bytes()));
        let mut params = vec![Value::from(str_ca), Value::from(str_pos)];

        let request =
            RpcStringsRequest::new(
                &id,
                RPC_VERSION,
                method_names::ETH_STORAGE_AT,
                &params
            );
        return EthStorageAt { 0: request };
    }
}

impl From<RpcStringsRequest> for EthStorageAt {
    fn from(request: RpcStringsRequest) -> Self {
        EthStorageAt { 0: request }
    }
}

impl ProcedureCall for EthStorageAt {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    /// call returns the value from a storage position at a given address.
    /// DATA    - 20bytes address of the storage
    /// QUANTITY- integer of the position in the storage
    fn receive(&self, ledger: &mut Ledger) -> String {
        // 해당하는 contract db파일에서 index에 해당하는 변수를 찾는다.
        let str_address = self.0.params.get(0).unwrap().split_at(2).1;
        let str_position = self.0.params.get(1).unwrap().split_at(2).1;
        let address = Address::from_slice(str_address.as_bytes());
        let index = H256::from_slice(str_position.as_bytes());
        let state = Ledger::account_state(&address);
        let result = state.get_storage_value(&index).value;
        // let node = ledger.get_account(&self.data);
        let res =
            RpcStringResponse::new(self.0.id, &hex::encode(result.as_bytes()));
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// 특정 account가 전송한 트랜잭션의 총합을 반환하는 RPC
pub struct EthGetTransactionCount(RpcStringsRequest);

impl EthGetTransactionCount {
    pub fn new(id: &u64, address: &Address) -> Self {
        let mut str_addr = String::new();
        write!(&mut str_addr, "0x{}", hex::encode(address.as_bytes()));
        let params = vec![Value::from(str_addr)];
        let request =
            RpcStringsRequest::new(
                id,
                RPC_VERSION,
                method_names::ETH_GET_TX_COUNT,
                &params
            );
        return EthGetTransactionCount { 0: request };
    }
}

impl From<RpcStringsRequest> for EthGetTransactionCount {
    fn from(_request: RpcStringsRequest) -> Self {
        EthGetTransactionCount { 0: _request }
    }
}

impl ProcedureCall for EthGetTransactionCount {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let str_address = self.0.params.get(0).unwrap().split_at(2).1;
        let hex_address = hex::decode(str_address).unwrap();
        let mut addr_slice = [0u8;20];
        for i in 0..20 {
            addr_slice[i] = hex_address.get(i).unwrap().clone();
        }
        let address = Address::from(addr_slice);
        let res = RpcStringResponse::new(self.0.id, "0x1");
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// account가 가진 code 영역을 반환하는 RPC
pub struct EthGetCode(RpcStringsRequest);

impl EthGetCode {
    pub fn new(id: &u64, contract_address: Address) -> Self {
        let mut str_ca = String::new();
        write!(&mut str_ca, "0x{}", hex::encode(contract_address.as_bytes()));
        let params = vec![Value::from(str_ca)];
        let request =
            RpcStringsRequest::new(
                &id,
                RPC_VERSION,
                method_names::ETH_GET_CODE,
                &params);
        return EthGetCode { 0: request };
    }
}

impl From<RpcStringsRequest> for EthGetCode {
    fn from(request: RpcStringsRequest) -> Self {
        EthGetCode { 0: request }
    }
}

impl ProcedureCall for EthGetCode {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let str_address = self.0.params.get(0).unwrap().split_at(2).1;
        let hex_address = hex::decode(str_address).unwrap();
        let address = Address::from_slice(hex_address.as_slice());
        let state = ledger.get_account(&address);
        let code = &state.get_node().codehash;
        let result = hex::encode(&code);
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}


/// DATA, The signed transaction data.
pub struct EthSendRawTransaction(RpcStringsRequest);

impl EthSendRawTransaction {
    pub fn new(id: &u64, vec_tx: Vec<u8>) -> Self {
        let mut str_tx = String::new();
        write!(&mut str_tx, "0x{}", hex::encode(vec_tx.as_slice()));
        let params = vec![Value::from(str_tx)];
        let request =
            RpcStringsRequest::new(
                &id,
                RPC_VERSION,
                method_names::ETH_SEND_RAW_TX,
                &params
            );
        return EthSendRawTransaction { 0: request };
    }
}

impl From<RpcStringsRequest> for EthSendRawTransaction {
    fn from(request: RpcStringsRequest) -> Self {
        EthSendRawTransaction { 0: request }
    }
}

impl ProcedureCall for EthSendRawTransaction {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        // 외부에서 받은 Raw Transaction은 Memory Pool로 이동된다.
        // Raw Transaction에는 function-call과 device-call이 존재한다.
        // device call의 경우 별도의 합의 없이 즉각적으로 업데이트 될 수 있다.
        let raw_tx = self.0.params.get(0).unwrap().split_at(2).1;
        let mut result = "0x".to_string();
        // for element in self.0.params.iter() {
        //     write!(&mut result, "{}", element);
        // }
        result.push_str("0000000000000000");
        result.push_str("0000000000000000");
        result.push_str("0000000000000000");
        result.push_str("0000000000000000");
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}

pub struct EthCall(RpcStringsRequest);

impl EthCall {
    pub fn new(id: &u64, sender: Address, receiver: Address, data: Vec<u8>) -> Self {
        let mut gas = String::new();
        write!(&mut gas, "0x{}", U256::zero().to_string());
        let mut gas_price = String::new();
        write!(&mut gas_price, "0x{}", U256::zero().to_string());
        let mut str_sender = String::new();
        write!(&mut str_sender, "0x{}", hex::encode(sender.as_bytes()));
        let mut str_receiver = String::new();
        write!(&mut str_receiver, "0x{}", hex::encode(receiver.as_bytes()));
        let params = vec![
            Value::from(str_sender),
            Value::from(str_receiver),
            Value::from(gas),
            Value::from(gas_price),
            Value::from("0x00"),
            Value::from(hex::encode(data.as_slice())),
        ];
        let request =
            RpcStringsRequest::new(&id, RPC_VERSION, method_names::ETH_CALL, &params);
        return EthCall { 0: request };
    }
}

impl From<RpcStringsRequest> for EthCall {
    fn from(request: RpcStringsRequest) -> Self {
        return EthCall { 0: request };
    }
}

impl ProcedureCall for EthCall {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let result = "";
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}

pub struct EthEstimatedGas(RpcEmptyRequest);

impl From<RpcEmptyRequest> for EthEstimatedGas {
    fn from(request: RpcEmptyRequest) -> Self {
        return EthEstimatedGas { 0: request };
    }
}

impl ProcedureCall for EthEstimatedGas {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcEmptyRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let result = "0x5208";
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}

pub struct EthGetBlockByHash(RpcStringsRequest);

impl From<RpcStringsRequest> for EthGetBlockByHash {
    fn from(request: RpcStringsRequest) -> Self {
        return EthGetBlockByHash { 0: request };
    }
}

impl ProcedureCall for EthGetBlockByHash {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let result = "0x00";
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}

pub struct EthGetBlockByNumber(RpcStringsRequest);

impl From<RpcStringsRequest> for EthGetBlockByNumber {
    fn from(request: RpcStringsRequest) -> Self {
        return EthGetBlockByNumber { 0: request };
    }
}

impl ProcedureCall for EthGetBlockByNumber {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let res = RpcStringResponse::new(self.0.id, &"".to_string());
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}

pub struct EthGetTransactionByHash(RpcStringsRequest);

impl From<RpcStringsRequest> for EthGetTransactionByHash {
    fn from(_: RpcStringsRequest) -> Self {
        todo!()
    }
}

impl ProcedureCall for EthGetTransactionByHash {
    fn call(&self) -> String {
        return serde_json::to_string::<RpcStringsRequest>(&self.0).unwrap();
    }

    fn receive(&self, ledger: &mut Ledger) -> String {
        let result = "";
        let res = RpcStringResponse::new(self.0.id, &result);
        return serde_json::to_string::<RpcStringResponse>(&res).unwrap();
    }
}