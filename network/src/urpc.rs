use std::net::UdpSocket;
use std::io;

pub struct RPCMessage {}


pub struct URPCNetwork {
    socket: UdpSocket,
    methods: Vec<RPCMessage>
}

impl URPCNetwork {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:8504").unwrap();
        socket.set_broadcast(true);
        socket.set_nonblocking(true);
        URPCNetwork { socket, methods: vec![] }
    }

    pub async fn run_non_blocking_loop(&mut self) {
        loop {
            let mut buf = [0u8;8096];
            let (msg_size, _) = loop {
                match self.socket.recv_from(&mut buf) {
                    Ok(n) => break n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {

                    }
                    Err(e) => { panic!("[uRPC] encountered IO error {}", e) }
                }
            };
            let msg = &mut buf[..msg_size];
            // match
        }
    }
}