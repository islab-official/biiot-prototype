use secp256k1::{Message, Secp256k1, Signature, PublicKey, SecretKey};
use secp256k1::recovery::{RecoverableSignature, RecoveryId};
use crate::key::{Sk, Pk};

pub fn sign(sk: &Sk, hmsg: &[u8; 32]) -> [u8; 64] {
    let msg = Message::from_slice(hmsg.as_ref()).unwrap();
    let seckey = SecretKey::from_slice(sk.as_ref()).unwrap();
    return Secp256k1::new().sign(&msg, &seckey).serialize_compact();
}

pub fn verify(pk: &Pk, hmsg: &[u8; 32], sig: &[u8; 64]) -> bool {
    let msg = Message::from_slice(hmsg.as_ref()).unwrap();
    let signature = Signature::from_compact(sig.as_ref()).unwrap();
    let pubkey = PublicKey::from_slice(pk.as_ref()).unwrap();
    Secp256k1::new().verify(&msg, &signature, &pubkey).is_ok()
}

pub fn sign_recoverable(sk: &Sk, hmsg: &[u8; 32]) -> (i32, [u8; 64]) {
    let secret_key = SecretKey::from_slice(sk.as_ref()).unwrap();
    let msg = Message::from_slice(hmsg.as_ref()).unwrap();
    let (rec_id, signature) = Secp256k1::new()
        .sign_recoverable(&msg, &secret_key)
        .serialize_compact();
    return (rec_id.to_i32(), signature);
}

pub fn recover_from_sig(rec_id: i32, comp_sig: [u8; 64], hmsg: &[u8; 32]) -> Pk {
    let id = RecoveryId::from_i32(rec_id).unwrap();
    let msg = Message::from_slice(hmsg).unwrap();
    let signature =
        RecoverableSignature::from_compact(&comp_sig, id)
            .unwrap();
    let public_key = Secp256k1::new().recover(&msg, &signature).unwrap();
    let pk = Pk::from(public_key.serialize_uncompressed());

    return pk;
}

pub fn recover_from_vrs(hmsg: &[u8; 32], v: i32, r: [u8; 32], s: [u8; 32]) -> Pk {
    let id = RecoveryId::from_i32(v).unwrap();
    let msg = Message::from_slice(hmsg).unwrap();
    let mut compact_signature = [0u8; 64];
    // let mut cnt = 0;
    for idx in 0..32 {
        compact_signature[idx] = r[idx];
        compact_signature[32 + idx] = s[idx];
        // cnt += 1;
    }
    let signature =
        RecoverableSignature::from_compact(&compact_signature, id)
            .unwrap();
    let public_key =
        Secp256k1::new().recover(&msg, &signature)
            .unwrap();
    let pk = Pk::from(public_key.serialize_uncompressed());
    return pk;
}