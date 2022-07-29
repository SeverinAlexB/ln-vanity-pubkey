use bip39::Mnemonic;
use crate::derivation::{is_key_match};
use bitcoin::{secp256k1::Secp256k1};
use bitcoin::hashes::hex::{FromHex};
use rand::Rng;
use futures::future::FutureExt;
use std::thread;
use std::time::Duration;


fn hex_string_to_u8(value: &str) -> Vec<u8> {
    Vec::from_hex(value).expect("Failed to convert string from hex")
}

pub struct GuessResult {
    pub prefix: String,
    pub mnemonic: Option<Mnemonic>,
    pub guesses: u128
}

pub fn guess_pubkey(prefix: &str) -> GuessResult {
    let secp_ctx = Secp256k1::new();
    let prefix_u8 = hex_string_to_u8(prefix);
    let prefix_slice = prefix_u8.as_slice();

    let mut generator = rand::thread_rng();

    let mut mnemonic: Option<Mnemonic> = None;
    let mut counter = 0;
    loop {
        let seed = generator.gen::<[u8; 32]>();
        let is_match = is_key_match(&seed, prefix_slice, &secp_ctx);
        counter +=1;
        if is_match {
            mnemonic = Some(Mnemonic::from_entropy(&seed).expect("Invalid mnemonic"));

            break;
        };
    };

    GuessResult{
        prefix: String::from(prefix),
        guesses: counter,
        mnemonic
    }
}

pub fn guess_pubkey_threaded(prefix: &str, thread_count: u16) -> Option<GuessResult> {
    let mut handles: Vec<std::thread::JoinHandle<GuessResult>> = vec![];

    for i in 0..thread_count {
        let prefix_clone = String::from(prefix);
        let handle = thread::spawn(move || {
            let pref = prefix_clone.as_str();
            println!("Start thread {}", i + 1);
            guess_pubkey(pref)
        });
        handles.push(handle)
    }

    let mut guess_result: Option<GuessResult> = None;
    loop {
        let finished_handles = handles.iter().filter(|handle| handle.is_finished()).count();
        if finished_handles == 0 {
            thread::sleep(Duration::from_secs(1));
            continue
        } else {
            handles.into_iter().for_each( |handle| {
                if handle.is_finished() {
                    let res = handle.join();
                    let mut guess = res.expect("No result");
                    guess.guesses *= thread_count as u128;
                    guess_result = Some(guess);

                    println!("Finished threads");
                    println!();
                }
            });
            break
        }
    };

    guess_result
}

#[cfg(test)]
mod tests {
    // use core::slice::SlicePattern;
    use bitcoin::hashes::hex::ToHex;
    use crate::derivation::node_keys2;
    use std::time::{Instant};
    use super::*;

    #[test]
    fn is_key_match_test() -> Result<(), ()> {
        let prefix = "F00DBABE";
        let start = Instant::now();
        let res = guess_pubkey(prefix);
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