use serde::{Serialize, Deserialize};
use serde_json::Value;

/// RPC 요청 메시지이며 params 값이 공백인 메시지
#[derive(Serialize, Deserialize)]
pub struct RpcEmptyRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    params: Vec<u8>
}

impl RpcEmptyRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str) -> Self {
        return RpcEmptyRequest {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: vec![],
        };
    }
}

/// RPC 요청 메시지이며 params 값의 타입이 String인 메시지
#[derive(Serialize, Deserialize)]
pub struct RpcStringsRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>
}

impl From<&Value> for RpcStringsRequest {
    fn from(js: &Value) -> Self {
        let id = js["id"].as_u64().unwrap();
        let jsonrpc = js["jsonrpc"].as_str().unwrap().to_string();
        let method = js["method"].as_str().unwrap().to_string();
        let arr = js["params"].as_array().unwrap();
        let mut params = Vec::<String>::new();
        for element in arr.iter() {
            let v = element.as_str().unwrap().to_string();
            params.push(v);
        }
        RpcStringsRequest { id, jsonrpc, method, params }
    }
}

impl RpcStringsRequest {
    pub fn new(id: &u64, jsonrpc: &str, method: &str, params: &Vec<Value>) -> Self {
        let mut str_params: Vec<String> = vec![];
        for element in params.iter() {
            str_params.push(element.as_str().unwrap().to_string());
        }
        return RpcStringsRequest {
            id: id.clone(),
            jsonrpc: jsonrpc.to_string(),
            method: method.to_string(),
            params: str_params
        };
    }
}

/// RPC 요청 메시지이며 params 값의 타입이 Integer인 메시지
#[derive(Serialize, Deserialize)]
pub struct RpcIntegersRequest {
    pub id: u64,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<usize>,
}

impl RpcIntegersRequest {
    pub fn new(id: &u64, jsonrpc: String, method: String, params: Vec<usize>) -> Self {
        RpcIntegersRequest { id: id.clone(), jsonrpc, method, params }
    }
}