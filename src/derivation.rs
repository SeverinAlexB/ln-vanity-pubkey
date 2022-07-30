use bitcoin::{secp256k1, secp256k1::Secp256k1};
use bitcoin::secp256k1::{PublicKey, SecretKey};
use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine};
use bitcoin::hashes::sha256::Hash as BitcoinSha256;


fn hkdf_extract_expand(salt: &[u8], secret: &[u8], info: &[u8], output: &mut [u8]) {
    let mut hmac = HmacEngine::<BitcoinSha256>::new(salt);
    hmac.input(secret);
    let prk = Hmac::from_engine(hmac).into_inner();

    let mut t = [0; 32];
    let mut n: u8 = 0;

    for chunk in output.chunks_mut(32) {
        let mut hmac = HmacEngine::<BitcoinSha256>::new(&prk[..]);
        n = n.checked_add(1).expect("HKDF size limit exceeded.");
        if n != 1 {
            hmac.input(&t);
        }
        hmac.input(&info);
        hmac.input(&[n]);
        t = Hmac::from_engine(hmac).into_inner();
        chunk.copy_from_slice(&t);
    }
}

fn hkdf_sha256(secret: &[u8], info: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    hkdf_extract_expand(salt, secret, info, &mut result);
    result
}

pub fn node_keys(
    seed: &[u8],
    secp_ctx: &Secp256k1<secp256k1::All>,
) -> (PublicKey, SecretKey) {
    let node_private_bytes = hkdf_sha256(seed, "nodeid".as_bytes(), &[]);
    let node_secret_key = SecretKey::from_slice(&node_private_bytes).unwrap();
    let node_id = PublicKey::from_secret_key(&secp_ctx, &node_secret_key);
    (node_id, node_secret_key)
}

pub fn node_keys2(
    seed: &[u8]
) -> (PublicKey, SecretKey) {
    let secp_ctx = Secp256k1::new();
    let node_private_bytes = hkdf_sha256(seed, "nodeid".as_bytes(), &[]);
    let node_secret_key = SecretKey::from_slice(&node_private_bytes).unwrap();
    let node_id = PublicKey::from_secret_key(&secp_ctx, &node_secret_key);
    (node_id, node_secret_key)
}

pub fn is_key_match(seed: &[u8], prefix: &[u8], secp_ctx: &Secp256k1<secp256k1::All>) -> bool {
    let (node_id, _) = node_keys(seed, &secp_ctx);
    let mut node_id_bytes = node_id.serialize().to_vec();
    node_id_bytes.remove(0); // Remove recovery byte that is always 02 or 03.
    node_id_bytes.starts_with(prefix)
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use bip39::Mnemonic;
    // use bitcoin::hashes::hex;
    use bitcoin::hashes::hex::{ToHex, FromHex};

    use super::*;

    #[test]
    fn key_derivation_cln_test() -> Result<(), ()> {
        let words = "sad desk shield chief admit east project congress gap must captain fly page project spawn paddle theory fold neglect dial world husband frost day";
        let mnemonic = Mnemonic::from_str(words).expect("invalid mnemonic");
        let entropy = mnemonic.to_entropy();
        // println!("mnemonic: {}", mnemonic);
        let secp_ctx = Secp256k1::new();
        let (node_id, _) = node_keys(entropy.as_slice(), &secp_ctx);
        let node_id_bytes = node_id.serialize().to_vec();
        println!("Node id: {}", node_id_bytes.to_hex());
        assert_eq!(
            node_id_bytes.to_hex(),
            "02b0ee72bf9a9559359314f5e3928cac471d5f8475d47214bcad54f5acfc642088" // cln key for this mnemonic.
        );
        Ok(())
    }

    // #[test]
    // fn key_derivation_test_old() -> Result<(), ()> {
        // let secp_ctx = Secp256k1::new();
        // let (node_id, _) = node_keys(&[0u8; 32], &secp_ctx);
        // let node_id_bytes = node_id.serialize().to_vec();
        // assert_eq!(
        //     node_id_bytes.to_hex(),
        //     "02058e8b6c2ad363ec59aa136429256d745164c2bdc87f98f0a68690ec2c5c9b0b"
        // );
        // Ok(())
    // }

    // #[test]
    // fn is_key_match_test() -> Result<(), ()> {
    //     let secp_ctx = Secp256k1::new();
    //     let prefix = "02058e";
    //     let prefix_vec = Vec::from_hex(prefix.clone()).expect("");
    //     let prefix_u8 = prefix_vec.as_slice();
    //     let is_match = is_key_match(&[0u8; 32], prefix_u8, &secp_ctx);
    //
    //     assert!(is_match);
    //     Ok(())
    // }
    //
    // #[test]
    // fn is_key_match_false_test() -> Result<(), ()> {
    //     let secp_ctx = Secp256k1::new();
    //     let prefix = "03058e";
    //     let prefix_vec = Vec::from_hex(prefix.clone()).expect("");
    //     let prefix_u8 = prefix_vec.as_slice();
    //     let is_match = is_key_match(&[0u8; 32], prefix_u8, &secp_ctx);
    //
    //     assert!(!is_match);
    //     Ok(())
    // }
}