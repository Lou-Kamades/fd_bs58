use crate::constants::{
    BASE58_CHARS, BINARY_SZ_32, BYTE_COUNT_32, ENC_TABLE_32, INTERMEDIATE_SZ_32, R1_DIV,
    RAW58_SZ_32,
};

pub(crate) fn encode_32<I: AsRef<[u8]>>(input: I) -> String {
    let bytes: &[u8; 32] = input.as_ref().try_into().unwrap();
    // Count leading zeros
    let mut in_leading_0s = 0;
    while in_leading_0s < BYTE_COUNT_32 {
        if bytes[in_leading_0s] != 0 {
            break;
        }
        in_leading_0s += 1;
    }

    let mut binary: [u32; BINARY_SZ_32] = [0; BINARY_SZ_32];
    let bytes_as_u32: &[u32] = unsafe {
        // Cast a reference to bytes as a reference to u32
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const u32,
            bytes.len() / std::mem::size_of::<u32>(),
        )
    };

    /* X = sum_i bytes[i] * 2^(8*(BYTE_CNT-1-i)) */

    /* Convert N to 32-bit limbs:
    X = sum_i binary[i] * 2^(32*(BINARY_SZ-1-i)) */

    for i in 0..BINARY_SZ_32 {
        binary[i] = bytes_as_u32[i].to_be(); // Convert to big-endian (network byte order)
    }

    let mut intermediate: [u64; INTERMEDIATE_SZ_32] = [0; INTERMEDIATE_SZ_32];

    /* Convert to the intermediate format:
      X = sum_i intermediate[i] * 58^(5*(INTERMEDIATE_SZ-1-i))
    Initially, we don't require intermediate[i] < 58^5, but we do want
    to make sure the sums don't overflow. */

    /* The worst case is if binary[7] is (2^32)-1. In that case
    intermediate[8] will be be just over 2^63, which is fine. */

    for i in 0..BINARY_SZ_32 {
        for j in 0..INTERMEDIATE_SZ_32 - 1 {
            intermediate[j + 1] += u64::from(binary[i]) * ENC_TABLE_32[i][j];
        }
    }

    /* Now we make sure each term is less than 58^5. Again, we have to be
    a bit careful of overflow.
    For N==32, in the worst case, as before, intermediate[8] will be
    just over 2^63 and intermediate[7] will be just over 2^62.6.  In
    the first step, we'll add floor(intermediate[8]/58^5) to
    intermediate[7].  58^5 is pretty big though, so intermediate[7]
    barely budges, and this is still fine.
    For N==64, in the worst case, the biggest entry in intermediate at
    this point is 2^63.87, and in the worst case, we add (2^64-1)/58^5,
    which is still about 2^63.87. */

    for i in (1..INTERMEDIATE_SZ_32).rev() {
        intermediate[i - 1] += intermediate[i] / R1_DIV;
        intermediate[i] %= R1_DIV;
    }

    let mut raw_base58: [u8; RAW58_SZ_32] = [0; RAW58_SZ_32];

    for i in 0..INTERMEDIATE_SZ_32 {
        /* We know intermediate[ i ] < 58^5 < 2^32 for all i, so casting to
        a uint is safe.  GCC doesn't seem to be able to realize this, so
        when it converts ulong/ulong to a magic multiplication, it
        generates the single-op 64b x 64b -> 128b mul instruction.  This
        hurts the CPU's ability to take advantage of the ILP here. */
        let v = intermediate[i] as u32;
        raw_base58[5 * i + 4] = (v % 58) as u8;
        raw_base58[5 * i + 3] = (v / 58 % 58) as u8;
        raw_base58[5 * i + 2] = (v / 3364 % 58) as u8;
        raw_base58[5 * i + 1] = (v / 195112 % 58) as u8;
        raw_base58[5 * i] = (v / 11316496) as u8; // This one is known to be less than 58
    }

    /* Finally, actually convert to the string.  We have to ignore all the
    leading zeros in raw_base58 and instead insert in_leading_0s
    leading '1' characters.  We can show that raw_base58 actually has
    at least in_leading_0s, so we'll do this by skipping the first few
    leading zeros in raw_base58. */

    let mut raw_leading_0s = 0;
    while raw_leading_0s < RAW58_SZ_32 {
        if raw_base58[raw_leading_0s] != 0 {
            break;
        }
        raw_leading_0s += 1;
    }

    /* It's not immediately obvious that raw_leading_0s >= in_leading_0s,
    but it's true.  In base b, X has floor(log_b X)+1 digits.  That
    means in_leading_0s = N-1-floor(log_256 X) and raw_leading_0s =
    RAW58_SZ-1-floor(log_58 X).  Let X<256^N be given and consider:
    raw_leading_0s - in_leading_0s =
      =  RAW58_SZ-N + floor( log_256 X ) - floor( log_58 X )
      >= RAW58_SZ-N - 1 + ( log_256 X - log_58 X ) .
    log_256 X - log_58 X is monotonically decreasing for X>0, so it
    achieves it minimum at the maximum possible value for X, i.e.
    256^N-1.
      >= RAW58_SZ-N-1 + log_256(256^N-1) - log_58(256^N-1)
    When N==32, RAW58_SZ is 45, so this gives skip >= 0.29
    When N==64, RAW58_SZ is 90, so this gives skip >= 1.59.
    Regardless, raw_leading_0s - in_leading_0s >= 0. */

    let mut out = String::with_capacity(44);

    let skip = raw_leading_0s - in_leading_0s;
    let end = RAW58_SZ_32 - skip;
    for i in 0..end {
        let idx = raw_base58[skip + i];
        out.push(BASE58_CHARS[idx as usize]);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::encode_32;

    #[test]
    fn test_encode_32() {
        let keys = vec![
            "XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr",
            "Awes4Tr6TX8JDzEhCZY2QVNimT6iD1zWHzf1vNyGvpLM",
            "DRpbCBMxVnDK7maPM5tGv6MvB3v1sRMC86PZ8okm21hy",
            "EgxVyTgh2Msg781wt9EsqYx4fW8wSvfFAHGLaJQjghiL",
            "EvnRmnMrd69kFdbLMxWkTn1icZ7DCceRhvmb2SJXqDo4",
            "Certusm1sa411sMpV9FPqU5dXAYhmmhygvxJ23S6hJ24",
            "1zfbgASTPZHoQ5DhqS5f2bnJk88rxMi137DmZowDztN",
            "11111111111111111111111111111111", // [0; 32]
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG", // [255; 32]
        ];

        for key in keys {
            let bytes = bs58::decode(key).into_vec().unwrap();
            let decoded: [u8; 32] = bytes.try_into().unwrap();
            let result = encode_32(&decoded);
            assert_eq!(result, key.to_string());
        }
    }
}
