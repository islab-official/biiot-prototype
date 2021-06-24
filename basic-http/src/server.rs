use std::net::{SocketAddr, Ipv4Addr, SocketAddrV4, TcpStream, Shutdown};
use std::collections::HashMap;
use std::str::FromStr;
use std::io::Read;
use crate::request::{HttpRequest, RawRequest};
use crate::response::HttpResponse;

pub type Handler = fn(request: HttpRequest, response: HttpResponse);

pub struct HttpServer {
    address: SocketAddr,
    handlers: HashMap<String, Handler>,
    options: HashMap<String, Box<dyn std::any::Any>>,
}

impl HttpServer {
    pub fn new(ip: &str, port: u16, options: Option<HashMap<String, Box<dyn std::any::Any>>>) -> Self {
        let ip = Ipv4Addr::from_str(ip).unwrap();
        let ipv4 = SocketAddrV4::new(ip, port);
        let socket = SocketAddr::from(ipv4);
        let mut server = HttpServer {
            address: socket,
            handlers: HashMap::new(),
            options: HashMap::new()
        };
        if options.is_some() {
            for option in options.unwrap() {
                server.options.insert(option.0, option.1);
            }
        }
        return server;
    }

    pub fn append_handler(&mut self, path: &str, handler: Handler) -> Result<(), ()> {
        if self.handlers.contains_key(path) {
            return Err(());
        }
        self.handlers.insert(path.to_string(), handler);
        return Ok(());
    }

    pub fn bind(&self) {
        let listener = std::net::TcpListener::bind(self.address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0u8;8096];
        let request_length = stream.read(&mut buffer).unwrap();
        let raw_request = RawRequest::new(buffer, request_length);
        let result_request = HttpRequest::from_raw_request(raw_request);
        if result_request.is_err() {
            stream.shutdown(Shutdown::Both);
            return;
        }
        let request = result_request.unwrap();
        let response = HttpResponse::from_stream(stream);
        let function = self.handlers.get(request.path());
        match function {
            None => {
                // println!("{}", request.to_string());
            }
            Some(function) => {
                function(request, response);
            }
        }
    }
}
