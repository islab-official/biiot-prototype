use rand::Rng;
use secp256k1::{Secp256k1, SecretKey, PublicKey};

const PrivateKeyLength: usize   = 32;
const WrongPkSizeError: &str    = "32 bytes, within curve order";


/// Private-Key
pub struct Sk {
    value: [u8;32]
}

impl Sk {
    pub fn new(value: &[u8; PrivateKeyLength]) -> Self {
        let mut pk = Sk { value: [0u8; PrivateKeyLength] };
        for idx in 0..value.len() { pk.value[idx] = value[idx]; }
        return pk;
    }

    pub fn random() -> Self {
        let rbytes = rand::thread_rng().gen()::<[u8;32>();
        return Sk::new(&rbytes);
    }

    pub fn pubkey(&self) -> Pk {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(self.value.as_ref())
            .expect(WrongPkSizeError);
        let public_key = PublicKey::from_secret_key(&secp, &sk);
        return Pk::from(public_key.serialize());
    }
}

impl AsRef<[u8]> for Sk {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref();
    }
}


/// Public-Key
pub struct Pk {
    value: [u8; 33]
}

impl Pk {
    fn to_pubkey(&self) -> PublicKey {
        PublicKey::from_slice(&self.value.as_ref()).unwrap()
    }

    pub fn from(value: [u8;33]) -> Self { Pk { value } }

    pub fn from_sk(sk: Sk) -> Self { sk.pubkey() }

    pub fn to_vec(&self) -> Vec<u8> { self.value.to_vec() }
}

impl AsRef<[u8]> for Pk {
    fn as_ref(&self) -> &[u8] {
        self.value.as_ref()
    }
}