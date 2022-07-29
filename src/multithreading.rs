use bip39::Mnemonic;
use crate::derivation::{is_key_match};
use bitcoin::{secp256k1::Secp256k1};
use bitcoin::hashes::hex::{FromHex};
use rand::Rng;


fn hex_string_to_u8(value: &str) -> Vec<u8> {
    Vec::from_hex(value).expect("Failed to convert string from hex")
}

pub fn guess_pubkey(prefix: &str, max_iterations: u128) -> Option<Mnemonic> {
    let secp_ctx = Secp256k1::new();
    let prefix_u8 = hex_string_to_u8(prefix);
    let prefix_slice = prefix_u8.as_slice();

    let mut generator = rand::thread_rng();

    let mut result: [u8; 32] = [0; 32];
    let mut result_found = false;
    for i in 0..max_iterations {
        let seed = generator.gen::<[u8; 32]>();
        // println!("{} Check seed {:?}", i, seed);
        let is_match = is_key_match(&seed, prefix_slice, &secp_ctx);

        if is_match {
            result = seed.clone();
            result_found = true;
            println!("Found after {} guesses", i);
            break;
        };
    };

    if result_found {
        let mnemonic = Mnemonic::from_entropy(&result).expect("invalid seed");
        Some(mnemonic)
    } else {
        None
    }

}



#[cfg(test)]
mod tests {
    // use core::slice::SlicePattern;
    use bitcoin::hashes::hex::ToHex;
    use crate::derivation::node_keys2;
    use std::time::{Duration, Instant};
    use super::*;

    #[test]
    fn is_key_match_test() -> Result<(), ()> {
        let prefix = "F00DBABE";
        let start = Instant::now();
        let res = guess_pubkey(prefix, 100000000);
        let duration = start.elapsed();
        println!("Guessing took {:?}", duration);
        match res {
            Some(mnemonic) => {
                let (pubkey, _) = node_keys2( mnemonic.to_entropy().as_slice());
                println!("Match prefix {} -> {}", prefix, pubkey);
                println!("Mnemonic: {}", mnemonic)
            },
            None => println!("Didn't find mnemonic")
        }

        Ok(())
    }

}