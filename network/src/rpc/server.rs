use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4};
use std::str::FromStr;
use basic_http::server::{HttpServer, Handler};
use std::collections::HashMap;
use crate::rpc::handler::rpc_handler;

pub struct RpcServer {
    server: HttpServer,
}

impl RpcServer {
    pub fn new(ip: &str, port: u16, ) -> Self {
        let mut server = HttpServer::new(ip, port, None);
        server.append_handler("/", rpc_handler);
        // let ip = Ipv4Addr::from_str(ip).unwrap();
        // let ipv4 = SocketAddrV4::new(ip, port);
        // let socket = SocketAddr::from(ipv4);
        RpcServer { server }
    }

    pub fn bind(&self) {
        self.server.bind();
    }
}