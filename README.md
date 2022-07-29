# LN-VANITY-PUBKEY

Guesses millions of combinations to find a Lightning private key so the public key starts with the right combination.
Uses multi-threading to be as fast as possible.

## Speed

### Guesses

| CPU                                                                                 | Cores   | Guesses   |
|-------------------------------------------------------------------------------------|---------|-----------|
| [Macbook Pro 2020 Intel Core i5](https://support.apple.com/kb/SP819?locale=en_GB)   | 4 cores | 117,181/s |
| [Ryzen5 3600](https://www.hetzner.com/dedicated-rootserver/ax41)                    | 6 cores | 251,148/s |

### Pubkey letters

Time to estimate a pubkey start thats with x specific letters. 2 letters are 1 byte.
For this calculation, the processing power of the Ryzen5 3600 with 251,148 guesses per second has been used.

> Important: The first two letters of the pubkey are always 02 or 03. `ln-vanity-pubkey` try to guess these, only
> the letters after.

| Letter | Guesses required     | Time  |
|--------|----------------------|-------|
| 2      | 2^8 = 256            | <1s   |
| 4      | 2^16 = 65,536        | <1s   |
| 6      | 2^24 = 16.7 Millions | 1.11m |
| 8      | 2^32 = 4.3 Billions  | 4.75h |
| 10     | 2^40 = 1.1 Trillions | 50.6d |
| 12     | 2^48 = 281 Trillions | 35.5y |