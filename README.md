An optimized implementation of [Base58](https://en.wikipedia.org/wiki/Base58) encoding/decoding for 32 and 64 byte numbers.
This library is based off of the original C implementation from Jump Crypto's [Firedancer](https://github.com/firedancer-io/firedancer)
repo which can be found [here](https://github.com/firedancer-io/firedancer/pull/75). These algorithms are significantly faster than the commonly used
[`bs58`](https://github.com/Nullus157/bs58-rs) library for 32 and 64 bytes.

https://crates.io/crates/fd_bs58

## Development

To run the fuzzer: `cargo-fuzz run decode_32`

To run a benchmark: `cargo bench encode_32`
