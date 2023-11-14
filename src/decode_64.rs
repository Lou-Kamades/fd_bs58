use crate::{
    constants::{
        BASE58_ENCODED_64_LEN, BASE58_INVALID_CHAR, BASE58_INVERSE, BASE58_INVERSE_TABLE_OFFSET,
        BASE58_INVERSE_TABLE_SENTINEL, BINARY_SZ_64, BYTE_COUNT_64, DEC_TABLE_64,
        INTERMEDIATE_SZ_64, RAW58_SZ_64,
    },
    Error,
};

pub(crate) fn decode_64<I: AsRef<[u8]>>(input: I) -> Result<[u8; BYTE_COUNT_64], Error> {
    let encoded_bytes = input.as_ref();
    if encoded_bytes.len() > BASE58_ENCODED_64_LEN as usize {
        return Err(Error::InputTooLong);
    }

    /* Validate string and count characters before the nul terminator */
    let mut char_cnt: u64 = 0;
    for c in encoded_bytes.iter() {
        let idx: u64 = if c < &b'1' {
            u64::MAX
        } else {
            (*c as u64) - (BASE58_INVERSE_TABLE_OFFSET as u64)
        };
        let idx = std::cmp::min(idx, BASE58_INVERSE_TABLE_SENTINEL as u64);

        if BASE58_INVERSE[idx as usize] == BASE58_INVALID_CHAR {
            return Err(Error::InvalidCharacter);
        }

        char_cnt += 1;
    }

    /* X = sum_i raw_base58[i] * 58^(RAW58_SZ-1-i) */
    let mut raw_base58: [u8; RAW58_SZ_64] = [0; RAW58_SZ_64];

    /* Prepend enough 0s to make it exactly RAW58_SZ characters */

    let prepend_0 = RAW58_SZ_64 - char_cnt as usize;

    for j in 0..RAW58_SZ_64 {
        if j < prepend_0 {
            raw_base58[j] = 0;
        } else {
            raw_base58[j] = BASE58_INVERSE
                [(encoded_bytes[j - prepend_0] as usize) - BASE58_INVERSE_TABLE_OFFSET as usize];
        }
    }

    /* Convert to the intermediate format (base 58^5):
    X = sum_i intermediate[i] * 58^(5*(INTERMEDIATE_SZ-1-i)) */

    let mut intermediate: [u64; INTERMEDIATE_SZ_64] = [0; INTERMEDIATE_SZ_64];
    for i in 0..INTERMEDIATE_SZ_64 {
        intermediate[i] = (raw_base58[5 * i] as u64) * 11_316_496
            + (raw_base58[5 * i + 1] as u64) * 195_112
            + (raw_base58[5 * i + 2] as u64) * 3_364
            + (raw_base58[5 * i + 3] as u64) * 58
            + (raw_base58[5 * i + 4] as u64);
    }

    /* Using the table, convert to overcomplete base 2^32 (terms can be
    larger than 2^32).  We need to be careful about overflow.
    For N==32, the largest anything in binary can get is binary[7]:
    even if intermediate[i]==58^5-1 for all i, then binary[7] < 2^63.
    For N==64, the largest anything in binary can get is binary[13]:
    even if intermediate[i]==58^5-1 for all i, then binary[13] <
    2^63.998.  Hanging in there, just by a thread! */

    let mut binary: [u64; BINARY_SZ_64] = [0; BINARY_SZ_64];
    for j in 0..BINARY_SZ_64 {
        let mut acc: u64 = 0;
        for i in 0..INTERMEDIATE_SZ_64 {
            acc += intermediate[i] * DEC_TABLE_64[i][j];
        }
        binary[j] = acc;
    }

    /* Make sure each term is less than 2^32.
    For N==32, we have plenty of headroom in binary, so overflow is
    not a concern this time.
    For N==64, even if we add 2^32 to binary[13], it is still 2^63.998,
    so this won't overflow. */

    for i in (1..(BINARY_SZ_64)).rev() {
        binary[i - 1] += binary[i] >> 32;
        binary[i] &= 0xFFFFFFFF;
    }

    /* If the largest term is 2^32 or bigger, it means N is larger than
    what can fit in BYTE_CNT bytes.  This can be triggered, by passing
    a base58 string of all 'z's for example. */

    if binary[0] > 0xFFFFFFFF {
        return Err(Error::InvalidByteAmount);
    }

    let mut out: [u8; BYTE_COUNT_64] = [0; BYTE_COUNT_64];
    for i in 0..BINARY_SZ_64 {
        let bytes = (binary[i] as u32).to_be_bytes();
        out[4 * i] = bytes[0];
        out[4 * i + 1] = bytes[1];
        out[4 * i + 2] = bytes[2];
        out[4 * i + 3] = bytes[3];
    }

    /* Make sure the encoded version has the same number of leading '1's
    as the decoded version has leading 0s. */

    let mut leading_zero_cnt: usize = 0;
    while leading_zero_cnt < BYTE_COUNT_64 {
        if out[leading_zero_cnt] != 0 {
            break;
        }
        if leading_zero_cnt >= encoded_bytes.len() {
            return Err(Error::InputTooShort);
        }
        if encoded_bytes[leading_zero_cnt] != b'1' {
            return Err(Error::InputTooShort);
        }
        leading_zero_cnt += 1;
    }

    if leading_zero_cnt < encoded_bytes.len() && encoded_bytes[leading_zero_cnt] == b'1' {
        return Err(Error::InputTooLong);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::decode_64;

    #[test]
    fn test_decode_64() {
        let keys = vec![
            "1111111111111111111111111111111111111111111111111111111111111111",
            "5eQS44iKV8B4b4gTt4tPZLPSHtD7F78fFDhbHDknsrAE1vUipnDf3pK6h5eZ8CqWqFgZPoYY6XHKUuvyt7BLWHpb",
            "4EZ6eZt7svb2gYEFFnf14KSpHMD9k6F57qjDwD7dDZhegkrn4e3EzoHNNV83Fjc9cN8BQgG2uRFGwDSivw9yk7Nx",
            "so5VqLRtAF6RxQJ4BSv31SPQfcFhUU1rqCroUJSLCWSEPhZqAEEwiTrH1kdndyztYbTCdmE7qKavgApDqVjmrKQ",
            "RSAtWLUiyEhWUrcBtqmFUgtBHQ2ghJz4poJdXyruFQJpbyfY9AQBfr3dZUP6xdBy7PRqzeXYGUsNai8gcEivZQL",
            "11cgTH4D5e8S3snD444WbbGrkepjTvWMj2jkmCGJtgn3H7qrPb1BnwapxpbGdRtHQh9t9Wbn9t6ZDGHzWpL4df"
        ];

        for key in keys {
            let fd = decode_64(key).unwrap();
            print!("{:?}", fd);
            let normal = bs58::decode(key).into_vec().unwrap();
            assert_eq!(fd, normal.as_slice());
        }
    }

    #[test]
    fn test_invalid_chars_64() {
        let keys = vec![
            "1111111111111111111111111111111111111111111111111111111111111110",
            "111111111111111111111111111111111111111111111111111111111111111!",
            "111111111111111111111111111111111111111111111111111111111111111;",
            "111111111111111111111111111111111111111111111111111111111111111I",
            "111111111111111111111111111111111111111111111111111111111111111O",
            "111111111111111111111111111111111111111111111111111111111111111_",
            "111111111111111111111111111111111111111111111111111111111111111l",
        ];
        for key in keys {
            let fd = decode_64(key);
            assert!(fd.is_err());
            assert!(fd.is_err_and(|x| x == Error::InvalidCharacter));
        }
    }

    #[test]
    fn test_failures_64() {
        let keys = vec![
            "1",
            "111111111111111111111111111111111111111111111111111111111111111",
            "2AFv15MNPuA84RmU66xw2uMzGipcVxNpzAffoacGVvjFue3CBmf633fAWuiP9cwL9C3z3CJiGgRSFjJfeEcA",        /* clearly too short */
            "2AFv15MNPuA84RmU66xw2uMzGipcVxNpzAffoacGVvjFue3CBmf633fAWuiP9cwL9C3z3CJiGgRSFjJfeEcA6QW",     /* largest 63 byte value */
            "2AFv15MNPuA84RmU66xw2uMzGipcVxNpzAffoacGVvjFue3CBmf633fAWuiP9cwL9C3z3CJiGgRSFjJfeEcA6QWabc",  /* clearly too long */
            "11111111111111111111111111111111111111111111111111111111111111111",                           /* Smallest 65 byte value */
            "67rpwLCuS5DGA8KGZXKsVQ7dnPb9goRLoKfgGbLfQg9WoLUgNY77E2jT11fem3coV9nAkguBACzrU1iyZM4B8roS",    /* 2nd-smallest 65 byte value that doesn't start with 0x0 */
            "1114tjGcyzrfXw2deDmDAFFaFyss32WRgkYdDJuprrNEL8kc799TrHSQHfE9fv6ZDBUg2dsMJdfYr71hjE4EfjEN",    /* Start with too many '1's */
        ];
        for key in keys {
            let fd = decode_64(key);
            assert!(fd.is_err());
        }
    }
}
