//! An optimized implementation of [Base58][] encoding/decoding for 32 and 64 byte numbers.
//! This library is based off of the original C implementation from Jump Crypto's [Firedancer]
//! repo which can be found [here]. These algorithms are significantly faster than the commonly used
//! [`bs58`] library for 32 and 64 bytes.
//!
//! # Performance vs. [`bs58`]
//!
//!   Algorithm  |  Speedup        
//! -------------|-----------
//!  `encode_32` | ~9x
//!  `encode_64` | ~13x
//!  `decode_32` | ~3x
//!  `decode_64` | ~5x
//!
//! [Base58]: https://en.wikipedia.org/wiki/Base58
//! [`bs58`]: https://github.com/Nullus157/bs58-rs
//! [Firedancer]: https://github.com/firedancer-io/firedancer
//! [here]: https://github.com/firedancer-io/firedancer/pull/75
//!

use constants::{BYTE_COUNT_32, BYTE_COUNT_64};

pub mod constants;
pub mod decode_32;
pub mod decode_64;
pub mod encode_32;
pub mod encode_64;

/// Encodes the given 32 bytes using an optimized base58 encoding algorithm.
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     [7, 224, 70, 147, 60, 112, 144, 250, 46, 62, 133, 57, 252, 149, 220, 143, 237, 77, 21, 208, 191, 61, 58, 206, 152, 136, 129, 103, 129, 48, 141, 139],
///     fd_bs58::decode_32("XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr")?);
/// # Ok::<(), fd_bs58::Error>(())
/// ```
pub fn encode_32<I: AsRef<[u8]>>(input: I) -> String {
    encode_32::encode_32(input)
}

/// Encodes the given 64 bytes using an optimized base58 encoding algorithm.
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     [7, 224, 70, 147, 60, 112, 144, 250, 46, 62, 133, 57, 252, 149, 220, 143, 237, 77, 21, 208, 191, 61, 58, 206, 152, 136, 129, 103, 129, 48, 141, 139],
///     fd_bs58::decode_32("XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr")?);
/// # Ok::<(), fd_bs58::Error>(())
/// ```
pub fn encode_64<I: AsRef<[u8]>>(input: I) -> String {
    encode_64::encode_64(input)
}

/// Decodes the given base58 string into 32 bytes using an optimized decoding algorithm.
/// This function will return an error if the string is not base58 encoded or the result is not 32 bytes.
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     [7, 224, 70, 147, 60, 112, 144, 250, 46, 62, 133, 57, 252, 149, 220, 143, 237, 77, 21, 208, 191, 61, 58, 206, 152, 136, 129, 103, 129, 48, 141, 139],
///     fd_bs58::decode_32("XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr")?);
/// # Ok::<(), fd_bs58::Error>(())
/// ```
///
/// ## Errors
///
/// ### Invalid Character
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InvalidCharacter,
///     fd_bs58::decode_32("l").unwrap_err());
/// ```
///
/// ### Input Too Long
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InputTooLong,
///     fd_bs58::decode_32("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofLRda4").unwrap_err());
/// ```
///
/// ### Input Too Short
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InputTooShort,
///     fd_bs58::decode_32("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJz").unwrap_err());
/// ```
/// ### Input Byte Amount
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InvalidByteAmount,
///     fd_bs58::decode_32("JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFJ").unwrap_err());
/// ```
///
pub fn decode_32<I: AsRef<[u8]>>(input: I) -> Result<[u8; BYTE_COUNT_32], Error> {
    decode_32::decode_32(input)
}

/// Decodes the given base58 string into 64 bytes using an optimized decoding algorithm.
/// This function will return an error if the string is not base58 encoded or the result is not 64 bytes.
///
/// # Examples
///
/// ## Basic example
///
/// ```rust
/// assert_eq!(
///     [0, 0, 10, 85, 198, 191, 71, 18, 5, 54, 6, 255, 181, 32, 227, 150, 208, 3, 157, 135, 222, 67, 50, 23, 237, 51, 240, 123, 34, 148, 111, 84, 98, 162, 236, 133, 31, 93, 185, 142, 108, 41, 191, 1, 138, 6, 192, 0, 46, 93, 25, 65, 243, 223, 225, 225, 85, 55, 82, 251, 109, 132, 165, 2],
///     fd_bs58::decode_64("11cgTH4D5e8S3snD444WbbGrkepjTvWMj2jkmCGJtgn3H7qrPb1BnwapxpbGdRtHQh9t9Wbn9t6ZDGHzWpL4df")?);
/// # Ok::<(), fd_bs58::Error>(())
/// ```
///
/// ## Errors
///
/// ### Invalid Character
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InvalidCharacter,
///     fd_bs58::decode_64("l").unwrap_err());
/// ```
///
/// ### Input Too Long
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InputTooLong,
///     fd_bs58::decode_64("2AFv15MNPuA84RmU66xw2uMzGipcVxNpzAffoacGVvjFue3CBmf633fAWuiP9cwL9C3z3CJiGgRSFjJfeEcA6QWabc").unwrap_err());
/// ```
///
/// ### Input Too Short
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InputTooShort,
///     fd_bs58::decode_64("2AFv15MNPuA84RmU66xw2uMzGipcVxNpzAffoacGVvjFue3CBmf633fAWuiP9cwL9C3z3CJiGgRSFjJfeEcA").unwrap_err());
/// ```
/// ### Input Byte Amount
///
/// ```rust
/// assert_eq!(
///     fd_bs58::Error::InvalidByteAmount,
///     fd_bs58::decode_64("67rpwLCuS5DGA8KGZXKsVQ7dnPb9goRLoKfgGbLfQg9WoLUgNY77E2jT11fem3coV9nAkguBACzrU1iyZM4B8roS").unwrap_err());
/// ```
///
pub fn decode_64<I: AsRef<[u8]>>(input: I) -> Result<[u8; BYTE_COUNT_64], Error> {
    decode_64::decode_64(input)
}

#[derive(Debug, PartialEq)]
pub enum Error {
    /// The input contains an invalid character
    InvalidCharacter,
    /// The input is too long
    InputTooLong,
    /// The input is too long
    InputTooShort,
    /// The decoded base58 array does not fit the expected byte size
    InvalidByteAmount,
}
