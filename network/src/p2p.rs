use std::net::{UdpSocket, Ipv4Addr};
use std::io;
use std::str::FromStr;

pub struct P2pService {
    socket: UdpSocket,
    commands: Vec<u8>
}

impl P2pService {
    pub fn new() -> Self {
        let ip = Ipv4Addr::from_str("").unwrap();
        let socket = UdpSocket::bind("0.0.0.0:8504").unwrap();
        socket.set_broadcast(true);
        socket.set_nonblocking(true);
        P2pService { socket, commands: vec![] }
    }

    pub async fn run_non_blocking_loop(&mut self) {
        loop {
            let mut buf = [0u8;8096];
            let (msg_size, _) = loop {
                match self.socket.recv_from(&mut buf) {
                    Ok(n) => break n,
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {

                    }
                    Err(e) => { panic!("[P2P] encountered IO error {}", e) }
                }
            };
            let msg = &mut buf[..msg_size];
            // match
        }
    }
}