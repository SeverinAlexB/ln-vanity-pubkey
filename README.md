# LN-VANITY-PUBKEY

Guesses millions of combinations to find a Lightning private key so the public key starts with the right combination.
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
> cargo run --release -- FFFF

Start guessing pubkey with prefix FFFF.
Use 1 threads
Finished threads

Guessing took 3.009831139s, 98845 guesses
32948 guesses per second
Matched FFFF -> 03FFFF0F7808D4092E0D4DBAD8C47A062B0247C3E21886E025BF585582D67BFD9F
Mnemonic: sad desk shield chief admit east project congress gap must captain fly page project spawn paddle theory fold neglect dial world husband frost day
```

### Add to CLN

TDB

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
