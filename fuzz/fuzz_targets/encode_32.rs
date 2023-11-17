#![no_main]

use bs58::encode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: [u8; 32]| {
    let correct = encode(data.clone()).into_string();
    let encoded = fd_bs58::encode_32(data);
    let decoded = fd_bs58::decode_32(encoded.clone()).unwrap();

    // check encoding matches
    if correct != encoded {
        panic!("encode_32 fuzz encoding failed: {:?}, {:?}", correct, encoded);
    }

    // check round trip
    if decoded != data {
        panic!("encode_32 round trip failed: {:?}, {:?}", data, decoded);
    }
});
