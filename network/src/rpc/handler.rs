use basic_http::request::HttpRequest;
use basic_http::response::HttpResponse;
use serde_json::Value;
use basic_http::status::HttpStatusCode;
use ledger::ledger::Ledger;
use crate::rpc::methods::ProcedureCall;

pub fn rpc_handler(request: HttpRequest, mut response: HttpResponse) {
    let data = request.body().data();
    let rpc_object: Value = serde_json::from_str(data).unwrap();
    let opt_rpc_method = rpc_object.get("method");
    let opt_rpc_id = rpc_object.get("id");
    let opt_rpc_params = rpc_object.get("params");

    if opt_rpc_method.is_none() && opt_rpc_id.is_none() && opt_rpc_params.is_none() {
        return
    }


    if opt_rpc_id.unwrap().as_u64().is_none() { return }
    let mut rpc_id = opt_rpc_id.unwrap().as_u64().unwrap();
    let rpc_method = opt_rpc_method.unwrap().as_str().unwrap();
    let rpc_params = opt_rpc_params.unwrap().as_array().unwrap();

    println!("{} :: {}", request.path(), rpc_method);
    let mut readonly_ledger = Ledger::new();

    // eth_chainId & net_version
    // eth_chainId & net_version
    // eth_gasPrice
    // eth_getTransactionCount
    match rpc_method {
        crate::rpc::method_names::WEB3CLIENT_VERSION => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
                crate::rpc::method_names::WEB3CLIENT_VERSION
            );
            let data = crate::rpc::methods::Web3ClientVersion::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::WEB3SHA3 => {}
        crate::rpc::method_names::NET_VERSION => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
                crate::rpc::method_names::NET_VERSION
            );
            let data = crate::rpc::methods::NetVersion::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::NET_LISTENING => {}
        crate::rpc::method_names::NET_PEER_COUNT => {}
        crate::rpc::method_names::ETH_PROTOCOL_VERSION => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
            crate::rpc::method_names::ETH_PROTOCOL_VERSION
            );
            let data = crate::rpc::methods::EthProtocolVersion::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_SYNCING => {}
        crate::rpc::method_names::ETH_COINBASE => {}
        crate::rpc::method_names::ETH_MINING => {}
        crate::rpc::method_names::ETH_HASHRATE => {}
        crate::rpc::method_names::ETH_GAS_PRICE => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
            crate::rpc::method_names::ETH_GAS_PRICE
            );
            let data = crate::rpc::methods::EthGasPrice::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_ACCOUNTS => {}
        crate::rpc::method_names::ETH_BLOCK_NUMBER => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
            crate::rpc::method_names::ETH_BLOCK_NUMBER
            );
            let data = crate::rpc::methods::EthBlockNumber::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_GET_BALANCE => {
            let rpc_request = crate::rpc::request::RpcStringsRequest::new(
                &rpc_id, "2.0",
                crate::rpc::method_names::ETH_GET_BALANCE,
                rpc_params
            );
            let data = crate::rpc::methods::EthGetBalance::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_STORAGE_AT => {}
        crate::rpc::method_names::ETH_GET_TX_COUNT => {
            let rpc_request = crate::rpc::request::RpcStringsRequest::new(
                &rpc_id, "2.0", crate::rpc::method_names::ETH_GET_TX_COUNT, &rpc_params
            );
            let data = crate::rpc::methods::EthGetTransactionCount::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_GET_BLOCK_TX_COUNT_BY_HASH => {}
        crate::rpc::method_names::ETH_GET_BLOCK_TX_COUNT_BY_NUMBER => {}
        crate::rpc::method_names::ETH_GET_UNCLE_COUNT_BY_BLOCK_HASH => {}
        crate::rpc::method_names::ETH_GET_UNCLE_COUNT_BY_BLOCK_NUMBER => {}
        crate::rpc::method_names::ETH_GET_CODE => {}
        crate::rpc::method_names::ETH_GET_SIGN => {}
        crate::rpc::method_names::ETH_SIGN_TX => {}
        crate::rpc::method_names::ETH_SEND_TX => {}
        crate::rpc::method_names::ETH_SEND_RAW_TX => {
            let rpc_request = crate::rpc::request::RpcStringsRequest::new(&rpc_id, "2.0",
            crate::rpc::method_names::ETH_SEND_RAW_TX, rpc_params);
            let mut num = 0;
            for i in rpc_params.iter() {
                let v = i.as_str().unwrap();
                println!("{}: {}", num, v);
                num += 1;
            }
            let data =crate::rpc::methods::EthSendRawTransaction::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        crate::rpc::method_names::ETH_CALL => {}
        crate::rpc::method_names::ETH_ESTIMATE_GAS => {
            let rpc_request = crate::rpc::request::RpcEmptyRequest::new(&rpc_id, "2.0",
            crate::rpc::method_names::ETH_ESTIMATE_GAS);
            let data = crate::rpc::methods::EthEstimatedGas::from(rpc_request)
                .receive(&mut readonly_ledger);
            response.set_code(HttpStatusCode::Ok);
            response.set_data(data.as_str());
        }
        &_ => {
            // Unknown method.. send 404
            response.set_code(HttpStatusCode::NotFound);
            response.set_data("");
        }
    }
    response.send();
}