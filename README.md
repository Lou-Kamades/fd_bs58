An optimized implementation of [Base58](https://en.wikipedia.org/wiki/Base58) encoding/decoding for 32 and 64 byte numbers.
This library is based off of the original C implementation from Jump Crypto's [Firedancer](https://github.com/firedancer-io/firedancer)
repo which can be found [here](https://github.com/firedancer-io/firedancer/pull/75). These algorithms are significantly faster than the commonly used
[`bs58`](https://github.com/Nullus157/bs58-rs) library for 32 and 64 bytes.

<br>

Performance vs. [`bs58`](https://github.com/Nullus157/bs58-rs) (run on an AMD Ryzen 7 3700X)

| Algorithm   | bs58 (ns) | fd_bs58 (ns) |
|-------------|-----------|--------------|
| `encode_32` | 999.66    | 112.58       |
| `encode_64` | 3842.3    | 289.57       |
| `decode_32` | 368.21    | 91.168       |
| `decode_64` | 1345.4    | 235.62       |

<br>
Rust crate: https://crates.io/crates/fd_bs58

## Development

To run the fuzzer: `cargo-fuzz run decode_32`

To run a benchmark: `cargo bench encode_32`
