#![no_main]

use libfuzzer_sys::fuzz_target;
use bs58::encode;

fuzz_target!(|data: [u8; 64]| {
    let normal = encode(data.clone()).into_string();
    let fd = fd_bs58::encode_64(data);

    if normal != fd {
        panic!("Encode 64 fuzz test failed!: {:?}, {:?}", normal, fd);
    }
});
