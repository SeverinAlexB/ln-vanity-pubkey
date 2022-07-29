extern crate core;

use std::time::Instant;
use crate::derivation::node_keys2;
use crate::multithreading::{guess_pubkey_threaded};

mod derivation;
mod multithreading;


fn main() {
    let prefix = "F0F0F0";
    let thread_count = 4;

    println!("Start guessing pubkey with prefix {}.", prefix);
    println!("Use {} threads", thread_count);

    let start = Instant::now();


    let res = guess_pubkey_threaded(prefix, thread_count);

    let duration = start.elapsed();

    match res {
        Some(guess_result) => {
            println!("Guessing took {duration:?},  after {} guesses", guess_result.guesses);
            println!("{} guesses per second", guess_result.guesses/(duration.as_secs() as u128));
            let mnemonic = guess_result.mnemonic.expect("No mnemonic found.");
            let (pubkey, _) = node_keys2(mnemonic.to_entropy().as_slice());
            println!("Match prefix {} -> {}", prefix, pubkey);
            println!("Mnemonic: {}", mnemonic.to_string())
        },
        None => println!("Didn't find mnemonic")
    }

    // Ok(())
}
