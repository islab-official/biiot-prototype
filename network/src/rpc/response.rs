use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::rpc::constants::RPC_VERSION;

const HTTP200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const HTTP404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

/// RPC 응답 메시지이며 result 값이 String인 메시지
/// 해당 메시지는 JSON: &str으로 변환되어 전송된다.
/// # Arguments
/// * `id` - 요청 메시지와 동일한 ID값을 가지며 리플레이 공격을 방지한다.
/// * `jsonrpc` - RPC의 버전을 의미하며 항상 "2.0"을 갖는다.
/// * `result` - 1
#[derive(Serialize, Deserialize)]
pub struct RpcStringResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: String,
}

impl RpcStringResponse {
    pub fn new(id: u64, result: &str) -> Self {
        RpcStringResponse {
            id,
            jsonrpc: RPC_VERSION.to_string(),
            result: result.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcBoolResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: bool,
}

impl RpcBoolResponse {
    pub fn new(id: u64, result: bool) -> Self {
        RpcBoolResponse {
            id,
            jsonrpc: RPC_VERSION.to_string(),
            result
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcStringArrayResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: Vec<String>,
}

impl RpcStringArrayResponse {
    pub fn new(id: u64, result: &Vec<String>) -> Self {
        let mut str_result = Vec::<String>::new();
        for element in result.iter() {
            str_result.push(element.clone());
        }
        RpcStringArrayResponse {
            id,
            jsonrpc: RPC_VERSION.to_string(),
            result: str_result
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RpcMapResponse {
    pub id: u64,
    pub jsonrpc: String,
    pub result: HashMap<String, String>,
}

impl RpcMapResponse {
    pub fn new(id: u64, result: &HashMap<String, String>) -> Self {
        let mut str_str_result = HashMap::<String, String>::new();
        for elements in result.iter() {
            str_str_result.insert(elements.0.clone(), elements.1.clone());
        }
        RpcMapResponse {
            id,
            jsonrpc: RPC_VERSION.to_string(),
            result: str_str_result,
        }
    }
}