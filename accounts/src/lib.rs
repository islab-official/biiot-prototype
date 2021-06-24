use std::io::{Read, Error};
use common::fileutil::new_file;

const ACCOUNT_FILE: &str = "device.account";

pub struct DeviceAccount {
    pub secret_key: crypto::key::Sk,
    pub address: ethereum_types::Address
}

impl DeviceAccount {
    pub fn new() -> Self {
        let sk = crypto::key::Sk::random();
        let address = ethereum_types::Address::from(sk.pubkey().address());
        Self::write_account(sk.as_ref().to_vec());
        return DeviceAccount { secret_key: sk, address };
    }

    pub fn read_account() -> Result<Self, ()> {
        let mut file = common::fileutil::get_file(ACCOUNT_FILE);
        if file.is_err() { return Err(()); }
        let data = common::fileutil::read_file(&mut file.unwrap());
        let mut u8l32data: [u8; 32] = [0u8; 32];
        if data.len() != 32 {
            println!("the size of data in `device.account` is not 32 bytes");
            return Err(());
        }
        for idx in 0..32 { u8l32data[idx] = data.get(idx).unwrap().clone(); }
        let sk = crypto::key::Sk::new(&u8l32data);
        let address = ethereum_types::Address::from(sk.pubkey().address());
        return Ok(DeviceAccount { address, secret_key: sk });
    }

    pub fn write_account(sk_vec: Vec<u8>) {
        let mut file = new_file(ACCOUNT_FILE);
        common::fileutil::write_file(&mut file, sk_vec);
    }

    pub fn store_account(&self) {
        let mut file = new_file(ACCOUNT_FILE);
        common::fileutil::write_file(&mut file, self.secret_key.as_ref().to_vec());
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
