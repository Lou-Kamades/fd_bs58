#![no_main]

use bs58::decode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 64 && data.len() <= 88 {
        if let Ok(s) = std::str::from_utf8(data) {
            let fd = fd_bs58::decode_64(s);
            let decoded = decode(s).into_vec();

            if fd.is_err() && !decoded.is_err() {
                let bytes = decoded.unwrap();
                if bytes.len() == 64 {
                    // other library can decode things that aren't 64 bytes
                    panic!(
                        "fd_bs58 errored when bs58 was ok: {:?}, {:?}",
                        bytes, fd
                    );
                }
            } else if decoded.is_err() && !fd.is_err() {
                panic!(
                    "bs58 errored when fd_bs58 was ok: {:?}, {:?}",
                    decoded, fd
                );
            } else if decoded.is_err() && fd.is_err() {
                // good
            } else {
                let decoded_result = decoded.unwrap();
                let fd_result = fd.unwrap();
                if decoded_result.as_slice() != fd_result {
                    panic!(
                        "decode_64 gave different result: {:?}, {:?}",
                        decoded_result.as_slice(),
                        fd_result
                    );
                }
            }
        }
    }
});
