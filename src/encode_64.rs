use crate::constants::{
    BASE58_CHARS, BINARY_SZ_64, BYTE_COUNT_64, ENC_TABLE_64, INTERMEDIATE_SZ_64, R1_DIV,
    RAW58_SZ_64,
};

pub(crate) fn encode_64<I: AsRef<[u8]>>(input: I) -> String {
    let bytes: &[u8; 64] = input.as_ref().try_into().unwrap();
    // Count leading zeros
    let mut in_leading_0s = 0;
    while in_leading_0s < BYTE_COUNT_64 {
        if bytes[in_leading_0s] != 0 {
            break;
        }
        in_leading_0s += 1;
    }

    let mut binary: [u32; BINARY_SZ_64] = [0; BINARY_SZ_64];
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

    for i in 0..BINARY_SZ_64 {
        binary[i] = bytes_as_u32[i].to_be(); // Convert to big-endian (network byte order)
    }

    let mut intermediate: [u64; INTERMEDIATE_SZ_64] = [0; INTERMEDIATE_SZ_64];

    /* Convert to the intermediate format:
      X = sum_i intermediate[i] * 58^(5*(INTERMEDIATE_SZ-1-i))
    Initially, we don't require intermediate[i] < 58^5, but we do want
    to make sure the sums don't overflow. */

    /* If we do it the same way as the 32B conversion, intermediate[16]
    can overflow when the input is sufficiently large.  We'll do a
    mini-reduction after the first 8 steps.  After the first 8 terms,
    the largest intermediate[16] can be is 2^63.87.  Then, after
    reduction it'll be at most 58^5, and after adding the last terms,
    it won't exceed 2^63.1.  We do need to be cautious that the
    mini-reduction doesn't cause overflow in intermediate[15] though.
    Pre-mini-reduction, it's at most 2^63.05.  The mini-reduction adds
    at most 2^64/58^5, which is negligible.  With the final terms, it
    won't exceed 2^63.69, which is fine. Other terms are less than
    2^63.76, so no problems there. */

    for i in 0..8 {
        for j in 0..INTERMEDIATE_SZ_64 - 1 {
            intermediate[j + 1] += u64::from(binary[i]) * u64::from(ENC_TABLE_64[i][j]);
        }
    }

    /* Mini-reduction */
    intermediate[15] += intermediate[16] / R1_DIV;
    intermediate[16] %= R1_DIV;
    /* Finish iterations */

    for i in 8..BINARY_SZ_64 {
        for j in 0..INTERMEDIATE_SZ_64 - 1 {
            intermediate[j + 1] += u64::from(binary[i]) * u64::from(ENC_TABLE_64[i][j]);
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

    for i in (1..INTERMEDIATE_SZ_64).rev() {
        intermediate[i - 1] += intermediate[i] / R1_DIV;
        intermediate[i] %= R1_DIV;
    }

    let mut raw_base58: [u8; RAW58_SZ_64] = [0; RAW58_SZ_64];

    for i in 0..INTERMEDIATE_SZ_64 {
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

    /* Finally, actually convert to the string. We have to ignore all the
    leading zeros in raw_base58 and instead insert in_leading_0s
    leading '1' characters.  We can show that raw_base58 actually has
    at least in_leading_0s, so we'll do this by skipping the first few
    leading zeros in raw_base58. */

    let mut raw_leading_0s = 0;
    while raw_leading_0s < RAW58_SZ_64 {
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

    let mut out = String::with_capacity(RAW58_SZ_64);

    let skip = raw_leading_0s - in_leading_0s;
    let end = RAW58_SZ_64 - skip;
    for i in 0..end {
        let idx = raw_base58[skip + i];
        out.push(BASE58_CHARS[idx as usize]);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::encode_64;

    #[test]
    fn test_encode_64() {
        let keys = vec![
            "5eQS44iKV8B4b4gTt4tPZLPSHtD7F78fFDhbHDknsrAE1vUipnDf3pK6h5eZ8CqWqFgZPoYY6XHKUuvyt7BLWHpb",
            "4EZ6eZt7svb2gYEFFnf14KSpHMD9k6F57qjDwD7dDZhegkrn4e3EzoHNNV83Fjc9cN8BQgG2uRFGwDSivw9yk7Nx",
            "so5VqLRtAF6RxQJ4BSv31SPQfcFhUU1rqCroUJSLCWSEPhZqAEEwiTrH1kdndyztYbTCdmE7qKavgApDqVjmrKQ",
            "RSAtWLUiyEhWUrcBtqmFUgtBHQ2ghJz4poJdXyruFQJpbyfY9AQBfr3dZUP6xdBy7PRqzeXYGUsNai8gcEivZQL",
            "11cgTH4D5e8S3snD444WbbGrkepjTvWMj2jkmCGJtgn3H7qrPb1BnwapxpbGdRtHQh9t9Wbn9t6ZDGHzWpL4df",
            "1111111111111111111111111111111111111111111111111111111111111111", // [0; 64]
            "67rpwLCuS5DGA8KGZXKsVQ7dnPb9goRLoKfgGbLfQg9WoLUgNY77E2jT11fem3coV9nAkguBACzrU1iyZM4B8roQ" // [255; 64]
        ];

        for key in keys {
            let bytes = bs58::decode(key).into_vec().unwrap();
            let result = encode_64(bytes);
            assert_eq!(result, key.to_string());
        }
    }
}
