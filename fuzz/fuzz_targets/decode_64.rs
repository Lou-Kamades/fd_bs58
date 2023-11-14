#![no_main]

use libfuzzer_sys::fuzz_target;
use bs58::decode;

fuzz_target!(|data: &[u8] | {
    if data.len() >= 64 && data.len() <= 88 {
        if let Ok(s) = std::str::from_utf8(data) {

            let fd = fd_bs58::decode_64(s);
            let normal = decode(s).into_vec();

            if fd.is_err() && !normal.is_err() {
                let bytes = normal.unwrap();
                if bytes.len() == 64 { // other library can decode things that aren't 64 bytes
                    panic!("Decode 64 Fd errored when bs58 was ok: {:?}, {:?}", bytes, fd);   
                }
            } else if normal.is_err() && !fd.is_err() {
                panic!("Decode 64 bs58 errored when Fd was ok: {:?}, {:?}", normal, fd);
            } else if normal.is_err() && fd.is_err() {
                // good
            } else {
                let normal_result = normal.unwrap();
                let fd_result = fd.unwrap();
                if normal_result.as_slice() != fd_result {
                    panic!("Decode 64 fuzz test failed: {:?}, {:?}", normal_result.as_slice(), fd_result);
                }
            }
        }
    }
});
