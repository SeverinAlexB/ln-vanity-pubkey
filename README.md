# LN-VANITY-PUBKEY

Guesses millions of combinations to find a Core-Lightning seed so the public key starts with the right combination.
Multi-threaded, speed-optimized.

## Build

The [rust toolchain](https://doc.rust-lang.org/cargo/getting-started/installation.html) is required to build the project.

```bash
git clone https://github.com/SeverinAlexB/ln-vanity-pubkey.git
cd ln-vanity-pubkey
```

After that, you can run `cargo run --release -- --help`.

## Usage

```text
USAGE:
    ln-vanity-pubkey [OPTIONS] [prefix]

ARGS:
    <prefix>    Prefix in HEX.

OPTIONS:
    -h, --help                Print help information
    -t, --threads <NUMBER>    Set the number of threads used. Default is the number of 
                              thread supported on the machine. [default: 8]
    -V, --version             Print version information
```

### Example
```bash
> cargo run --release -- FFFFFF

Start guessing pubkey with prefix FFFFFF.
Use 8 threads
Finished threads

Guessing took 69.204873452s, 8785904 guesses
127331 guesses per second
Matched FFFFFF -> 03FFFFFF7F383CA8F171759320A6143CC34E4E3D5402942CCF6BC8050E8266C4D9
Mnemonic: witness penalty kingdom super street occur guilt room crumble absorb pulse awesome ordinary minimum flip rhythm romance siege motor excess lift jewel spike protect
CLN command: echo -n -e '\xfc\xb4\x51\xea\xec\xdd\x6f\x31\x99\xed\xdf\x34\xc0\x1a\xb6\x08\x49\xc5\x1a\x16\x45\xc6\xbb\x99\x02\x41\xa7\x48\x16\xef\xf4\x75' > hsm_secret
```

### Entropy

Make sure your machine has [enough entropy](https://blog.cloudflare.com/ensuring-randomness-with-linuxs-random-number-generator/). 
This linux command should return a number close to 4,096. Otherwise your keys will be weak.

```bash
cat /proc/sys/kernel/random/entropy_avail
```

### Add to CLN

Use the `CLN command` in the output of the result to write your new seed to the hsm_secret file. The hsm_secret is 
located at `.lightning/bitcoin/hsm_secret`. After the secret has been written, your node can be started as regular.

>  **🛑️** IMPORTANT: Do NOT replace your seed on an already existing node. This can lead to the loss of all funds!

Use the cli to check if the pubkey has been successfully applied.

```bash
> lightning-cli getinfo

{
   "id": "03FFFFFF7F383CA8F171759320A6143CC34E4E3D5402942CCF6BC8050E8266C4D9",
   "alias": "BIZARRENIGHT-.11.0.1-62-g92cc76a",
   "color": "02ff51",
   ....
}
```

## Speed

### Pubkey letters

Time to estimate a pubkey that starts with x specific letters. 2 letters are 1 byte.
For this calculation, the processing power of the Ryzen5 3600 with 251,148 guesses per second has been used.

> Important: The first two letters of the pubkey are always 02 or 03. `ln-vanity-pubkey` tries to guess only
> the letters after.

| Letter | Guesses required     | Time  |
|--------|----------------------|-------|
| 2      | 2^8 = 256            | <5s   |
| 4      | 2^16 = 65,536        | <5s   |
| 6      | 2^24 = 16.7 Millions | 1.11m |
| 8      | 2^32 = 4.3 Billions  | 4.75h |
| 10     | 2^40 = 1.1 Trillions | 50.6d |
| 12     | 2^48 = 281 Trillions | 35.5y |

### Guesses

| CPU                                                                                 | Cores   | Guesses   |
|-------------------------------------------------------------------------------------|---------|-----------|
| [Macbook Pro 2020 Intel Core i5](https://support.apple.com/kb/SP819?locale=en_GB)   | 4 cores | 117,181/s |
| [Ryzen5 3600](https://www.hetzner.com/dedicated-rootserver/ax41)                    | 6 cores | 251,148/s |
