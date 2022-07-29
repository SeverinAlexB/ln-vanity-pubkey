extern crate core;

use std::time::Instant;
use crate::derivation::node_keys2;
use crate::multithreading::guess_pubkey;

mod derivation;
mod multithreading;

fn main() {
    println!("Start guessing");
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

    // Ok(())
}
