#![no_main]

use bs58::encode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: [u8; 64]| {
    let correct = encode(data.clone()).into_string();
    let encoded = fd_bs58::encode_64(data);
    let decoded = fd_bs58::decode_64(encoded.clone()).unwrap();

    // check encoding matches
    if correct != encoded {
        panic!("encode_64 fuzz encoding failed: {:?}, {:?}", correct, encoded);
    }

    // check round trip
    if decoded != data {
        panic!("encode_64 round trip failed: {:?}, {:?}", data, decoded);
    }
});
